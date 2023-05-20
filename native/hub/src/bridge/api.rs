use flutter_rust_bridge::RustOpaque;
use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use ref_thread_local::ref_thread_local;
use ref_thread_local::RefThreadLocal;
use std::cell::RefCell;
use std::collections::HashMap;
pub use std::sync::Mutex;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct Serialized {
    pub data: Vec<u8>,
    pub formula: String,
}

#[derive(Debug)]
pub struct EndpointsOnRustThread {
    pub user_action_receiver: Receiver<(String, Serialized)>,
    pub viewmodel_update_sender: Sender<(String, Serialized)>,
}

type ViewmodelUpdateStream = RefCell<Option<StreamSink<String>>>;
type UserActionSender = RefCell<Option<Sender<(String, Serialized)>>>;
type UserActionReceiver = RefCell<Option<Receiver<(String, Serialized)>>>;
type ViewmodelUpdateSender = RefCell<Option<Sender<(String, Serialized)>>>;
type ViewmodelUpdateReceiver = RefCell<Option<Receiver<(String, Serialized)>>>;

ref_thread_local! {
    // With this macro, each thread gets its own static variables instead of sharing them.
    pub static managed STARTED_FLAG: bool = false; // For Dart thread
    pub static managed VIEWMODEL: HashMap<String, Serialized> = HashMap::new(); // For Dart thread
    pub static managed VIEWMODEL_UPDATE_STREAM: ViewmodelUpdateStream = RefCell::new(None); // For Rust thread
    pub static managed USER_ACTION_SENDER: UserActionSender = RefCell::new(None); // For Dart thread
    pub static managed USER_ACTION_RECEIVER: UserActionReceiver = RefCell::new(None); // For Rust thread
    pub static managed VIEWMODEL_UPDATE_SENDER: ViewmodelUpdateSender= RefCell::new(None); // For Rust thread
    pub static managed VIEWMODEL_UPDATE_RECEIVER: ViewmodelUpdateReceiver =RefCell::new( None); // For Dart thread
}

pub fn prepare_viewmodel_update_stream(viewmodel_update_stream: StreamSink<String>) {
    // Thread 1 running Rust
    let refcell = VIEWMODEL_UPDATE_STREAM.borrow_mut();
    refcell.replace(Some(viewmodel_update_stream));
}

pub fn prepare_channels() -> SyncReturn<RustOpaque<Mutex<EndpointsOnRustThread>>> {
    // Thread 0 running Dart
    let (user_action_sender, user_action_receiver) = channel(1024);
    let refcell = USER_ACTION_SENDER.borrow();
    refcell.replace(Some(user_action_sender));
    let (viewmodel_update_sender, viewmodel_update_receiver) = channel(1024);
    let refcell = VIEWMODEL_UPDATE_RECEIVER.borrow();
    refcell.replace(Some(viewmodel_update_receiver));
    let endpoints_on_rust_thread = EndpointsOnRustThread {
        user_action_receiver,
        viewmodel_update_sender,
    };
    SyncReturn(RustOpaque::new(Mutex::new(endpoints_on_rust_thread)))
}

pub fn lay_endpoints_on_rust_thread(rust_opaque: RustOpaque<Mutex<EndpointsOnRustThread>>) {
    // Thread 1 running Rust
    let result = rust_opaque.try_unwrap();
    let mutex = result.expect("Failed to unwrap received `RustOpaque` object!");
    let result = mutex.into_inner();
    let endpoints_at_dart_thread =
        result.expect("Data inside received `RustOpaque` object is not valid!");
    let inner = endpoints_at_dart_thread.user_action_receiver;
    let refcell = USER_ACTION_RECEIVER.borrow();
    refcell.replace(Some(inner));
    let inner = endpoints_at_dart_thread.viewmodel_update_sender;
    let refcell = VIEWMODEL_UPDATE_SENDER.borrow();
    refcell.replace(Some(inner));
}

pub fn start_rust_logic() {
    // Thread 1 running Rust
    crate::main();
}

pub fn send_user_action(task_address: String, serialized: Serialized) -> SyncReturn<()> {
    // Thread 0 running Dart
    let refcell = USER_ACTION_SENDER.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    let sender = option.expect("User action sender does not exist!");
    let user_action = (task_address, serialized);
    sender.try_send(user_action).ok();
    SyncReturn(())
}

/// This function is meant to be called when Dart's hot restart is triggered in debug mode.
pub fn clean_viewmodel() -> SyncReturn<()> {
    // Thread 0 running Dart
    let mut hashmap = VIEWMODEL.borrow_mut();
    *hashmap = HashMap::new();
    SyncReturn(())
}

pub fn read_viewmodel(item_address: String) -> SyncReturn<Option<Serialized>> {
    // Thread 0 running Dart
    let mut hashmap = VIEWMODEL.borrow_mut();
    let receiver_refcell = VIEWMODEL_UPDATE_RECEIVER.borrow();
    let receiver_option = receiver_refcell.replace(None);
    let mut receiver = receiver_option.expect("Viewmodel update receiver does not exist!");
    while let Ok(viewmodel_update) = receiver.try_recv() {
        hashmap.insert(viewmodel_update.0, viewmodel_update.1);
    }
    receiver_refcell.replace(Some(receiver));
    let item_option = hashmap.get(&item_address).cloned();
    SyncReturn(item_option)
}
