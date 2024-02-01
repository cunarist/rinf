use super::SharedCell;
use allo_isolate::IntoDart;
use allo_isolate::Isolate;
use allo_isolate::ZeroCopyBuffer;
use std::cell::RefCell;
use std::panic::catch_unwind;
use std::sync::Mutex;
use std::sync::OnceLock;

static DART_ISOLATE: SharedCell<Isolate> = OnceLock::new();

#[no_mangle]
pub extern "C" fn prepare_isolate_extern(port: i64) {
    let _ = catch_unwind(|| {
        let dart_isolate = Isolate::new(port);
        let cell = DART_ISOLATE
            .get_or_init(|| Mutex::new(RefCell::new(None)))
            .lock()
            .unwrap();
        cell.replace(Some(dart_isolate));
    });
}

pub fn send_rust_signal_extern(
    message_id: i32,
    message_bytes: Vec<u8>,
    blob_valid: bool,
    blob_bytes: Vec<u8>,
) {
    let cell = DART_ISOLATE.get().unwrap().lock().unwrap();
    let dart_isolate = cell.borrow().unwrap();
    dart_isolate.post(
        vec![
            message_id.into_dart(),
            ZeroCopyBuffer(message_bytes).into_dart(),
            blob_valid.into_dart(),
            ZeroCopyBuffer(blob_bytes).into_dart(),
        ]
        .into_dart(),
    );
}
