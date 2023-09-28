//! This module supports communication with Dart.
//! More specifically, sending responses and
//! stream signals to Dart are supported.
//! DO NOT EDIT.
#![allow(dead_code)]

use api::RustResponseUnique;
use api::RustSignal;
use tokio::sync::mpsc::Receiver;

pub mod api;
pub mod bridge_engine;
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
/// No memory copy is involved as the bytes are moved directly to Dart.
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

/// Sends a response to Dart with a unique interaction ID
/// to remember which request that response corresponds to.
/// No memory copy is involved as the bytes are moved directly to Dart.
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

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!()`,
/// as it seamlessly adapts to different environments.
/// Note that this macro produces nothing in release mode.
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {
        let print_content = format!( $( $t )* ).into();
        $crate::bridge::api::PRINT_STREAM.with(|inner| {
            let mut borrowed = inner.borrow_mut();
            let option = borrowed.as_ref();
            if let Some(stream) = option {
                stream.add(print_content);
            } else {
                let cell = $crate::bridge::api::PRINT_STREAM_SHARED.lock().unwrap();
                let stream = cell.borrow().as_ref().unwrap().clone();
                stream.add(print_content);
                borrowed.replace(stream);
            }
        });
    }
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {};
}
