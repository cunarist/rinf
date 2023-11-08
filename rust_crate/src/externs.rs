#[cfg(not(target_family = "wasm"))]
pub use backtrace;
#[cfg(not(target_family = "wasm"))]
pub use os_thread_local;

#[cfg(target_family = "wasm")]
pub use js_sys;

pub use lazy_static;
