use crate::error::RinfError;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

pub struct MessageSender<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
}

pub struct MessageReceiver<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
}

struct MessageChannel<T> {
    queue: VecDeque<T>, // Message queue for storing multiple messages
    waker: Option<Waker>,
    sender_dropped: bool,   // Track whether the sender has been dropped
    receiver_dropped: bool, // Track whether the receiver has been dropped
}

impl<T> MessageSender<T> {
    // Send a message and store it in the queue
    pub fn send(&self, msg: T) -> Result<(), RinfError> {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return Err(RinfError::BrokenMessageChannel),
        };

        // Return an error if the receiver has been dropped
        if inner.receiver_dropped {
            return Err(RinfError::ClosedMessageChannel);
        }

        // Enqueue the message
        inner.queue.push_back(msg);
        if let Some(waker) = inner.waker.take() {
            waker.wake(); // Wake the receiver if it's waiting
        }
        Ok(())
    }

    // Check if the receiver is still alive
    pub fn is_closed(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.receiver_dropped
    }
}

impl<T> Drop for MessageSender<T> {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.sender_dropped = true; // Mark that the sender has been dropped
        if let Some(waker) = inner.waker.take() {
            waker.wake(); // Wake the receiver in case it's waiting
        }
    }
}

impl<T> MessageReceiver<T> {
    // Receive the next message from the queue asynchronously
    pub async fn recv(&self) -> Option<T> {
        RecvFuture {
            inner: self.inner.clone(),
        }
        .await
    }
}

impl<T> Drop for MessageReceiver<T> {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.receiver_dropped = true; // Mark that the receiver has been dropped
        if let Some(waker) = inner.waker.take() {
            waker.wake(); // Wake any waiting sender
        }
    }
}

// Future implementation for receiving a message
struct RecvFuture<T> {
    inner: Arc<Mutex<MessageChannel<T>>>,
}

impl<T> Future for RecvFuture<T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return Poll::Ready(None), // Return None on poisoned mutex
        };

        // Check if there are any messages in the queue
        if let Some(msg) = inner.queue.pop_front() {
            return Poll::Ready(Some(msg)); // Return the next message
        }

        // If no messages and the sender is dropped, return None
        if inner.sender_dropped && inner.queue.is_empty() {
            Poll::Ready(None)
        } else {
            inner.waker = Some(cx.waker().clone()); // Set the waker for later notification
            Poll::Pending // No message available, wait
        }
    }
}

// Create the message channel with a message queue
pub fn message_channel<T>() -> (MessageSender<T>, MessageReceiver<T>) {
    let channel = Arc::new(Mutex::new(MessageChannel {
        queue: VecDeque::new(), // Initialize an empty message queue
        waker: None,
        sender_dropped: false,   // Initially, the sender is not dropped
        receiver_dropped: false, // Initially, the receiver is not dropped
    }));
    (
        MessageSender {
            inner: channel.clone(),
        },
        MessageReceiver { inner: channel },
    )
}
