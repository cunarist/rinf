use crate::actors::{FirstActor, SmallBool};
use messages::prelude::{Actor, Address};
use rinf::debug_print;
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

pub struct SecondActor {}

impl Actor for SecondActor {}

impl SecondActor {
  pub fn new(first_addr: Address<FirstActor>) -> Self {
    spawn(Self::ask_first_actor(first_addr));
    SecondActor {}
  }
}

impl SecondActor {
  async fn ask_first_actor(mut other_addr: Address<FirstActor>) {
    sleep(Duration::from_secs(10)).await;
    let small_bool = SmallBool(true);
    if let Ok(response) = other_addr.send(small_bool).await {
      debug_print!("{}", response);
    }
  }
}
