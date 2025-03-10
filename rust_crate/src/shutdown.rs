use crate::LockRecovery;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock, Mutex};
use std::task::{Context, Poll, Waker};

// Thread-blocking operations are possible
// only on non-web platforms.
#[cfg(not(target_family = "wasm"))]
use std::sync::Condvar;

type ShutdownEventsLock = LazyLock<ShutdownEvents>;
pub static SHUTDOWN_EVENTS: ShutdownEventsLock =
    LazyLock::new(|| ShutdownEvents {
        dart_stopped: Event::new(),
        #[cfg(not(target_family = "wasm"))]
        rust_stopped: Event::new(),
    });

/// A collection of shutdown events
/// expected to occur one by one on app close.
pub struct ShutdownEvents {
    pub dart_stopped: Event,
    #[cfg(not(target_family = "wasm"))]
    pub rust_stopped: Event,
}

/// Retrieves the shutdown receiver that listens for
/// the Dart runtime's closure.
/// Awaiting this receiver in the async main Rust function
/// is necessary to prevent the async runtime in Rust from
/// finishing immediately.
pub fn dart_shutdown() -> impl Future<Output = ()> {
    SHUTDOWN_EVENTS.dart_stopped.wait_async()
}

/// Synchronization primitive that allows
/// threads or async tasks to wait until a condition is met.
pub struct Event {
    inner: Arc<Mutex<EventInner>>,
    #[cfg(not(target_family = "wasm"))]
    condvar: Arc<Condvar>,
}

impl Event {
    /// Creates a new `Event` with the initial flag state.
    pub fn new() -> Self {
        Event {
            inner: Arc::new(Mutex::new(EventInner::new())),
            #[cfg(not(target_family = "wasm"))]
            condvar: Arc::new(Condvar::new()),
        }
    }

    /// Creates a future that will be resolved
    /// when the flag is set to `true`.
    pub fn wait_async(&self) -> EventFuture {
        let guard = self.inner.lock().recover();
        EventFuture {
            started_session: guard.session,
            inner: self.inner.clone(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Event {
    /// Sets the flag to `true` and notifies all waiting threads.
    /// This will wake up any threads or async tasks.
    pub fn set(&self) {
        let mut guard = self.inner.lock().recover();
        guard.flag = true; // Set the flag
        guard.session += 1; // Increment the session count

        // Wake all threads and async tasks when the event is set
        self.condvar.notify_all();
        for waker in guard.wakers.drain(..) {
            waker.wake();
        }
    }

    /// Clears the flag, setting it to `false`.
    /// This does not affect any waiting threads,
    /// but subsequent calls to `wait` will
    /// block until the flag is set again.
    pub fn clear(&self) {
        let mut guard = self.inner.lock().recover();
        guard.flag = false; // Clear the flag
    }

    /// Blocks the current thread until the flag is set to `true`.
    /// If the flag is already set,
    /// this method will return immediately.
    /// Otherwise, it will block until `set` is called by another thread.
    pub fn wait(&self) {
        let event_blocking =
            EventBlocking::new(self.inner.clone(), self.condvar.clone());
        event_blocking.wait();
    }
}

/// Internal state for the `Event` synchronization primitive.
struct EventInner {
    flag: bool,         // Current flag state
    session: usize,     // Session count to detect changes
    wakers: Vec<Waker>, // List of wakers to be notified
}

impl EventInner {
    fn new() -> Self {
        EventInner {
            flag: false,
            session: 0,
            wakers: Vec::new(),
        }
    }
}

/// Struct to handle waiting with session tracking.
#[cfg(not(target_family = "wasm"))]
struct EventBlocking {
    inner: Arc<Mutex<EventInner>>,
    condvar: Arc<Condvar>,
    started_session: usize,
}

#[cfg(not(target_family = "wasm"))]
impl EventBlocking {
    fn new(inner: Arc<Mutex<EventInner>>, condvar: Arc<Condvar>) -> Self {
        let guard = inner.lock().recover();
        EventBlocking {
            inner: inner.clone(),
            condvar,
            started_session: guard.session,
        }
    }

    pub fn wait(&self) {
        // Lock the inner state and wait on the condition variable
        let mut guard = self.inner.lock().recover();
        loop {
            // Check if the condition is met
            if guard.flag || guard.session != self.started_session {
                break;
            }
            // Wait on the condition variable and reassign the guard
            guard = self.condvar.wait(guard).recover();
        }
    }
}

/// Future that resolves when the `Event` flag is set to `true`.
pub struct EventFuture {
    started_session: usize,
    inner: Arc<Mutex<EventInner>>, // Use the combined inner state
}

impl Future for EventFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.inner.lock().recover();

        // Check if the flag is set or if the session count has changed.
        // If the flag is true or the session count is different
        // because a new event session has started, stop polling.
        if guard.flag || guard.session != self.started_session {
            Poll::Ready(())
        } else {
            // Check if the current waker is already in the list of wakers.
            // If the waker is unique (not already in the list),
            // add it to the list.
            let waker = cx.waker();
            if !guard
                .wakers
                .iter()
                .any(|existing_waker| existing_waker.will_wake(waker))
            {
                guard.wakers.push(waker.clone());
            }

            Poll::Pending
        }
    }
}
