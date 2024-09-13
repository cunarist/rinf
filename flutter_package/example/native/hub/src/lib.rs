//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;
mod sample_functions;

use common::*;
use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Spawn the concurrent tasks.
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());

    // Get the shutdown receiver from Rinf.
    // This receiver will await a signal from Dart shutdown.
    let shutdown_receiver = rinf::get_shutdown_receiver()?;
    shutdown_receiver.await;

    Ok(())
}
