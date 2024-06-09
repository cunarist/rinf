//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;
mod sample_functions;

#[cfg(not(target_family = "wasm"))]
use tokio;
#[cfg(target_family = "wasm")]
use tokio_with_wasm as tokio;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());
}
