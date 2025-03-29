use crate::signals::{SampleNumberInput, SampleNumberOutput};
use async_trait::async_trait;
use messages::prelude::{Actor, Address, Context, Notifiable};
use rinf::{DartSignal, RustSignal, debug_print};
use tokio::task::JoinSet;
use tokio_with_wasm::alias as tokio;

/// The actor that holds the counter state.
pub struct CountingActor {
  count: i32,
  _owned_tasks: JoinSet<()>,
}

impl Actor for CountingActor {}

impl CountingActor {
  pub fn new(self_addr: Address<Self>) -> Self {
    let mut owned_tasks = JoinSet::new();
    owned_tasks.spawn(Self::listen_to_button_click(self_addr));
    CountingActor {
      count: 0,
      _owned_tasks: owned_tasks,
    }
  }

  async fn listen_to_button_click(mut self_addr: Address<Self>) {
    let receiver = SampleNumberInput::get_dart_signal_receiver();
    while let Some(dart_signal) = receiver.recv().await {
      let message = dart_signal.message;
      let _ = self_addr.notify(message).await;
    }
  }
}

#[async_trait]
impl Notifiable<SampleNumberInput> for CountingActor {
  async fn notify(&mut self, msg: SampleNumberInput, _: &Context<Self>) {
    // Increase the counter number.
    debug_print!("{}", msg.letter);
    self.count += 7;

    // The send method is generated on structs that derive `RustSignal`.
    SampleNumberOutput {
      current_number: self.count,
      dummy_one: 11,
      dummy_two: None,
      dummy_three: vec![22, 33, 44, 55],
    }
    .send_signal_to_dart();
  }
}
