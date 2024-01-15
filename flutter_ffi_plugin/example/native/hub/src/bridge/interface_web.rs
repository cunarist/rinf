use super::interface::*;
use rinf::externs::js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_rust_logic_extern() {
    start_rust_logic();
}

#[wasm_bindgen]
pub fn stop_rust_logic_extern() {
    stop_rust_logic();
}

#[wasm_bindgen]
pub fn send_dart_signal_extern(message_id: i32, message_raw: &[u8], blob_raw: &[u8]) {
    let message_bytes = message_raw.to_vec();
    let blob_bytes = blob_raw.to_vec();

    crate::messages::receive::receive_messages(message_id, message_bytes, blob_bytes);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = rinf_send_rust_signal_extern)]
    pub fn send_rust_signal_extern_raw(
        resource: i32,
        message_raw: Uint8Array,
        blob_raw: Uint8Array,
    );
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_name = rinf_send_rust_report_extern)]
    pub fn send_rust_report_extern(rust_report: String);
}

pub fn send_rust_signal_extern(message_id: i32, message_bytes: Vec<u8>, blob_bytes: Vec<u8>) {
    send_rust_signal_extern_raw(
        message_id,
        rinf::externs::js_sys::Uint8Array::from(message_bytes.as_slice()),
        rinf::externs::js_sys::Uint8Array::from(blob_bytes.as_slice()),
    );
}
