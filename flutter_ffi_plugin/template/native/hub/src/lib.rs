//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;

use crate::common::*;
use tokio; // Comment this line to target the web.
// use tokio_with_wasm::alias as tokio; // Uncomment this line to target the web.

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    tokio::spawn(communicate());
}

async fn communicate() -> Result<()> {
    use messages::basic::*;
    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();
    // Get receivers that listen to Dart signals like below.
    let mut receiver = SmallText::get_dart_signal_receiver()?;
    while let Some(dart_signal) = receiver.recv().await {
        let message: SmallText = dart_signal.message;
        rinf::debug_print!("{message:?}");
    }
    Ok(())
}
