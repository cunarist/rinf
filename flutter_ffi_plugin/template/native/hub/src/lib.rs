//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;

use tokio;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // Use `tokio::spawn` to run concurrent tasks.
    use messages::basic::*;
    // Send signals to Dart like below.
    SmallNumber { current_number: 7 }.send_signal_to_dart();
    // Get receivers that listen to Dart signals like below.
    let _ = SmallLetter::get_dart_signal_receiver();
}
