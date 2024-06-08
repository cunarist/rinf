use crate::debug_print;
use js_sys::Uint8Array;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub fn start_rust_logic_extern<F>(main_future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    // Add kind description for panics.
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|panic_info| {
            debug_print!("A panic occurred in Rust.\n{panic_info}");
        }));
    }

    // Run the main function.
    spawn_local(main_future);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = rinf, js_name = send_rust_signal_extern)]
    pub fn send_rust_signal_extern_raw(
        resource: i32,
        message_bytes: Uint8Array,
        binary: Uint8Array,
    );
}

pub fn send_rust_signal_extern(message_id: i32, message_bytes: Vec<u8>, binary: Vec<u8>) {
    send_rust_signal_extern_raw(
        message_id,
        js_sys::Uint8Array::from(message_bytes.as_slice()),
        js_sys::Uint8Array::from(binary.as_slice()),
    );
}
