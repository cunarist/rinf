mod bridge;
mod data_model;
mod sample_functions;
mod with_user_action;

/// There are 2 threads behind this app, one for Dart and one for Rust.
/// This `main` function is the entry point for the Rust logic,
/// which occupies one of those 2 threads.
/// `tokio`'s runtime is used for single-threaded async concurrency.
/// Use your separate threadpool or GPU only when more computing power is needed.
/// Always use non-blocking async functions on the main thread, such as `tokio::time::sleep`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // In debug mode, clean up the data upon Dart's hot restart
    if cfg!(debug_assertions) {
        data_model::clean_model();
    }
    // This is `tokio::sync::mpsc::Reciver` that receives user actions in an async manner.
    let mut user_action_receiver = bridge::get_user_action_receiver();
    // This tells the tasks to stop running.
    let (shutdown_signal_sender, shutdown_signal_receiver) = tokio::sync::oneshot::channel();
    // By using `tokio::task::LocalSet`, all tasks are ensured
    // that they are executed on the main thread.
    let local_set = tokio::task::LocalSet::new();
    // Always use `tokio::task::spawn_local` over `tokio::task::spawn`
    // to clarify that we want the task to be executed on the current thread.
    local_set.spawn_local(async move {
        while let Some(user_action) = user_action_receiver.recv().await {
            tokio::task::spawn_local(with_user_action::handle_user_action(user_action));
        }
        shutdown_signal_sender.send(true).ok();
    });
    // You can repeat `tokio::task::LocalSet::spawn_local`
    // if there should be more concurrent tasks.
    local_set.spawn_local(sample_functions::keep_drawing_mandelbrot());
    // Begin the tasks and terminate them upon receiving the shutdown signal,
    // which is typically triggered by the disposed user action channel
    // resulting from Dart's hot restart.
    tokio::select! {
        _ = local_set => {}
        _ = shutdown_signal_receiver => {}
    }
}
