//! The actor model is highly recommended for state management,
//! as it provides modularity and scalability,
//! especially when your app has a lot of shared state.
//! This module demonstrates how to use actors
//! within the async system in Rust.
//! To build a solid app, do not communicate by sharing memory;
//! instead, share memory by communicating.

use crate::common::*;
use crate::messages::*;
use messages::prelude::*;
use rinf::debug_print;

/// Holds the addresses of two actors.
/// This struct is used to enable communication between the two actors.
#[derive(Clone)]
pub struct Addresses {
    /// Address of `CountingActor`.
    pub _counting_addr: Address<CountingActor>,
    /// Address of `DummyActor`.
    pub _dummy_addr: Address<DummyActor>,
}

/// Creates and spawns the actors in the async system.
pub async fn create_actors() -> Result<()> {
    // Create actor contexts.
    let counting_context = Context::new();
    let counting_addr = counting_context.address();
    let dummy_context = Context::new();
    let dummy_addr = dummy_context.address();

    // Collect addresses.
    let addresses = Addresses {
        _counting_addr: counting_addr,
        _dummy_addr: dummy_addr,
    };

    // Spawn actors.
    let actor = CountingActor::new(addresses.clone());
    spawn(counting_context.run(actor));
    let actor = DummyActor::new(addresses.clone());
    spawn(dummy_context.run(actor));

    Ok(())
}

/// Letter types that actors will handle.
pub struct LetterButton;
pub struct LetterDummy;

// The actor that holds the counter state and handles messages.
pub struct CountingActor {
    /// Holds the addresses of other actors for communication.
    _addresses: Addresses,
    /// The counter number.
    count: i32,
}

/// Implementing the `Actor` trait for `CountingActor`.
/// This defines `CountingActor` as an actor in the async system.
impl Actor for CountingActor {}

impl CountingActor {
    pub fn new(addresses: Addresses) -> Self {
        spawn(Self::listen_to_button_click(addresses.clone()));
        CountingActor {
            _addresses: addresses,
            count: 0,
        }
    }

    async fn listen_to_button_click(mut addresses: Addresses) {
        let receiver = SampleNumberInput::get_dart_signal_receiver();
        while let Some(dart_signal) = receiver.recv().await {
            let letter = dart_signal.message.letter;
            debug_print!("{letter}");
            let _ = addresses._counting_addr.send(LetterButton).await;
        }
    }
}

#[async_trait]
impl Handler<LetterDummy> for CountingActor {
    type Result = ();
    /// Handles messages received by the actor.
    async fn handle(&mut self, _msg: LetterDummy, _context: &Context<Self>) {}
}

#[async_trait]
impl Handler<LetterButton> for CountingActor {
    type Result = ();
    /// Handles messages received by the actor.
    async fn handle(&mut self, _msg: LetterButton, _context: &Context<Self>) {
        // Perform a simple calculation.
        let current_number = self.count + 7;
        self.count = current_number;

        // The send method is generated from a marked Protobuf message.
        SampleNumberOutput {
            current_number,
            dummy_one: 11,
            dummy_two: None,
            dummy_three: vec![22, 33, 44, 55],
        }
        .send_signal_to_dart();
    }
}

/// An actor that manages its state and handles messages.
/// It can send and receive messages to interact with other actors.
pub struct DummyActor {
    /// Holds the addresses of other actors for communication.
    _addresses: Addresses,
}

impl DummyActor {
    pub fn new(addresses: Addresses) -> Self {
        DummyActor {
            _addresses: addresses,
        }
    }

    async fn _send_to_counting_actor(&mut self) {
        let _ = self._addresses._counting_addr.send(LetterDummy).await;
    }
}

/// Implementing the `Actor` trait for `DummyActor`.
/// This defines `CountingActor` as an actor in the async system.
impl Actor for DummyActor {}
