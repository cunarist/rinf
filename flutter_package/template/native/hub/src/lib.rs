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
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Spawn the concurrent tasks.
    tokio::spawn(communicate());

    // Get the shutdown receiver from Rinf.
    // This receiver will await a signal from Dart shutdown.
    let shutdown_receiver = rinf::get_shutdown_receiver()?;
    shutdown_receiver.await;

    Ok(())
}

async fn communicate() {
    use messages::basic::*;

    // Send signals to Dart like below.
    SmallNumber { number: 7 }.send_signal_to_dart();

    // Get receivers that listen to Dart signals like below.
    let receiver = SmallText::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
        let message: SmallText = dart_signal.message;
        rinf::debug_print!("{message:?}");
    }
}
