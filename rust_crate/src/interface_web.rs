use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = rinf_send_rust_signal_extern)]
    pub fn send_rust_signal_extern_raw(
        resource: i32,
        message_bytes: Uint8Array,
        blob_valid: bool,
        blob_bytes: Uint8Array,
    );
}

pub fn send_rust_signal_extern(
    message_id: i32,
    message_bytes: Vec<u8>,
    blob_valid: bool,
    blob_bytes: Vec<u8>,
) {
    send_rust_signal_extern_raw(
        message_id,
        js_sys::Uint8Array::from(message_bytes.as_slice()),
        blob_valid,
        js_sys::Uint8Array::from(blob_bytes.as_slice()),
    );
}
