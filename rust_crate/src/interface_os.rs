use crate::error::RinfError;
use allo_isolate::{IntoDart, Isolate, ZeroCopyBuffer};
use os_thread_local::ThreadLocal;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
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
            let error = RinfError::LockDartIsolate;
            println!("{error}");
            return;
        }
    };
    guard.replace(dart_isolate);
}

// We use `os_thread_local` so that when the program fails
// and the main thread exits unexpectedly,
// the whole async tokio runtime can shut down as well
// by receiving a signal via the shutdown channel.
// Without this solution,
// zombie threads inside the tokio runtime might outlive the app.
// This `ThreadLocal` is intended to be used only on the main thread.
type ShutdownSenderLock = OnceLock<ThreadLocal<RefCell<Option<ShutdownSender>>>>;
static SHUTDOWN_SENDER: ShutdownSenderLock = OnceLock::new();

pub fn start_rust_logic_real<F, T>(main_future: F) -> Result<(), RinfError>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    // Enable backtrace output for panics.
    #[cfg(debug_assertions)]
    {
        #[cfg(not(feature = "backtrace"))]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
            }));
        }
        #[cfg(feature = "backtrace")]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                let backtrace = backtrace::Backtrace::new();
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}\n{backtrace:?}");
            }));
        }
    }

    // Prepare the channel that will notify tokio runtime to shutdown
    // after the main Dart thread has gone.
    let (shutdown_sender, shutdown_receiver, shutdown_reporter) = shutdown_channel();
    let sender_lock = SHUTDOWN_SENDER.get_or_init(move || ThreadLocal::new(|| RefCell::new(None)));
    sender_lock.with(|cell| cell.replace(Some(shutdown_sender)));

    // Build the tokio runtime.
    #[cfg(not(feature = "rt-multi-thread"))]
    {
        let tokio_runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| RinfError::BuildRuntime)?;
        thread::spawn(move || {
            tokio_runtime.spawn(main_future);
            tokio_runtime.block_on(shutdown_receiver);
            // Dropping the tokio runtime makes it shut down.
            drop(tokio_runtime);
            println!("DROPPED RUNTIME (TEMP)");
            // After dropping the runtime, tell the main thread to stop waiting.
            drop(shutdown_reporter);
        });
    }
    #[cfg(feature = "rt-multi-thread")]
    {
        static TOKIO_RUNTIME: Mutex<Option<tokio::runtime::Runtime>> = Mutex::new(None);
        let tokio_runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|_| RinfError::BuildRuntime)?;
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
                    // After dropping the runtime, tell the main thread to stop waiting.
                    drop(shutdown_reporter);
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

#[no_mangle]
pub extern "C" fn stop_rust_logic_extern() {
    let sender_lock = SHUTDOWN_SENDER.get_or_init(move || ThreadLocal::new(|| RefCell::new(None)));
    let sender_option = sender_lock.with(|cell| cell.take());
    if let Some(shutdown_sender) = sender_option {
        // Dropping the sender tells the tokio runtime to stop running.
        // Also, it blocks the main thread until
        // it gets the report that tokio shutdown is dropped.
        drop(shutdown_sender);
    }
}

pub fn send_rust_signal_real(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<(), RinfError> {
    // When `DART_ISOLATE` is not initialized, just return the error.
    // This can happen when running test code in Rust.
    let guard = DART_ISOLATE
        .lock()
        .map_err(|_| RinfError::LockDartIsolate)?;
    let dart_isolate = guard.as_ref().ok_or(RinfError::NoDartIsolate)?;

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
    should_shutdown: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
    did_shutdown: Arc<Mutex<bool>>,
    is_done: Arc<Condvar>,
}

impl Drop for ShutdownSender {
    fn drop(&mut self) {
        self.should_shutdown.store(true, Ordering::SeqCst);
        if let Ok(mut guard) = self.waker.lock() {
            if let Some(waker) = guard.take() {
                waker.wake();
            }
        }
        while let Ok(guard) = self.did_shutdown.lock() {
            if *guard {
                break;
            } else {
                let _unused = self.is_done.wait(guard);
            }
        }
        println!("END (TEMP)");
    }
}

struct ShutdownReceiver {
    should_shutdown: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for ShutdownReceiver {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.should_shutdown.load(Ordering::SeqCst) {
            if let Ok(mut guard) = self.waker.lock() {
                guard.replace(cx.waker().clone());
            }
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

type ChannelTuple = (ShutdownSender, ShutdownReceiver, ShutdownReporter);
fn shutdown_channel() -> ChannelTuple {
    // This code assumes that
    // this function is being called from the main thread.

    let should_shutdown = Arc::new(AtomicBool::new(false));
    let waker = Arc::new(Mutex::new(None));
    let did_shutdown = Arc::new(Mutex::new(false));
    let is_done = Arc::new(Condvar::new());

    let sender = ShutdownSender {
        should_shutdown: should_shutdown.clone(),
        waker: waker.clone(),
        did_shutdown: did_shutdown.clone(),
        is_done: is_done.clone(),
    };
    let receiver = ShutdownReceiver {
        should_shutdown,
        waker,
    };
    let reporter = ShutdownReporter {
        did_shutdown,
        is_done,
    };

    (sender, receiver, reporter)
}

struct ShutdownReporter {
    did_shutdown: Arc<Mutex<bool>>,
    is_done: Arc<Condvar>,
}

impl Drop for ShutdownReporter {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.did_shutdown.lock() {
            *guard = true;
        }
        self.is_done.notify_all();
    }
}
