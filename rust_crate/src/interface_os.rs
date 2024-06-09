use crate::debug_print;
use allo_isolate::{IntoDart, Isolate, ZeroCopyBuffer};
use os_thread_local::ThreadLocal;
use std::cell::RefCell;
use std::future::Future;
use std::panic::catch_unwind;
use std::sync::{Mutex, OnceLock};
use tokio::runtime::{Builder, Runtime};

static DART_ISOLATE: Mutex<Option<Isolate>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn prepare_isolate_extern(port: i64) {
    let _ = catch_unwind(|| {
        let dart_isolate = Isolate::new(port);
        let mut guard = DART_ISOLATE.lock().unwrap();
        guard.replace(dart_isolate);
    });
}

// We use `os_thread_local` so that when the program fails
// and the main thread exits unexpectedly,
// the whole async tokio runtime can disappear as well.
// Without this solution, zombie threads inside the tokio runtime
// might outlive the app.
type TokioRuntime = OnceLock<ThreadLocal<RefCell<Option<Runtime>>>>;
static TOKIO_RUNTIME: TokioRuntime = OnceLock::new();

pub fn start_rust_logic_real<F>(main_future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    // Enable backtrace output for panics.
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|panic_info| {
            let backtrace = backtrace::Backtrace::new();
            debug_print!("A panic occurred in Rust.\n{panic_info}\n{backtrace:?}");
        }));
    }

    // Run the main function.
    let tokio_runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    tokio_runtime.spawn(main_future);
    TOKIO_RUNTIME
        .get_or_init(|| ThreadLocal::new(|| RefCell::new(None)))
        .with(move |cell| {
            // If there was already a tokio runtime previously,
            // most likely due to Dart's hot restart,
            // its tasks as well as itself will be terminated,
            // being replaced with the new one.
            cell.replace(Some(tokio_runtime));
        });
}

pub fn send_rust_signal_real(message_id: i32, message_bytes: Vec<u8>, binary: Vec<u8>) {
    // When `DART_ISOLATE` is not initialized, do nothing.
    // This can happen when running test code in Rust.
    let guard = DART_ISOLATE.lock().unwrap();
    let dart_isolate = match guard.as_ref() {
        Some(inner) => inner,
        None => {
            debug_print!("Dart isolate for sending Rust signals is not present.");
            return;
        }
    };

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
