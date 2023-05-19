use api::Serialized;
use ref_thread_local::RefThreadLocal;
use tokio::sync::mpsc::Receiver;

pub mod api;
mod bridge_generated;

/// Updating the viewmodel will
/// automatically send a stream signal to Flutter widgets
/// which would trigger the rebuild.
/// `item_address` would be something like `someItemAddress.someName`.
pub fn update_viewmodel(item_address: &str, serialized: Serialized) {
    let refcell = api::VIEWMODEL_UPDATE_SENDER.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    if let Some(sender) = option {
        let tuple = (String::from(item_address), serialized);
        sender.try_send(tuple).ok();
    }
    let refcell = api::VIEWMODEL_UPDATE_STREAM.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    if let Some(stream) = option {
        stream.add(item_address.to_string());
    }
}

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_user_action_receiver() -> Receiver<(String, Serialized)> {
    let refcell = api::USER_ACTION_RECEIVER.borrow();
    let option = refcell.replace(None);
    option.expect("User action receiver does not exist!")
}
