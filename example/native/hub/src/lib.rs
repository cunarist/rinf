use bridge::respond_to_dart;
use web_alias::*;

mod bridge;
mod sample_functions;
mod web_alias;
mod with_request;

/// This `hub` crate is the entry point for the Rust logic.
/// `tokio`'s async runtime allows millions of concurrent tasks to be
/// executed at the same time utilizing only the number of threads
/// equivalent to the number of cores on the computer.
/// This is much more efficient and scalable than switching threads.
/// Always use non-blocking async functions in `tokio`'s core threads,
/// such as `tokio::fs::File::open`.
#[cfg_attr(not(target_family = "wasm"), tokio::main)]
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests in an async manner.
    let mut request_receiver = bridge::get_request_receiver();
    // These are used for telling the tasks to stop running.
    let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();
    let root_future = async move {
        // Repeat `crate::spawn` anywhere in your code
        // if more concurrent tasks are needed.
        crate::spawn(sample_functions::keep_drawing_mandelbrot());
        while let Some(request_unique) = request_receiver.recv().await {
            crate::spawn(async {
                respond_to_dart(with_request::handle_request(request_unique).await)
            });
        }
        // Send the shutdown signal after the request channel is closed,
        // which is typically triggered by Dart's hot restart.
        shutdown_sender.send(()).ok();
    };
    // Begin the tasks and terminate them upon receiving the shutdown signal
    tokio::select! {
        _ = root_future => {}
        _ = shutdown_receiver => {}
    }
}
