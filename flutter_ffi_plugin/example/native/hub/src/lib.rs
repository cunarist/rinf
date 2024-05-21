//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;
mod sample_functions;

use tokio;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());
}
