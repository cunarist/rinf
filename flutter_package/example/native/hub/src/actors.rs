//! The actor model is highly recommended for state management,
//! as it provides modularity and scalability,
//! especially when your app has a lot of shared state.
//! This module demonstrates how to use actors
//! within the async system in Rust.
//! To build a solid app, do not communicate by sharing memory;
//! instead, share memory by communicating.

use crate::common::*;
use messages::prelude::*;

/// Holds the addresses of two actors.
/// This struct is used to enable communication between the two actors.
#[derive(Clone)]
pub struct Addresses {
    /// Address of `SampleActorOne`.
    pub _one_addr: Address<SampleActorOne>,
    /// Address of `SampleActorTwo`.
    pub _two_addr: Address<SampleActorTwo>,
}

/// Creates and spawns the actors in the async system.
pub async fn create_actors() -> Result<()> {
    // Create actor contexts.
    let one_context = Context::new();
    let one_addr = one_context.address();
    let two_context = Context::new();
    let two_addr = two_context.address();

    // Collect addresses.
    let addresses = Addresses {
        _one_addr: one_addr,
        _two_addr: two_addr,
    };

    // Spawn actors.
    let actor = SampleActorOne::new(addresses.clone()).await?;
    spawn(one_context.run(actor));
    let actor = SampleActorTwo::new(addresses.clone()).await?;
    spawn(two_context.run(actor));

    Ok(())
}

/// Message types that actors can handle.
mod letters {
    pub struct LetterOne;
    pub struct LetterTwo;
}

use sample_actor_one::SampleActorOne;
use sample_actor_two::SampleActorTwo;

mod sample_actor_one {
    use crate::actors::letters::{LetterOne, LetterTwo};
    use crate::actors::Addresses;
    use crate::common::*;
    use messages::prelude::*;

    /// An actor that manages its state and handles messages.
    /// It can send and receive messages to interact with other actors.
    pub struct SampleActorOne {
        /// Holds the addresses of other actors for communication.
        _addresses: Addresses,
        /// A dummy field to demonstrate actor state management.
        _dummy_field: i32,
    }

    impl SampleActorOne {
        pub async fn new(addresses: Addresses) -> Result<Self> {
            let actor = SampleActorOne {
                _addresses: addresses,
                _dummy_field: 3,
            };

            Ok(actor)
        }

        pub async fn _send_to_actor_two(&mut self) {
            let _ = self._addresses._two_addr.send(LetterOne).await;
        }
    }

    /// Implementing the `Actor` trait for `SampleActorOne`.
    /// This defines `SampleActorOne` as an actor in the async system.
    impl Actor for SampleActorOne {}

    #[async_trait]
    impl Handler<LetterOne> for SampleActorOne {
        type Result = ();
        /// Handles messages received by the actor.
        async fn handle(&mut self, _msg: LetterOne, _context: &Context<Self>) {}
    }

    #[async_trait]
    impl Handler<LetterTwo> for SampleActorOne {
        type Result = ();
        /// Handles messages received by the actor.
        async fn handle(&mut self, _msg: LetterTwo, _context: &Context<Self>) {}
    }
}

mod sample_actor_two {
    use crate::actors::letters::{LetterOne, LetterTwo};
    use crate::actors::Addresses;
    use crate::common::*;
    use messages::prelude::*;

    /// An actor that manages its state and handles messages.
    /// It can send and receive messages to interact with other actors.
    pub struct SampleActorTwo {
        /// Holds the addresses of other actors for communication.
        _addresses: Addresses,
        /// A dummy field to demonstrate actor state management.
        _dummy_field: i32,
    }

    impl SampleActorTwo {
        pub async fn new(addresses: Addresses) -> Result<Self> {
            let actor = SampleActorTwo {
                _addresses: addresses,
                _dummy_field: 3,
            };

            Ok(actor)
        }

        pub async fn _send_to_actor_one(&mut self) {
            let _ = self._addresses._one_addr.send(LetterOne).await;
        }
    }

    /// Implementing the `Actor` trait for `SampleActorTwo`.
    /// This defines `SampleActorOne` as an actor in the async system.
    impl Actor for SampleActorTwo {}

    #[async_trait]
    impl Handler<LetterOne> for SampleActorTwo {
        type Result = ();
        /// Handles messages received by the actor.
        async fn handle(&mut self, _msg: LetterOne, _context: &Context<Self>) {}
    }

    #[async_trait]
    impl Handler<LetterTwo> for SampleActorTwo {
        type Result = ();
        /// Handles messages received by the actor.
        async fn handle(&mut self, _msg: LetterTwo, _context: &Context<Self>) {}
    }
}
