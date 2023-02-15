use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_prepare_rust_signal_stream(port_: MessagePort) {
    wire_prepare_rust_signal_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_prepare_rust_response_stream(port_: MessagePort) {
    wire_prepare_rust_response_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_prepare_channels() -> support::WireSyncReturn {
    wire_prepare_channels_impl()
}

#[wasm_bindgen]
pub fn wire_start_rust_logic(port_: MessagePort) {
    wire_start_rust_logic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_request_to_rust(request_unique: JsValue) -> support::WireSyncReturn {
    wire_request_to_rust_impl(request_unique)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<RustRequest> for JsValue {
    fn wire2api(self) -> RustRequest {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        RustRequest {
            address: self_.get(0).wire2api(),
            operation: self_.get(1).wire2api(),
            bytes: self_.get(2).wire2api(),
        }
    }
}
impl Wire2Api<RustRequestUnique> for JsValue {
    fn wire2api(self) -> RustRequestUnique {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        RustRequestUnique {
            id: self_.get(0).wire2api(),
            request: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Operation> for JsValue {
    fn wire2api(self) -> Operation {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
