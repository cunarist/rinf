use super::SharedLock;
use allo_isolate::IntoDart;
use allo_isolate::Isolate;
use allo_isolate::ZeroCopyBuffer;
use std::panic::catch_unwind;
use std::sync::Mutex;
use std::sync::OnceLock;

static DART_ISOLATE: SharedLock<Isolate> = OnceLock::new();

#[no_mangle]
pub extern "C" fn prepare_isolate_extern(port: i64) {
    let _ = catch_unwind(|| {
        let dart_isolate = Isolate::new(port);
        let mut guard = DART_ISOLATE
            .get_or_init(|| Mutex::new(None))
            .lock()
            .unwrap();
        guard.replace(dart_isolate);
    });
}

pub fn send_rust_signal_extern(message_id: i32, message_bytes: Vec<u8>, binary: Vec<u8>) {
    let dart_isolate = DART_ISOLATE.get().unwrap().lock().unwrap().unwrap();

    // If a `Vec<u8>` is empty, we can't just simply send it to Dart
    // because panic can occur from null pointers.
    // Instead, we will reconstruct the empty vector from the Dart side.
    let message_filled = !message_bytes.is_empty();
    let binary_filled = !binary.is_empty();

    dart_isolate.post(
        vec![
            message_id.into_dart(),
            if message_filled {
                ZeroCopyBuffer(message_bytes).into_dart()
            } else {
                ().into_dart()
            },
            if binary_filled {
                ZeroCopyBuffer(binary).into_dart()
            } else {
                ().into_dart()
            },
        ]
        .into_dart(),
    );
}
