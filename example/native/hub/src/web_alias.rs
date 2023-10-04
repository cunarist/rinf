//! The web has many restrictions due to its sandboxed environment
//! which prevents the use of
//! threads, atomics, time, file IO, network IO,
//! and many other native functionalities.
//! Consequently, certain features are missing from various crates
//! including Rust's `std` due to these limitations.
//!
//! To address this issue, this module offers various imports
//! with the **same names** as the original native ones,
//! providing workarounds for these constraints.
//!
//! You might encounter situations
//! where you cannot use native Rust code directly on the web.
//! Add more custom web aliases here if needed.
//! There are many crates at `crates.io`
//! that mimic native functionalities on the web
//! by interacting with JavaScript,
//! so use them if necessary.
//!
//! If your app is not targeting web, you can simply remove this module.

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

#[cfg(not(target_family = "wasm"))]
pub(crate) fn spawn<T>(future: T)
where
    T: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::task::spawn(future);
}
#[cfg(target_family = "wasm")]
pub(crate) fn spawn<T>(future: T)
where
    T: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

// On the web, `tokio` cannot access the system to check the passed time.
// The JavaScript function `setTimeout()`
// performs this task.

#[cfg(not(target_family = "wasm"))]
pub async fn sleep(duration: std::time::Duration) {
    tokio::time::sleep(duration).await;
}
#[cfg(target_family = "wasm")]
pub async fn sleep(duration: std::time::Duration) {
    let milliseconds = duration.as_millis() as i32;
    use wasm_bindgen::JsCast;
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let global = js_sys::global();
        let scope = global.dyn_into::<web_sys::WorkerGlobalScope>().unwrap();
        let _ = scope.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, milliseconds);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

// To avoid blocking inside a long-running function,
// you have to yield to the async event loop regularly.
// The JavaScript function `setTimeout()`
// can handle this.

#[cfg(not(target_family = "wasm"))]
pub async fn yield_now() {
    tokio::task::yield_now().await;
}
#[cfg(target_family = "wasm")]
pub async fn yield_now() {
    use wasm_bindgen::JsCast;
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let global = js_sys::global();
        let scope = global.dyn_into::<web_sys::WorkerGlobalScope>().unwrap();
        let _ = scope.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}
