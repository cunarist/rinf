use std::sync::Mutex;
use std::sync::OnceLock;

#[cfg(not(target_family = "wasm"))]
use super::interface_os::*;
#[cfg(target_family = "wasm")]
use super::interface_web::*;

/// This is a mutable cell type that can be shared across threads.
pub type SharedLock<T> = OnceLock<Mutex<Option<T>>>;

/// This contains a message from Dart.
/// Optionally, a custom binary called `binary` can also be included.
/// This type is generic, and the message
/// can be of any type declared in Protobuf.
pub struct DartSignal<T> {
    pub message: T,
    pub binary: Vec<u8>,
}

/// Send a signal to Dart.
pub fn send_rust_signal(message_id: i32, message_bytes: Vec<u8>, binary: Vec<u8>) {
    send_rust_signal_extern(message_id, message_bytes, binary);
}
