//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod common;
mod messages;
mod sample_functions;

use common::*;
use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    spawn(sample_functions::stream_fractal());
    spawn(sample_functions::run_debug_tests());
    spawn(actors::create_actors());

    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}
