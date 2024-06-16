use crate::common::*;
use allo_isolate::{IntoDart, Isolate, ZeroCopyBuffer};
use os_thread_local::ThreadLocal;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};
use std::thread;
use tokio::runtime::Builder;

static DART_ISOLATE: Mutex<Option<Isolate>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn prepare_isolate_extern(port: i64) {
    let dart_isolate = Isolate::new(port);
    let mut guard = match DART_ISOLATE.lock() {
        Ok(inner) => inner,
        Err(_) => {
            println!("Could not unlock Dart isolate mutex.");
            return;
        }
    };
    guard.replace(dart_isolate);
}

// We use `os_thread_local` so that when the program fails
// and the main thread exits unexpectedly,
// the whole async tokio runtime can shut down as well
// by receiving a signal via the shutdown channel.
// Without this solution, zombie threads inside the tokio runtime
// might outlive the app.
type ShutdownSenderLock = OnceLock<ThreadLocal<RefCell<Option<ShutdownSender>>>>;
static SHUTDOWN_SENDER: ShutdownSenderLock = OnceLock::new();

pub fn start_rust_logic_real<F>(main_future: F) -> Result<()>
where
    F: Future + Send + 'static,
{
    // Enable backtrace output for panics.
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|panic_info| {
            let backtrace = backtrace::Backtrace::new();
            crate::debug_print!("A panic occurred in Rust.\n{panic_info}\n{backtrace:?}");
        }));
    }

    // Prepare the channel that will notify tokio runtime to shutdown
    // after the main Dart thread has gone.
    let (shutdown_sender, shutdown_receiver) = shutdown_channel();
    let shutdown_sender_lock =
        SHUTDOWN_SENDER.get_or_init(move || ThreadLocal::new(|| RefCell::new(None)));
    shutdown_sender_lock.with(|cell| cell.replace(Some(shutdown_sender)));

    // Build the tokio runtime.
    #[cfg(not(feature = "multi-worker"))]
    {
        let tokio_runtime = Builder::new_current_thread().enable_all().build()?;
        thread::spawn(move || {
            tokio_runtime.spawn(async {
                main_future.await;
            });
            tokio_runtime.block_on(shutdown_receiver);
            // Dropping the tokio runtime makes it shut down.
            drop(tokio_runtime);
        });
    }
    #[cfg(feature = "multi-worker")]
    {
        static TOKIO_RUNTIME: Mutex<Option<tokio::runtime::Runtime>> = Mutex::new(None);
        let tokio_runtime = Builder::new_multi_thread().enable_all().build()?;
        tokio_runtime.spawn(async {
            main_future.await;
        });
        tokio_runtime.spawn(async {
            shutdown_receiver.await;
            thread::spawn(|| {
                if let Ok(mut guard) = TOKIO_RUNTIME.lock() {
                    let runtime_option = guard.take();
                    if let Some(runtime) = runtime_option {
                        // Dropping the tokio runtime makes it shut down.
                        drop(runtime);
                    }
                }
            })
        });
        if let Ok(mut guard) = TOKIO_RUNTIME.lock() {
            // If there was already a tokio runtime previously,
            // most likely due to Dart's hot restart,
            // its tasks as well as itself will be terminated,
            // being replaced with the new one.
            let runtime_option = guard.replace(tokio_runtime);
            if let Some(previous_runtime) = runtime_option {
                drop(previous_runtime);
            }
        }
    }

    Ok(())
}

pub fn send_rust_signal_real(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<()> {
    // When `DART_ISOLATE` is not initialized, do nothing.
    // This can happen when running test code in Rust.
    let guard = DART_ISOLATE.lock()?;
    let dart_isolate = guard
        .as_ref()
        .ok_or("Dart isolate for sending Rust signals is not present.")?;

    // If a `Vec<u8>` is empty, we can't just simply send it to Dart
    // because panic can occur from null pointers.
    // Instead, we will reconstruct the empty vector from the Dart side.
    let message_filled = !message_bytes.is_empty();
    let binary_filled = !binary.is_empty();

    dart_isolate.post(
        vec![
            message_id.into_dart(),
            if message_filled {
                ZeroCopyBuffer(message_bytes).into_dart()
            } else {
                ().into_dart()
            },
            if binary_filled {
                ZeroCopyBuffer(binary).into_dart()
            } else {
                ().into_dart()
            },
        ]
        .into_dart(),
    );

    Ok(())
}

struct ShutdownSender {
    is_sent: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Drop for ShutdownSender {
    fn drop(&mut self) {
        self.is_sent.store(true, Ordering::SeqCst);
        if let Ok(mut guard) = self.waker.lock() {
            if let Some(waker) = guard.take() {
                waker.wake();
            }
        }
    }
}

struct ShutdownReceiver {
    is_sent: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for ShutdownReceiver {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_sent.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            if let Ok(mut guard) = self.waker.lock() {
                guard.replace(cx.waker().clone());
            }
            Poll::Pending
        }
    }
}

fn shutdown_channel() -> (ShutdownSender, ShutdownReceiver) {
    let is_sent = Arc::new(AtomicBool::new(false));
    let waker = Arc::new(Mutex::new(None));

    let sender = ShutdownSender {
        is_sent: Arc::clone(&is_sent),
        waker: Arc::clone(&waker),
    };
    let receiver = ShutdownReceiver { is_sent, waker };

    (sender, receiver)
}
