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

// On native platforms,`tokio`'s async runtime
// allows millions of concurrent tasks to run the same time
// utilizing only the number of threads
// equivalent to the number of cores on the computer.
// This is much more efficient and scalable than switching threads.
//
// On the web, async tasks are executed in the JavaScript event loop.
// Crate `wasm_bindgen_futures` has the ability
// to convert Rust `Future`s into JavaScript `Promise`s.

pub(crate) fn spawn<T>(future: T)
where
    T: std::future::Future<Output = ()> + Send + 'static,
{
    #[cfg(not(target_family = "wasm"))]
    tokio::task::spawn(future);
    #[cfg(target_family = "wasm")]
    wasm_bindgen_futures::spawn_local(future);
}

// On the web, `tokio` cannot access the system time.
// Crate `wasmtimer` is the exact complement for `tokio::time` on the web.

#[cfg(not(target_family = "wasm"))]
pub(crate) use tokio::time;
#[cfg(target_family = "wasm")]
pub(crate) use wasmtimer::tokio as time;

// On the web, the `println!` macro does not print to the browser console.
// Crate `web_sys` does something exactly like `console.log()` in JavaScript.

#[macro_export]
#[cfg(target_family = "wasm")]
macro_rules! print {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[cfg(not(target_family = "wasm"))]
pub(crate) use println as print;
