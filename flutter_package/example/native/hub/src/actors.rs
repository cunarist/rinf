//! The actor model is highly recommended for state management,
//! as it provides modularity and scalability,
//! especially when your app has a lot of shared state.
//! This module demonstrates how to use actors
//! within the async system in Rust.
//! - https://doc.rust-lang.org/beta/book/ch16-02-message-passing.html

use crate::common::*;
use messages::prelude::*;

#[derive(Clone)]
pub struct Addresses {
    pub _one_addr: Address<SampleActorOne>,
    pub _two_addr: Address<SampleActorTwo>,
}

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

mod letters {
    pub struct SampleLetterOne;
    pub struct SampleLetterTwo;
}

use sample_actor_one::SampleActorOne;
use sample_actor_two::SampleActorTwo;

mod sample_actor_one {
    use crate::actors::letters::{SampleLetterOne, SampleLetterTwo};
    use crate::actors::Addresses;
    use crate::common::*;
    use messages::prelude::*;

    pub struct SampleActorOne {
        _addresses: Addresses,
        _dummy_field: i32,
    }

    impl SampleActorOne {
        pub async fn new(addresses: Addresses) -> Result<Self> {
            // Create the actor.
            let actor = SampleActorOne {
                _addresses: addresses,
                _dummy_field: 3,
            };

            Ok(actor)
        }

        async fn _send_to_actor_two(&mut self) {
            let _ = self._addresses._two_addr.send(SampleLetterOne).await;
        }
    }

    impl Actor for SampleActorOne {}

    #[async_trait]
    impl Handler<SampleLetterOne> for SampleActorOne {
        type Result = Result<()>;
        async fn handle(
            &mut self,
            _msg: SampleLetterOne,
            _context: &Context<Self>,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl Handler<SampleLetterTwo> for SampleActorOne {
        type Result = Result<()>;
        async fn handle(
            &mut self,
            _msg: SampleLetterTwo,
            _context: &Context<Self>,
        ) -> Result<()> {
            Ok(())
        }
    }
}

mod sample_actor_two {
    use crate::actors::letters::{SampleLetterOne, SampleLetterTwo};
    use crate::actors::Addresses;
    use crate::common::*;
    use messages::prelude::*;

    pub struct SampleActorTwo {
        _addresses: Addresses,
        _dummy_field: i32,
    }

    impl SampleActorTwo {
        pub async fn new(addresses: Addresses) -> Result<Self> {
            // Create the actor.
            let actor = SampleActorTwo {
                _addresses: addresses,
                _dummy_field: 3,
            };

            Ok(actor)
        }

        async fn _send_to_actor_one(&mut self) {
            let _ = self._addresses._one_addr.send(SampleLetterOne).await;
        }
    }

    impl Actor for SampleActorTwo {}

    #[async_trait]
    impl Handler<SampleLetterOne> for SampleActorTwo {
        type Result = Result<()>;
        async fn handle(
            &mut self,
            _msg: SampleLetterOne,
            _context: &Context<Self>,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl Handler<SampleLetterTwo> for SampleActorTwo {
        type Result = Result<()>;
        async fn handle(
            &mut self,
            _msg: SampleLetterTwo,
            _context: &Context<Self>,
        ) -> Result<()> {
            Ok(())
        }
    }
}
