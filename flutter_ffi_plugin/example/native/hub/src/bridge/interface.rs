#![allow(dead_code)]

use crate::tokio;

#[cfg(not(target_family = "wasm"))]
pub use super::interface_os::*;
#[cfg(target_family = "wasm")]
pub use super::interface_web::*;

#[cfg(not(target_family = "wasm"))]
rinf::externs::lazy_static::lazy_static! {
    pub static ref TOKIO_RUNTIME: rinf::externs::os_thread_local::ThreadLocal<rinf::SimpleCell<tokio::runtime::Runtime>> =
        rinf::externs::os_thread_local::ThreadLocal::new(|| std::cell::RefCell::new(None));
}

/// Start the main function of Rust.
pub fn start_rust_logic() {
    // Enable backtrace output for panics.
    #[cfg(debug_assertions)]
    {
        #[cfg(not(target_family = "wasm"))]
        {
            use rinf::externs::backtrace;
            std::panic::set_hook(Box::new(|panic_info| {
                let mut frames_filtered = Vec::new();
                backtrace::trace(|frame| {
                    // Filter some backtrace frames
                    // as those from infrastructure functions are not needed.
                    let mut should_keep_tracing = true;
                    backtrace::resolve_frame(frame, |symbol| {
                        if let Some(symbol_name) = symbol.name() {
                            let name = symbol_name.to_string();
                            let name_trimmed = name.trim_start_matches('_');
                            if name_trimmed.starts_with("rust_begin_unwind") {
                                frames_filtered.clear();
                                return;
                            }
                            if name_trimmed.starts_with("rust_try") {
                                should_keep_tracing = false;
                                return;
                            }
                        }
                        let backtrace_frame = backtrace::BacktraceFrame::from(frame.to_owned());
                        frames_filtered.push(backtrace_frame);
                    });
                    should_keep_tracing
                });
                let mut backtrace_filtered = backtrace::Backtrace::from(frames_filtered);
                backtrace_filtered.resolve();
                crate::debug_print!(
                    "A panic occurred in Rust.\n{}\n{:?}",
                    panic_info,
                    backtrace_filtered
                );
            }));
        }
        #[cfg(target_family = "wasm")]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
            }));
        }
    }

    // Run the main function.
    #[cfg(not(target_family = "wasm"))]
    {
        TOKIO_RUNTIME.with(move |inner| {
            let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            tokio_runtime.spawn(crate::main());
            // If there was already a tokio runtime previously,
            // most likely due to Dart's hot restart,
            // its tasks as well as itself will be terminated,
            // being replaced with the new one.
            inner.replace(Some(tokio_runtime));
        });
    }
    #[cfg(target_family = "wasm")]
    {
        tokio::spawn(crate::main());
    }
}

/// Stop and terminate all Rust tasks.
pub fn stop_rust_logic() {
    #[cfg(not(target_family = "wasm"))]
    TOKIO_RUNTIME.with(move |ref_cell| {
        ref_cell.replace(None);
    });
}

/// Send a signal to Dart.
pub fn send_rust_signal(message_id: i32, message_bytes: Vec<u8>, blob: Option<Vec<u8>>) {
    send_rust_signal_extern(
        message_id,
        message_bytes,
        blob.is_some(),
        blob.unwrap_or_default(),
    );
}

/// Send a string to Dart that should be printed in the CLI.
/// Do NOT use this function directly in the code.
/// Use `debug_print!` macro instead.
#[cfg(debug_assertions)]
pub fn send_rust_report(rust_report: String) {
    send_rust_signal_extern(
        -1, // This is a special message ID for Rust reports
        Vec::new(),
        true,
        rust_report.into_bytes(),
    );
}
