//! This module communicates with Dart.
//! More specifically, receiveing requests and
//! sending responses updates are supported.
//! DO NOT EDIT.
#![allow(dead_code)]

use api::RustSignal;
use tokio::sync::mpsc::Receiver;

use self::api::RustResponseUnique;

pub mod api;
mod bridge_generated;

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_request_receiver() -> Receiver<api::RustRequestUnique> {
    let cell = api::REQUST_RECEIVER_SHARED.lock().unwrap();
    let option = cell.replace(None);
    option.unwrap()
}

/// Sending the signal will notify the Flutter widgets
/// and trigger the rebuild.
/// No memory copy is involved as the bytes are moved directly to Dart,
/// thanks to the `zero-copy` feature of `flutter_rust_bridge`.
pub fn send_rust_signal(rust_signal: RustSignal) {
    api::SIGNAL_STREAM.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        if let Some(stream) = option {
            stream.add(rust_signal);
        } else {
            let cell = api::SIGNAL_STREAM_SHARED.lock().unwrap();
            let stream = cell.borrow().as_ref().unwrap().clone();
            stream.add(rust_signal);
            borrowed.replace(stream);
        }
    });
}

pub fn respond_to_dart(response_unique: RustResponseUnique) {
    api::RESPONSE_STREAM.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        if let Some(stream) = option {
            stream.add(response_unique);
        } else {
            let cell = api::RESPONSE_STREAM_SHARED.lock().unwrap();
            let stream = cell.borrow().as_ref().unwrap().clone();
            stream.add(response_unique);
            borrowed.replace(stream);
        }
    });
}
