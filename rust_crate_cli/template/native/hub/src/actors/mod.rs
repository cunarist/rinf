//! This module contains actors.
//! To build a solid app, avoid communicating by sharing memory.
//! Focus on message passing instead.

mod first;
mod second;

use first::FirstActor;
use messages::prelude::Context;
use second::SecondActor;
use tokio::spawn;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

/// Creates and spawns the actors in the async system.
pub async fn create_actors() {
  // Though simple async tasks work, using the actor model
  // is highly recommended for state management
  // to achieve modularity and scalability in your app.
  // Actors keep ownership of their state and run in their own loops,
  // handling messages from other actors or external sources,
  // such as websockets or timers.

  // Create actor contexts.
  let first_context = Context::new();
  let first_addr = first_context.address();
  let second_context = Context::new();

  // Spawn the actors.
  let first_actor = FirstActor::new(first_addr.clone());
  spawn(first_context.run(first_actor));
  let second_actor = SecondActor::new(first_addr);
  spawn(second_context.run(second_actor));
}
