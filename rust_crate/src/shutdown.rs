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
    flag: Arc<Mutex<(bool, usize)>>, // Tuple for flag and session count
    condvar: Arc<Condvar>,
    wakers: Arc<Mutex<Vec<Waker>>>, // Store multiple wakers
}

impl Event {
    /// Creates a new `Event` with the initial flag state.
    pub fn new() -> Self {
        Event {
            flag: Arc::new(Mutex::new((false, 0))),
            condvar: Arc::new(Condvar::new()),
            wakers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Sets the flag to `true` and notifies all waiting threads.
    /// This will wake up any threads waiting on the condition variable.
    pub fn set(&self) {
        let mut state = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.0 = true; // Set the flag
        state.1 += 1; // Increment the count

        // Wake all threads and async tasks when the event is set
        self.condvar.notify_all();
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
        flag.0 = false; // Clear the flag
    }

    /// Blocks the current thread until the flag is set to `true`.
    /// If the flag is already set, this method will return immediately. Otherwise, it
    /// will block until `set` is called by another thread.
    pub fn wait(&self) {
        let mut flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        while !flag.0 {
            flag = match self.condvar.wait(flag) {
                Ok(inner) => inner,
                Err(poisoned) => poisoned.into_inner(),
            };
        }
    }

    /// Creates a future that will be resolved when the flag is set to `true`.
    pub fn wait_async(&self) -> WaitFuture {
        let flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };
        let started_session = flag.1;
        WaitFuture {
            started_session,
            flag: self.flag.clone(),
            wakers: self.wakers.clone(),
        }
    }
}

/// Future that resolves when the `Event` flag is set to `true`.
pub struct WaitFuture {
    started_session: usize,
    flag: Arc<Mutex<(bool, usize)>>,
    wakers: Arc<Mutex<Vec<Waker>>>, // Store multiple wakers
}

impl Future for WaitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Lock the flag to get the current state and session count.
        let flag = match self.flag.lock() {
            Ok(inner) => inner,
            Err(poisoned) => poisoned.into_inner(),
        };

        // Check if the flag is set or if the session count has changed.
        // If the flag is true or the session count is different
        // because a new event session has started, stop polling.
        if flag.0 || self.started_session != flag.1 {
            Poll::Ready(())
        } else {
            // Lock the wakers to manage the list of waiting wakers.
            let mut wakers = match self.wakers.lock() {
                Ok(inner) => inner,
                Err(poisoned) => poisoned.into_inner(),
            };

            // Check if the current waker is already in the list of wakers.
            // If the waker is unique (not already in the list), add it to the list.
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
