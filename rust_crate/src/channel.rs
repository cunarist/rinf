use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// The `MessageSender` is used to send messages into a shared message queue.
/// It is clonable, and multiple senders can be created to send messages into
/// the same queue. Each message is sent to a receiver, but only the currently
/// active receiver can receive messages.
#[derive(Clone)]
pub struct MessageSender<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
}

/// The `MessageReceiver` is used to asynchronously receive messages from the
/// shared message queue. Only one receiver can be active at a time; new
/// receivers are created by cloning the original. When a receiver is cloned,
/// it becomes the active receiver, and the previous receiver will no longer
/// receive messages.
pub struct MessageReceiver<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
    id: usize, // Each receiver has a unique ID
}

/// A channel holding a message queue and managing the current active receiver.
/// Only the active receiver can receive messages.
struct MessageChannel<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    active_receiver_id: usize, // Track the active receiver by ID
}

impl<T> MessageSender<T> {
    /// Sends a message to the shared queue. If a receiver is waiting for a
    /// message, it will be woken up. This method does not fail if the mutex
    /// is poisoned but simply ignores the failure.
    pub fn send(&self, msg: T) {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return, // Do not consider poisoned mutex
        };

        // Enqueue the message
        inner.queue.push_back(msg);
        // Wake up the previous receiver making it receive `None`, if any
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl<T> MessageReceiver<T> {
    /// Asynchronously receives the next message from the queue. Only the active
    /// receiver is allowed to receive messages. If there are no messages in the
    /// queue, the receiver will wait until a new message is sent. If this receiver
    /// is not active, it will return `None`.
    pub async fn recv(&self) -> Option<T> {
        RecvFuture {
            inner: self.inner.clone(),
            receiver_id: self.id, // Pass the receiver's ID to the future
        }
        .await
    }
}

// Automatically make the cloned receiver the active one
impl<T> Clone for MessageReceiver<T> {
    /// Clones the receiver and makes the new receiver the active one. The
    /// original receiver will no longer receive messages after this clone.
    /// This ensures only the most recent receiver can access the message queue.
    fn clone(&self) -> Self {
        let mut inner = self.inner.lock().unwrap();
        let new_receiver = MessageReceiver {
            inner: self.inner.clone(),
            id: inner.active_receiver_id + 1, // Increment ID for new receiver
        };
        inner.active_receiver_id = new_receiver.id;
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
        new_receiver
    }
}

/// A future that represents the attempt of a `MessageReceiver` to receive a
/// message. This future is only completed when the active receiver receives
/// a message from the queue.
struct RecvFuture<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
    receiver_id: usize, // Track which receiver is polling
}

impl<T> Future for RecvFuture<T> {
    type Output = Option<T>;

    /// Polls the future to check if the active receiver has a message in the
    /// queue. If no message is available, the task will be put to sleep until
    /// a message is sent. If this receiver is not the active receiver, it will
    /// return `None`.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return Poll::Ready(None), // Return None on poisoned mutex
        };

        // Only allow the current active receiver to receive messages
        if inner.active_receiver_id == self.receiver_id {
            if let Some(msg) = inner.queue.pop_front() {
                Poll::Ready(Some(msg))
            } else {
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        } else {
            Poll::Ready(None) // Return None if this receiver is not the current active one
        }
    }
}

/// Creates a message channel with a sender and a receiver. The sender can be
/// used to send messages, and the receiver can be used to receive them
/// asynchronously. Only one receiver is active at a time, and new receivers
/// are created by cloning the original receiver.
pub fn message_channel<T>() -> (MessageSender<T>, MessageReceiver<T>) {
    let channel = Arc::new(Mutex::new(MessageChannel {
        queue: VecDeque::new(),
        waker: None,
        active_receiver_id: 0, // Start with receiver ID 0
    }));

    let sender = MessageSender {
        inner: channel.clone(),
    };
    let receiver = MessageReceiver {
        inner: channel,
        id: 0,
    };
    (sender, receiver)
}
