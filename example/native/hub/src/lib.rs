use bridge::respond_to_dart;

mod bridge;
mod sample_functions;
mod with_request;

/// This `hub` crate is the entry point for the Rust logic.
/// `tokio`'s async runtime allows millions of concurrent tasks to be
/// executed at the same time utilizing only the number of threads
/// equivalent to the number of cores on the computer.
/// This is much more efficient and scalable than switching threads.
/// Always use non-blocking async functions in `tokio`'s core threads,
/// such as `async_std::task::sleep` or `tokio::fs::File::open`.
#[cfg_attr(not(target_family = "wasm"), tokio::main)]
#[cfg_attr(target_family = "wasm", allow(unused_variables))]
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests in an async manner.
    let mut request_receiver = bridge::get_request_receiver();
    // These are used for telling the tasks to stop running.
    let (restart_sender, restart_receiver) = tokio::sync::oneshot::channel();
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
        restart_sender.send(()).ok();
    };
    // Begin the tasks and terminate them upon receiving the shutdown signal
    tokio::select! {
        _ = root_future => {}
        _ = restart_receiver => {}
    }
}

/// On the web, async tasks are executed in the JavaScript event loop,
/// unlike when we run the app on native platforms with a `tokio` runtime.
/// That's why we need this `spawn` alias on the web.
#[cfg(not(target_family = "wasm"))]
use tokio::task::spawn;
#[cfg(target_family = "wasm")]
use wasm_bindgen_futures::spawn_local as spawn;
