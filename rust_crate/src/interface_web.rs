use crate::error::RinfError;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub fn start_rust_logic_real<F, T>(main_fn: F) -> Result<(), RinfError>
where
    F: Fn() -> T + 'static,
{
    // Add kind description for panics.
    #[cfg(debug_assertions)]
    {
        std::panic::set_hook(Box::new(|panic_info| {
            crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
        }));
    }

    // Run the main function.
    main_fn();

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = rinf, catch)]
    fn send_rust_signal_extern(
        resource: i32,
        message_bytes: Uint8Array,
        binary: Uint8Array,
    ) -> Result<(), JsValue>; // catch the JS exception
}

pub fn send_rust_signal_real(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<(), RinfError> {
    match send_rust_signal_extern(
        message_id,
        js_sys::Uint8Array::from(message_bytes.as_slice()),
        js_sys::Uint8Array::from(binary.as_slice()),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(RinfError::NoSignalHandler)
        }
    }
}
