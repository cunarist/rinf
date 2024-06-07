//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;

use tokio;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    use messages::basic::*;
    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();
    // Get receivers that listen to Dart signals like below.
    let _ = SmallText::get_dart_signal_receiver();
    // Keep the tokio runtime alive until the widget is disposed.
    widget_dispose().await;
    // Perform finalization here, such as saving files.
    // Ensure this process is quick to avoid blocking the screen.
}
