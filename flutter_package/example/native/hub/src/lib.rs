//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod sample_functions;
mod signal_test;
mod signals;

use actors::create_actors;

use rinf::{dart_shutdown, write_interface};
use signal_test::send_signals_back_and_forth;
use tokio::spawn;
use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
  // Spawn concurrent tasks.
  // Always use non-blocking async functions like `tokio::fs::File::open`.
  // If you must use blocking code, use `tokio::task::spawn_blocking`
  // or the equivalent provided by your async library.
  spawn(create_actors());
  spawn(send_signals_back_and_forth());

  // Keep the main function running until Dart shutdown.
  dart_shutdown().await;
}
