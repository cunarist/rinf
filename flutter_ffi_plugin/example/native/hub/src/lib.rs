use bridge::respond_to_dart;
use tokio_with_wasm::tokio;
use with_request::handle_request;

mod bridge;
mod messages;
mod sample_functions;
mod with_request;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests from Dart.
    let mut request_receiver = bridge::get_request_receiver();
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    tokio::spawn(sample_functions::stream_mandelbrot());
    tokio::spawn(sample_functions::run_debug_tests());
    while let Some(request_unique) = request_receiver.recv().await {
        tokio::spawn(async {
            let response_unique = handle_request(request_unique).await;
            respond_to_dart(response_unique);
        });
    }
}
