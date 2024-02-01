// This `tokio` will be used by Rinf.
use tokio_with_wasm::tokio;

mod messages;
mod sample_functions;

rinf::write_interface!();

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(sample_functions::tell_numbers());
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());
}
