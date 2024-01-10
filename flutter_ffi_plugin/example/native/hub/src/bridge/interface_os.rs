use super::interface::*;
use allo_isolate::Isolate;
use rinf::externs::lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

#[no_mangle]
pub extern "C" fn start_rust_logic_extern() {
    start_rust_logic();
}

#[no_mangle]
pub extern "C" fn stop_rust_logic_extern() {
    stop_rust_logic();
}

#[no_mangle]
pub extern "C" fn request_to_rust_extern(
    interaction_id: i64,
    resource: i64,
    operation: i64,
    message_pointer: *const u8,
    message_size: usize,
    blob_pointer: *const u8,
    blob_size: usize,
) {
    let message = if message_size == 0 {
        None
    } else {
        unsafe {
            Some(Vec::from_raw_parts(
                message_pointer as *mut u8,
                message_size,
                message_size,
            ))
        }
    };

    let blob = if blob_size == 0 {
        None
    } else {
        unsafe {
            Some(Vec::from_raw_parts(
                blob_pointer as *mut u8,
                blob_size,
                blob_size,
            ))
        }
    };

    let operation_enum;
    if operation == 0 {
        operation_enum = RustOperation::Create;
    } else if operation == 1 {
        operation_enum = RustOperation::Read;
    } else if operation == 2 {
        operation_enum = RustOperation::Update;
    } else {
        operation_enum = RustOperation::Delete;
    }

    let rust_request = RustRequest {
        resource: resource as i32,
        operation: operation_enum,
        message,
        blob,
    };

    let rust_request_unique = RustRequestUnique {
        id: interaction_id as i32,
        request: rust_request,
    };

    request_to_rust(rust_request_unique);
}

type Cell<T> = RefCell<Option<T>>;
type SharedCell<T> = Arc<Mutex<Cell<T>>>;

// Native: `tokio` runtime threads
// Web: Worker thread
thread_local! {
    pub static SIGNAL_ISOLATE: Cell<Isolate> = RefCell::new(None);
    pub static RESPONSE_ISOLATE: Cell<Isolate> = RefCell::new(None);
    pub static REPORT_ISOLATE: Cell<Isolate> = RefCell::new(None);
}

// Native: All threads
// Web: Worker thread
lazy_static! {
    pub static ref SIGNAL_ISOLATE_SHARED: SharedCell<Isolate> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref RESPONSE_ISOLATE_SHARED: SharedCell<Isolate> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref REPORT_ISOLATE_SHARED: SharedCell<Isolate> =
        Arc::new(Mutex::new(RefCell::new(None)));
}

#[no_mangle]
pub extern "C" fn prepare_isolates_extern(port_signal: i64, port_response: i64, port_report: i64) {
    let isolate = Isolate::new(port_signal);
    let cell = SIGNAL_ISOLATE_SHARED.lock().unwrap();
    cell.replace(Some(isolate));

    let isolate = Isolate::new(port_response);
    let cell = RESPONSE_ISOLATE_SHARED.lock().unwrap();
    cell.replace(Some(isolate));

    #[cfg(debug_assertions)]
    {
        let isolate = Isolate::new(port_report);
        let cell = REPORT_ISOLATE_SHARED.lock().unwrap();
        cell.replace(Some(isolate));
    }
}

#[no_mangle]
pub extern "C" fn prepare_channels_extern() {
    prepare_channels();
}

impl allo_isolate::IntoDart for RustSignal {
    fn into_dart(self) -> allo_isolate::ffi::DartCObject {
        vec![
            self.resource.into_dart(),
            self.message.into_dart(),
            self.blob.into_dart(),
        ]
        .into_dart()
    }
}

pub fn send_rust_signal_extern(rust_signal: RustSignal) {
    SIGNAL_ISOLATE.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        if let Some(isolate) = option {
            isolate.post(rust_signal);
        } else {
            let cell = SIGNAL_ISOLATE_SHARED.lock().unwrap();
            let isolate = *cell.borrow().as_ref().unwrap();
            isolate.post(rust_signal);
            borrowed.replace(isolate);
        }
    });
}

impl allo_isolate::IntoDart for RustResponse {
    fn into_dart(self) -> allo_isolate::ffi::DartCObject {
        vec![self.message.into_dart(), self.blob.into_dart()].into_dart()
    }
}

impl allo_isolate::IntoDart for RustResponseUnique {
    fn into_dart(self) -> allo_isolate::ffi::DartCObject {
        vec![self.id.into_dart(), self.response.into_dart()].into_dart()
    }
}

pub fn respond_to_dart_extern(response_unique: RustResponseUnique) {
    RESPONSE_ISOLATE.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        if let Some(isolate) = option {
            isolate.post(response_unique);
        } else {
            let cell = RESPONSE_ISOLATE_SHARED.lock().unwrap();
            let isolate = *cell.borrow().as_ref().unwrap();
            isolate.post(response_unique);
            borrowed.replace(isolate);
        }
    });
}

#[cfg(debug_assertions)]
pub fn send_rust_report_extern(rust_report: String) {
    REPORT_ISOLATE.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        if let Some(isolate) = option {
            isolate.post(rust_report);
        } else {
            let cell = REPORT_ISOLATE_SHARED.lock().unwrap();
            let isolate = *cell.borrow().as_ref().unwrap();
            isolate.post(rust_report);
            borrowed.replace(isolate);
        }
    });
}
