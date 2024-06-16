#[macro_export]
/// Writes the interface code
/// needed to communicate with Dart.
/// This should be used once, and only once,
/// at the root of the `hub` crate.
macro_rules! write_interface {
    () => {
        #[cfg(not(target_family = "wasm"))]
        #[no_mangle]
        pub extern "C" fn start_rust_logic_extern() {
            let result = $crate::start_rust_logic(main());
            if let Err(error) = result {
                println!("Could not start Rust logic.\n{error:#?}");
            }
        }

        #[cfg(target_family = "wasm")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn start_rust_logic_extern() {
            let result = $crate::start_rust_logic(main());
            if let Err(error) = result {
                println!("Could not start Rust logic.\n{error:#?}");
            }
        }

        #[cfg(not(target_family = "wasm"))]
        #[no_mangle]
        pub unsafe extern "C" fn send_dart_signal_extern(
            message_id: i32,
            message_pointer: *const u8,
            message_size: usize,
            binary_pointer: *const u8,
            binary_size: usize,
        ) {
            let message_bytes =
                unsafe { std::slice::from_raw_parts(message_pointer, message_size) };
            let binary =
                unsafe { std::slice::from_raw_parts(binary_pointer, binary_size).to_vec() };
            messages::generated::handle_dart_signal(message_id, message_bytes, binary);
        }

        #[cfg(target_family = "wasm")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn send_dart_signal_extern(message_id: i32, message_bytes: &[u8], binary: &[u8]) {
            let message_bytes = message_bytes.to_vec();
            let binary = binary.to_vec();
            messages::generated::handle_dart_signal(message_id, message_bytes, binary);
        }
    };
}

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!()`,
/// as it seamlessly adapts to different environments.
/// Note that this macro does nothing in release mode.
#[macro_export]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {
        let rust_report = format!( $( $t )* );
        #[cfg(debug_assertions)]
        $crate::send_rust_signal(
            -1, // This is a special message ID for Rust reports
            Vec::new(),
            rust_report.into_bytes(),
        );
        #[cfg(not(debug_assertions))]
        let _ = rust_report;
    }
}
