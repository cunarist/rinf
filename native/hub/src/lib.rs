mod bridge;
mod model;
mod sample_functions;
mod user_action_handler;

use ctor::dtor;
use tokio::task::spawn;
use user_action_handler::handle_user_action;

pub use bridge::api;

#[tokio::main]
pub async fn main() {
    // Thread dedicated for Rust
    let user_action_receiver = api::USER_ACTION_RECEIVER.get().unwrap().lock().unwrap();
    loop {
        if let Ok(user_action) = user_action_receiver.recv() {
            spawn(handle_user_action(user_action.0, user_action.1));
        }
    }
}

#[dtor]
fn finalize() {
    // Main thread by Flutter
    // This code is executed before closing unless crashed
    println!("Bye!");
}
