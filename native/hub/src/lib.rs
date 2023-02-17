mod model;
mod sample_functions;
mod user_action_handler;

use ctor::dtor;
use once_cell::sync::OnceCell;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use tokio::task::spawn;
use user_action_handler::handle_user_action;

type UserActionReceiver = OnceCell<Mutex<Receiver<(String, String)>>>;
pub static USER_ACTION_RECEIVER: UserActionReceiver = OnceCell::new();
type ViewmodelUpdateSender = OnceCell<Mutex<Sender<(String, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_SENDER: ViewmodelUpdateSender = OnceCell::new();

#[tokio::main]
pub async fn main() {
    // Thread dedicated for Rust
    let user_action_receiver = USER_ACTION_RECEIVER.get().unwrap().lock().unwrap();
    loop {
        if let Ok(user_action) = user_action_receiver.recv() {
            spawn(handle_user_action(user_action));
        }
    }
}

#[dtor]
fn finalize() {
    // Main thread by Flutter
    // This code is executed before closing unless crashed
    println!("Bye!");
}
