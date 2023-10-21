use std::panic::{RefUnwindSafe, UnwindSafe};

pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use rust_to_dart::StreamSink;
pub mod ffi;
pub use ffi::*;
pub mod handler;
mod into_into_dart;
mod macros;
pub mod rust_to_dart;
pub mod support;

#[cfg(target_family = "wasm")]
pub mod wasm_bindgen_src;

/// Marker trait for types that are safe to share with Dart and can be dropped
/// safely in case of a panic.
pub trait DartSafe: UnwindSafe + RefUnwindSafe {}

impl<T: UnwindSafe + RefUnwindSafe> DartSafe for T {}
