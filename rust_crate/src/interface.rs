use std::future::Future;

#[cfg(not(target_family = "wasm"))]
use super::interface_os::*;
#[cfg(target_family = "wasm")]
use super::interface_web::*;

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

pub fn start_rust_logic<F>(main_future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    start_rust_logic_extern(main_future);
}
