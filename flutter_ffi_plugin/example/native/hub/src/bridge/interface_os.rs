use super::interface::*;
use allo_isolate::IntoDart;
use allo_isolate::Isolate;
use rinf::externs::lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref ISOLATE_SIGNAL: SharedCell<Isolate> = Arc::new(Mutex::new(RefCell::new(None)));
}

#[no_mangle]
pub extern "C" fn prepare_isolates_extern(port_signal: i64) {
    let isolate = Isolate::new(port_signal);
    let cell = ISOLATE_SIGNAL.lock().unwrap();
    cell.replace(Some(isolate));
}

#[no_mangle]
pub extern "C" fn start_rust_logic_extern() {
    start_rust_logic();
}

#[no_mangle]
pub extern "C" fn stop_rust_logic_extern() {
    stop_rust_logic();
}

#[no_mangle]
pub extern "C" fn send_dart_signal_extern(
    message_id: i64,
    message_pointer: *const u8,
    message_size: usize,
    blob_valid: bool,
    blob_pointer: *const u8,
    blob_size: usize,
) {
    let message_bytes =
        unsafe { Vec::from_raw_parts(message_pointer as *mut u8, message_size, message_size) };
    let blob = if blob_valid {
        unsafe {
            Some(Vec::from_raw_parts(
                blob_pointer as *mut u8,
                blob_size,
                blob_size,
            ))
        }
    } else {
        None
    };
    crate::messages::handle::handle_signal(message_id as i32, message_bytes, blob);
}

pub fn send_rust_signal_extern(
    message_id: i32,
    message_bytes: Vec<u8>,
    blob_valid: bool,
    blob_bytes: Vec<u8>,
) {
    let cell = ISOLATE_SIGNAL.lock().unwrap();
    let dart_isolate = cell.borrow().unwrap();
    dart_isolate.post(
        vec![
            message_id.into_dart(),
            message_bytes.into_dart(),
            blob_valid.into_dart(),
            blob_bytes.into_dart(),
        ]
        .into_dart(),
    );
}
