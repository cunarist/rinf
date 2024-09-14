use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, LazyLock, Mutex};
use std::task::{Context, Poll, Waker};

type ShutdownEventsLock = LazyLock<ShutdownEvents>;
pub static SHUTDOWN_EVENTS: ShutdownEventsLock = LazyLock::new(|| ShutdownEvents {
    dart_stopped: Event::new(),
    rust_stopped: Event::new(),
});

/// A collection of shutdown events
/// expected to occur one by one on app close.
pub struct ShutdownEvents {
    pub dart_stopped: Event,
    pub rust_stopped: Event,
}

/// Retrieves the shutdown receiver that listens for
/// the Dart runtime's closure.
/// Awaiting this receiver in the async main Rust function
/// is necessary to prevent the async runtime in Rust from
/// finishing immediately.
pub async fn dart_shutdown() {
    SHUTDOWN_EVENTS.dart_stopped.wait_async().await;
}

/// Synchronization primitive that allows
/// threads or async tasks to wait until a condition is met.
pub struct Event {
    flag: Arc<Mutex<bool>>,
    condvar: Arc<Condvar>,
    wakers: Arc<Mutex<Vec<Waker>>>,
}

impl Event {
    /// Creates a new `Event` with the initial state of the flag set to `false`.
    pub fn new() -> Self {
        Event {
            flag: Arc::new(Mutex::new(false)),
            condvar: Arc::new(Condvar::new()),
            wakers: Arc::new(Mutex::new(Vec::new())), // Initialize as an empty Vec
        }
    }

    /// Sets the flag to `true` and notifies all waiting threads.
    /// This will wake up any threads waiting on the condition variable.
    pub fn set(&self) {
        let mut flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        *flag = true;
        self.condvar.notify_all();

        // Wake all wakers when the event is set
        let mut wakers = match self.wakers.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        for waker in wakers.drain(..) {
            waker.wake();
        }
    }

    /// Clears the flag, setting it to `false`.
    /// This does not affect any waiting threads, but subsequent calls to `wait` will
    /// block until the flag is set again.
    pub fn clear(&self) {
        let mut flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        *flag = false;
    }

    /// Blocks the current thread until the flag is set to `true`.
    /// If the flag is already set, this method will return immediately. Otherwise, it
    /// will block until `set` is called by another thread.
    pub fn wait(&self) {
        let mut flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        while !*flag {
            flag = match self.condvar.wait(flag) {
                Ok(inner) => inner,
                Err(poisoned) => poisoned.into_inner(),
            };
        }
    }

    /// Creates a future that will be resolved when the flag is set to `true`.
    pub fn wait_async(&self) -> WaitFuture {
        WaitFuture {
            flag: self.flag.clone(),
            wakers: self.wakers.clone(),
        }
    }
}

/// Future that resolves when the `Event` flag is set to `true`.
pub struct WaitFuture {
    flag: Arc<Mutex<bool>>,
    wakers: Arc<Mutex<Vec<Waker>>>,
}

impl Future for WaitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };

        if *flag {
            Poll::Ready(())
        } else {
            let mut wakers = match self.wakers.lock() {
                Ok(inner) => inner,
                Err(poisoned) => poisoned.into_inner(),
            };

            // Remember the current waker if not in the list
            let waker = cx.waker();
            let is_unique = !wakers
                .iter()
                .any(|existing_waker| existing_waker.will_wake(waker));
            if is_unique {
                wakers.push(waker.clone());
            }

            Poll::Pending
        }
    }
}
