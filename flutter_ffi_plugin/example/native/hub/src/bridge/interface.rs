#![allow(dead_code)]

use rinf::externs::lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_with_wasm::tokio;

/// Available operations that a `RustRequest` object can hold.
/// There are 4 options, `Create`,`Read`,`Update`, and `Delete`.
pub enum RustOperation {
    Create,
    Read,
    Update,
    Delete,
}

/// Holds the data that Rust streams to Dart.
#[derive(Clone)]
pub struct RustSignal {
    pub resource: i32,
    pub message: Option<Vec<u8>>,
    pub blob: Option<Vec<u8>>,
}

/// Request object that is sent from Dart to Rust.
pub struct RustRequest {
    pub resource: i32,
    pub operation: RustOperation,
    pub message: Option<Vec<u8>>,
    pub blob: Option<Vec<u8>>,
}

/// Wrapper for `RustRequest` with a unique ID.
pub struct RustRequestUnique {
    pub id: i32,
    pub request: RustRequest,
}

/// Response object that is sent from Rust to Dart.
#[derive(Clone)]
pub struct RustResponse {
    pub message: Option<Vec<u8>>,
    pub blob: Option<Vec<u8>>,
}

/// Wrapper for `RustResponse` with a unique ID.
#[derive(Clone)]
pub struct RustResponseUnique {
    pub id: i32,
    pub response: Option<RustResponse>,
}

type Cell<T> = RefCell<Option<T>>;
type SharedCell<T> = Arc<Mutex<Cell<T>>>;

type RustRequestSender = Sender<RustRequestUnique>;
type RustRequestReceiver = Receiver<RustRequestUnique>;

// Native: Main thread
// Web: Worker thread
thread_local! {
    pub static REQUEST_SENDER: Cell<RustRequestSender> = RefCell::new(None);
}

// Native: All threads
// Web: Worker thread
lazy_static! {
    pub static ref REQUST_RECEIVER_SHARED: SharedCell<RustRequestReceiver> =
        Arc::new(Mutex::new(RefCell::new(None)));
}

#[cfg(not(target_family = "wasm"))]
lazy_static! {
    pub static ref TOKIO_RUNTIME: rinf::externs::os_thread_local::ThreadLocal<Cell<tokio::runtime::Runtime>> =
        rinf::externs::os_thread_local::ThreadLocal::new(|| RefCell::new(None));
}

#[cfg(target_family = "wasm")]
thread_local! {
    pub static IS_MAIN_STARTED: RefCell<bool> = RefCell::new(false);
}

/// Prepare channels that are used in the Rust world.
pub fn prepare_channels() {
    let (request_sender, request_receiver) = channel(1024);
    REQUEST_SENDER.with(move |inner| {
        inner.replace(Some(request_sender));
    });
    let cell = REQUST_RECEIVER_SHARED.lock().unwrap();
    cell.replace(Some(request_receiver));
}

/// Start the main function of Rust.
pub fn start_rust_logic() {
    #[cfg(not(target_family = "wasm"))]
    {
        use rinf::externs::backtrace;
        #[cfg(debug_assertions)]
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
        TOKIO_RUNTIME.with(move |inner| {
            let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            tokio_runtime.spawn(crate::main());
            inner.replace(Some(tokio_runtime));
        });
    }
    #[cfg(target_family = "wasm")]
    {
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(|panic_info| {
            crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
        }));
        IS_MAIN_STARTED.with(move |ref_cell| {
            let is_started = *ref_cell.borrow();
            if !is_started {
                tokio::spawn(crate::main());
                ref_cell.replace(true);
            }
        });
    }
}

/// Stop and terminate all Rust tasks.
pub fn stop_rust_logic() {
    #[cfg(not(target_family = "wasm"))]
    TOKIO_RUNTIME.with(move |ref_cell| {
        ref_cell.replace(None);
    });
}

/// Send a request to Rust and receive a response in Dart.
pub fn request_to_rust(request_unique: RustRequestUnique) {
    REQUEST_SENDER.with(move |inner| {
        let borrowed = inner.borrow();
        let sender = borrowed.as_ref().unwrap();
        sender.try_send(request_unique).ok();
    });
}

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_request_receiver() -> Receiver<RustRequestUnique> {
    let cell = REQUST_RECEIVER_SHARED.lock().unwrap();
    let option = cell.replace(None);
    option.unwrap()
}

/// Sending the signal will notify the Flutter widgets
/// and trigger the rebuild.
/// No memory copy is involved as the bytes are moved directly to Dart.
pub fn send_rust_signal(rust_signal: RustSignal) {
    #[cfg(not(target_family = "wasm"))]
    super::interface_os::send_rust_signal_extern(rust_signal);
}

/// Sends a response to Dart with a unique interaction ID
/// to remember which request that response corresponds to.
/// No memory copy is involved as the bytes are moved directly to Dart.
pub fn respond_to_dart(response_unique: RustResponseUnique) {
    #[cfg(not(target_family = "wasm"))]
    super::interface_os::respond_to_dart_extern(response_unique);
}

/// Sends a string to Dart that should be printed in the CLI.
/// Do NOT use this function directly in the code.
/// Use `debug_print!` macro instead.
#[cfg(debug_assertions)]
pub fn send_rust_report(rust_report: String) {
    #[cfg(not(target_family = "wasm"))]
    super::interface_os::send_rust_report_extern(rust_report);
}
