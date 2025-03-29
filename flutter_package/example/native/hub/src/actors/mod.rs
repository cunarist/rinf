//! The actor model is highly recommended for state management,
//! as it provides modularity and scalability.
//! This module demonstrates how to use actors
//! within the async system in Rust.
//! To build a solid app, do not communicate by sharing memory;
//! instead, share memory by communicating.

mod counting;
mod performing;

use crate::signals::CreateActors;
use messages::prelude::Context;
use performing::PerformingActor;
use rinf::DartSignal;
use tokio::spawn;
use tokio_with_wasm::alias as tokio;

pub use counting::*;

// Creates and spawns the actors in the async system.
pub async fn create_actors() {
  // Wait until the start signal arrives.
  let start_receiver = CreateActors::get_dart_signal_receiver();
  start_receiver.recv().await;

  // Create actor contexts.
  let counting_context = Context::new();
  let counting_addr = counting_context.address();
  let performing_context = Context::new();
  let performing_addr = performing_context.address();

  // Spawn the actors.
  let counting_actor = CountingActor::new(counting_addr);
  spawn(counting_context.run(counting_actor));
  let performing_actor = PerformingActor::new(performing_addr);
  spawn(performing_context.run(performing_actor));
}
