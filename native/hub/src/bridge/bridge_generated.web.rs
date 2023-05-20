use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_prepare_viewmodel_update_stream(port_: MessagePort) {
    wire_prepare_viewmodel_update_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_prepare_channels() -> support::WireSyncReturn {
    wire_prepare_channels_impl()
}

#[wasm_bindgen]
pub fn wire_lay_endpoints_on_rust_thread(port_: MessagePort, rust_opaque: JsValue) {
    wire_lay_endpoints_on_rust_thread_impl(port_, rust_opaque)
}

#[wasm_bindgen]
pub fn wire_start_rust_logic(port_: MessagePort) {
    wire_start_rust_logic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_send_user_action(task_address: String, serialized: JsValue) -> support::WireSyncReturn {
    wire_send_user_action_impl(task_address, serialized)
}

#[wasm_bindgen]
pub fn wire_clean_viewmodel() -> support::WireSyncReturn {
    wire_clean_viewmodel_impl()
}

#[wasm_bindgen]
pub fn wire_read_viewmodel(item_address: String) -> support::WireSyncReturn {
    wire_read_viewmodel_impl(item_address)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_MutexEndpointsOnRustThread(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<EndpointsOnRustThread>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexEndpointsOnRustThread(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<EndpointsOnRustThread>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<Serialized> for JsValue {
    fn wire2api(self) -> Serialized {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Serialized {
            data: self_.get(0).wire2api(),
            formula: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<Mutex<EndpointsOnRustThread>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<EndpointsOnRustThread>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
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
