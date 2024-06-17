use crate::common::*;
use js_sys::Uint8Array;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub fn start_rust_logic_real<F>(main_future: F) -> Result<()>
where
    F: Future<Output = ()> + 'static,
{
    // Add kind description for panics.
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|panic_info| {
            crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
        }));
    }

    // Run the main function.
    spawn_local(main_future);

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = rinf)]
    pub fn send_rust_signal_extern(resource: i32, message_bytes: Uint8Array, binary: Uint8Array);
}

pub fn send_rust_signal_real(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<()> {
    send_rust_signal_extern(
        message_id,
        js_sys::Uint8Array::from(message_bytes.as_slice()),
        js_sys::Uint8Array::from(binary.as_slice()),
    );

    Ok(())
}
