use tokio_with_wasm::tokio;

mod bridge;
mod messages;
mod sample_functions;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // Register the generated function that will handle Dart signals.
    bridge::register_signal_handler(Box::new(messages::handle::handle_signal));
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(sample_functions::stream_fractal());
    tokio::spawn(sample_functions::run_debug_tests());
    let mut receiver = messages::counter_number::number_input_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        tokio::spawn(sample_functions::handle_number(dart_signal.message));
    }
}
