use crate::AppError;

#[cfg(not(target_family = "wasm"))]
use crate::interface_os::{send_rust_signal_real, start_rust_logic_real};
#[cfg(target_family = "wasm")]
use crate::interface_web::{send_rust_signal_real, start_rust_logic_real};

/// This contains a message from Dart.
/// Optionally, a custom binary called `binary` can also be included.
#[cfg_attr(feature = "bevy", derive(bevy_ecs::event::Event))]
pub struct DartSignalPack<T> {
  /// The message instance.
  pub message: T,
  /// Binary data included in the signal.
  /// This field is useful for sending custom bytes
  /// without the overhead of serialization/deserialization.
  pub binary: Vec<u8>,
}

/// Runs the async main function in Rust.
/// On native platforms, futures usually implement the `Send` trait
/// to be safely sent between threads.
/// Even in a single-threaded (current-thread) runtime,
/// the `Runtime` object itself might be moved between threads,
/// along with all the tasks it manages.
#[doc(hidden)]
#[cfg(not(target_family = "wasm"))]
pub fn start_rust_logic<F, T>(main_fn: F) -> Result<(), AppError>
where
  F: Fn() -> T + Send + 'static,
{
  start_rust_logic_real(main_fn)
}

/// Runs the async main function in Rust.
/// On the web, futures usually don't implement the `Send` trait
/// because JavaScript environment is fundamentally single-threaded.
#[cfg(target_family = "wasm")]
pub fn start_rust_logic<F, T>(main_fn: F) -> Result<(), AppError>
where
  F: Fn() -> T + 'static,
{
  start_rust_logic_real(main_fn)
}

/// Send a signal to Dart.
#[doc(hidden)]
pub fn send_rust_signal(
  endpoint: &str,
  message_bytes: Vec<u8>,
  binary: Vec<u8>,
) -> Result<(), AppError> {
  send_rust_signal_real(endpoint, message_bytes, binary)
}
