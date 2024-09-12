use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Clone)]
pub struct MessageSender<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
}

pub struct MessageReceiver<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
    id: usize, // Each receiver has a unique ID
}

struct MessageChannel<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    active_receiver_id: usize, // Track the active receiver by ID
}

impl<T> MessageSender<T> {
    pub fn send(&self, msg: T) {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return, // Do not consider poisoned mutex
        };

        // Enqueue the message
        inner.queue.push_back(msg);
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl<T> MessageReceiver<T> {
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

struct RecvFuture<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
    receiver_id: usize, // Track which receiver is polling
}

impl<T> Future for RecvFuture<T> {
    type Output = Option<T>;

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

// Create the message channel with a message queue
pub fn message_channel<T>() -> (MessageSender<T>, MessageReceiver<T>) {
    let channel = Arc::new(Mutex::new(MessageChannel {
        queue: VecDeque::new(),
        waker: None,
        active_receiver_id: 0, // Start with receiver ID 0
    }));

    let receiver = MessageReceiver {
        inner: channel.clone(),
        id: 0,
    };

    (
        MessageSender {
            inner: channel.clone(),
        },
        receiver,
    )
}
