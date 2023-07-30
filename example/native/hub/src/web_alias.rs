//! The web has many restrictions due to its sandboxed environment,
//! which prevents the use of native threads, atomics, and time functionalities.
//! Consequently, certain features are missing from Rust's `std`
//! and other crates due to these limitations.
//!
//! To address this issue, this module offers various imports
//! with the **same names** as the original native ones,
//! providing workarounds for these constraints
//!
//! You might encounter situations
//! where you cannot use native Rust code directly on the web.
//! Add more custom aliases if needed.

#![allow(dead_code, unused_imports, unused_macros)]

// On the web, async tasks are executed in the JavaScript event loop,
// unlike when we run the app on native platforms with the `tokio` runtime.

#[cfg(not(target_family = "wasm"))]
pub(crate) use tokio::task::spawn;
#[cfg(target_family = "wasm")]
pub(crate) use wasm_bindgen_futures::spawn_local as spawn;

// On the web, `tokio` cannot access the system time.

#[cfg(not(target_family = "wasm"))]
pub(crate) use tokio::time;
#[cfg(target_family = "wasm")]
pub(crate) use wasmtimer::tokio as time;

// On the web, `println!` macro does not print to the browser console.

#[macro_export]
#[cfg(target_family = "wasm")]
macro_rules! print {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[cfg(not(target_family = "wasm"))]
pub(crate) use println as print;
