mod channel;
mod error;
mod macros;
mod shutdown;

mod interface;
#[cfg(not(target_family = "wasm"))]
mod interface_os;
#[cfg(target_family = "wasm")]
mod interface_web;

pub use channel::{message_channel, MessageReceiver, MessageSender};
pub use error::RinfError;
pub use interface::{send_rust_signal, start_rust_logic, DartSignal};
pub use shutdown::get_shutdown_receiver;
