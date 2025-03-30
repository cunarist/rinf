mod counting;
mod performing;

use crate::signals::CreateActors;
use messages::prelude::Context;
use performing::PerformingActor;
use rinf::DartSignal;
use tokio::spawn;
use tokio_with_wasm::alias as tokio;

pub use counting::*;

/// Spawns the actors.
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
