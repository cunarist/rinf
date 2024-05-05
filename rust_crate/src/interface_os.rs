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
    binary_included: bool,
    binary_bytes: Vec<u8>,
) {
    let cell = DART_ISOLATE.get().unwrap().lock().unwrap();
    let dart_isolate = cell.borrow().unwrap();

    // If a `Vec<u8>` is empty, we can't just simply send it to Dart
    // because panic can occur from null pointers.
    // Instead, we will reconstruct the empty vector from the Dart side.
    let message_filled = !message_bytes.is_empty();
    let binary_filled = !binary_bytes.is_empty();

    dart_isolate.post(
        vec![
            message_id.into_dart(),
            if message_filled {
                ZeroCopyBuffer(message_bytes).into_dart()
            } else {
                ().into_dart()
            },
            binary_included.into_dart(),
            if binary_filled {
                ZeroCopyBuffer(binary_bytes).into_dart()
            } else {
                ().into_dart()
            },
        ]
        .into_dart(),
    );
}
