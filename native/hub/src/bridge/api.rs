use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::iter::ExactSizeIterator;
use std::ops::Index;
use std::ops::IndexMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

#[derive(Clone, Eq, Hash, PartialEq)]

pub struct DotAddress {
    pub layered: Vec<String>,
}
impl From<&str> for DotAddress {
    fn from(plain: &str) -> DotAddress {
        let layered = plain.split('.').map(String::from).collect();
        DotAddress { layered }
    }
}
impl Index<usize> for DotAddress {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.layered[index]
    }
}
impl IndexMut<usize> for DotAddress {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.layered[index]
    }
}
impl Iterator for DotAddress {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.layered.pop()
    }
}
impl ExactSizeIterator for DotAddress {
    fn len(&self) -> usize {
        self.layered.len()
    }
}
impl ToString for DotAddress {
    fn to_string(&self) -> String {
        self.layered.join(".")
    }
}

type UserActionSender = OnceCell<Mutex<Sender<(DotAddress, String)>>>;
pub static USER_ACTION_SENDER: UserActionSender = OnceCell::new();
type UserActionReceiver = OnceCell<Mutex<Receiver<(DotAddress, String)>>>;
pub static USER_ACTION_RECEIVER: UserActionReceiver = OnceCell::new();
type ViewmodelUpdateSender = OnceCell<Mutex<Sender<(DotAddress, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_SENDER: ViewmodelUpdateSender = OnceCell::new();
type ViewmodelUpdateReceiver = OnceCell<Mutex<Receiver<(DotAddress, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_RECEIVER: ViewmodelUpdateReceiver = OnceCell::new();

type ViewModel = OnceCell<Mutex<HashMap<DotAddress, Vec<u8>>>>;
static VIEWMODEL: ViewModel = OnceCell::new();

static IS_RUST_LOGIC_STARTED: AtomicBool = AtomicBool::new(false);
static DART_HOT_RESTART_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn start_and_get_viewmodel_update_stream(viewmodel_update_stream: StreamSink<String>) {
    // Thread by flutter_rust_bridge

    DART_HOT_RESTART_COUNT.fetch_add(1, Ordering::SeqCst);
    let hot_restart_number = DART_HOT_RESTART_COUNT.load(Ordering::SeqCst);

    if !IS_RUST_LOGIC_STARTED.load(Ordering::SeqCst) {
        // Dart first run
        IS_RUST_LOGIC_STARTED.store(true, Ordering::SeqCst);
        let (sender, receiver) = channel();
        USER_ACTION_SENDER.set(Mutex::new(sender)).ok();
        USER_ACTION_RECEIVER.set(Mutex::new(receiver)).ok();
        let (sender, receiver) = channel();
        VIEWMODEL_UPDATE_SENDER.set(Mutex::new(sender)).ok();
        VIEWMODEL_UPDATE_RECEIVER.set(Mutex::new(receiver)).ok();
        let viewmodel = HashMap::<DotAddress, Vec<u8>>::new();
        VIEWMODEL.set(Mutex::new(viewmodel)).ok();
        std::thread::spawn(crate::main);
    } else {
        // Dart hot restart
        let sender = VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
        sender.send((DotAddress::from("breakTheLoop"), vec![])).ok();
    }

    std::thread::spawn(move || {
        let receiver = VIEWMODEL_UPDATE_RECEIVER.get().unwrap().lock().unwrap();
        loop {
            if let Ok(received) = receiver.recv() {
                let data_address = received.0;
                let bytes = received.1;
                let mut viewmodel = VIEWMODEL.get().unwrap().lock().unwrap();
                viewmodel.insert(data_address.clone(), bytes);
                viewmodel_update_stream.add(data_address.to_string());
            }
            if hot_restart_number < DART_HOT_RESTART_COUNT.load(Ordering::SeqCst) {
                // When another `StreamSink` is established by hot restart
                break;
            }
        }
    });
}

pub fn read_viewmodel(
    data_address: DotAddress,
    take_ownership: bool,
) -> SyncReturn<Option<Vec<u8>>> {
    let mut viewmodel = VIEWMODEL.get().unwrap().lock().unwrap();
    let bytes = if take_ownership {
        viewmodel.remove(&data_address)
    } else {
        viewmodel.get(&data_address).cloned()
    };
    SyncReturn(bytes)
}

pub fn send_user_action(task_address: DotAddress, json_string: String) -> SyncReturn<()> {
    // Main thread by Flutter

    let user_action = (task_address, json_string);
    let sender = USER_ACTION_SENDER.get().unwrap().lock().unwrap();
    sender.send(user_action).ok();
    SyncReturn(())
}
