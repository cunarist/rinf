//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;
mod sample_functions;

use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Spawn the concurrent tasks.
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());

    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}
