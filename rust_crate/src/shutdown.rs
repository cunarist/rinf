use crate::error::RinfError;
use os_thread_local::ThreadLocal;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};

// We use `os_thread_local` so that when the program fails
// and the main thread exits unexpectedly,
// the whole async runtime can shut down as well
// by receiving a signal via the shutdown channel.
// Without this solution,
// zombie threads inside the async runtime might outlive the app.
// This `ThreadLocal` is intended to be used only on the main thread.
type ShutdownSenderLock = OnceLock<ThreadLocal<RefCell<Option<ShutdownSender>>>>;
pub static SHUTDOWN_SENDER: ShutdownSenderLock = OnceLock::new();

type ShutdownReceiverLock = Mutex<Option<ShutdownReceiver>>;
pub static SHUTDOWN_RECEIVER: ShutdownReceiverLock = Mutex::new(None);

pub fn get_shutdown_receiver() -> Result<ShutdownReceiver, RinfError> {
    let mut reciver_lock = SHUTDOWN_RECEIVER
        .lock()
        .map_err(|_| RinfError::LockShutdownReceiver)?;
    reciver_lock.take().ok_or(RinfError::NoShutdownReceiver)
}

pub fn create_shutdown_channel() -> Result<ShutdownReporter, RinfError> {
    let (shutdown_sender, shutdown_receiver, shutdown_reporter) = shutdown_channel();

    let sender_lock = SHUTDOWN_SENDER.get_or_init(move || ThreadLocal::new(|| RefCell::new(None)));
    sender_lock.with(|cell| cell.replace(Some(shutdown_sender)));

    let mut reciver_lock = SHUTDOWN_RECEIVER
        .lock()
        .map_err(|_| RinfError::LockShutdownReceiver)?;
    reciver_lock.replace(shutdown_receiver);

    Ok(shutdown_reporter)
}

type ChannelTuple = (ShutdownSender, ShutdownReceiver, ShutdownReporter);
fn shutdown_channel() -> ChannelTuple {
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

pub struct ShutdownSender {
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
    }
}

pub struct ShutdownReceiver {
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

pub struct ShutdownReporter {
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

// `os_thread_local` is only available on native platforms,
// Let's simply mimic `ThreadLocal` on the web.
#[cfg(target_family = "wasm")]
mod os_thread_local {
    pub struct ThreadLocal<T> {
        inner: T,
    }
    unsafe impl<T> Sync for ThreadLocal<T> {}
    impl<T> ThreadLocal<T> {
        pub fn new<F: Fn() -> T>(inner: F) -> ThreadLocal<T> {
            ThreadLocal { inner: inner() }
        }
        pub fn with<R, F: FnOnce(&T) -> R>(&self, f: F) {
            f(&self.inner);
        }
    }
}
