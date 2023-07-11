use tokio::task::spawn;

mod bridge;
mod sample_functions;
mod with_request;
use crate::bridge::respond_to_dart;

/// Dart operates within a single thread, while Rust has multiple threads.
/// This `main` function is the entry point for the Rust logic.
/// `tokio`'s async runtime is used for concurrency.
/// Always use non-blocking async functions in `tokio`'s core threads,
/// such as `tokio::time::sleep` or `tokio::fs::File::open`.
#[tokio::main]
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests in an async manner.
    let mut request_receiver = bridge::get_request_receiver();
    // These are used for telling the tasks to stop running.
    let (shutdown_signal_sender, shutdown_signal_receiver) = tokio::sync::oneshot::channel();
    let root_join_handle = spawn(async move {
        // Repeat `tokio::task::spawn` anywhere in your code
        // if more concurrent tasks are needed.
        spawn(sample_functions::keep_drawing_mandelbrot());
        while let Some(request_unique) = request_receiver.recv().await {
            spawn(async { respond_to_dart(with_request::handle_request(request_unique).await) });
        }
        // Send the shutdown signal after the request channel is closed,
        // which is typically triggered by Dart's hot restart.
        shutdown_signal_sender.send(()).ok();
    });
    // Begin the tasks and terminate them upon receiving the shutdown signal
    tokio::select! {
        _ = root_join_handle => {}
        _ = shutdown_signal_receiver => {}
    }
}
