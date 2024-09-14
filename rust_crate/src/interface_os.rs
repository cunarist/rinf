use crate::error::RinfError;
use crate::shutdown::{create_shutdown_channel, SHUTDOWN_SENDER};
use allo_isolate::{IntoDart, Isolate, ZeroCopyBuffer};
use std::sync::Mutex;
use std::thread;

static DART_ISOLATE: Mutex<Option<Isolate>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn prepare_isolate_extern(port: i64) {
    let dart_isolate = Isolate::new(port);
    let mut guard = match DART_ISOLATE.lock() {
        Ok(inner) => inner,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.replace(dart_isolate);
}

pub fn start_rust_logic_real<F, T>(main_fn: F) -> Result<(), RinfError>
where
    F: Fn() -> T + Send + 'static,
{
    // Enable backtrace output for panics.
    #[cfg(debug_assertions)]
    {
        #[cfg(not(feature = "backtrace"))]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
            }));
        }
        #[cfg(feature = "backtrace")]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                let backtrace = backtrace::Backtrace::new();
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}\n{backtrace:?}");
            }));
        }
    }

    // Prepare the channel that will help notify async runtime to shutdown
    // after the main Dart thread has gone.
    let shutdown_reporter = create_shutdown_channel()?;

    // Run the async runtime.
    thread::spawn(move || {
        main_fn();
        // After the runtime is closed, tell the main thread to stop waiting.
        drop(shutdown_reporter);
    });

    Ok(())
}

#[no_mangle]
pub extern "C" fn stop_rust_logic_extern() {
    let sender_option = SHUTDOWN_SENDER.with(|cell| cell.take());
    if let Some(shutdown_sender) = sender_option {
        // Dropping the sender tells the async runtime to stop running.
        // Also, it blocks the main thread until
        // it gets the report that async runtime is dropped.
        drop(shutdown_sender);
    }
}

pub fn send_rust_signal_real(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<(), RinfError> {
    // When `DART_ISOLATE` is not initialized, just return the error.
    // This can happen when running test code in Rust.
    let guard = match DART_ISOLATE.lock() {
        Ok(inner) => inner,
        Err(poisoned) => poisoned.into_inner(),
    };
    let dart_isolate = guard.as_ref().ok_or(RinfError::NoDartIsolate)?;

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

    Ok(())
}
