//! This module supports communication with Dart.
//! More specifically, sending responses and
//! stream signals to Dart are supported.
//! DO NOT EDIT.

#![allow(dead_code)]

pub mod interface;
pub use interface::*;

#[cfg(not(target_family = "wasm"))]
mod interface_os;
#[cfg(target_family = "wasm")]
mod interface_web;

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!()`,
/// as it seamlessly adapts to different environments.
/// Note that this macro does nothing in release mode.
#[macro_export]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {
        let rust_report = format!( $( $t )* );
        #[cfg(debug_assertions)]
        $crate::bridge::send_rust_report(rust_report.into());
        #[cfg(not(debug_assertions))]
        let _ = rust_report;
    }
}
