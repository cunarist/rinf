use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_prepare_viewmodel_update_stream(port_: i64) {
    wire_prepare_viewmodel_update_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_prepare_channels() -> support::WireSyncReturn {
    wire_prepare_channels_impl()
}

#[no_mangle]
pub extern "C" fn wire_lay_endpoints_on_rust_thread(
    port_: i64,
    rust_opaque: wire_MutexEndpointsOnRustThread,
) {
    wire_lay_endpoints_on_rust_thread_impl(port_, rust_opaque)
}

#[no_mangle]
pub extern "C" fn wire_start_rust_logic(port_: i64) {
    wire_start_rust_logic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_send_user_action(
    task_address: *mut wire_uint_8_list,
    serialized: *mut wire_Serialized,
) -> support::WireSyncReturn {
    wire_send_user_action_impl(task_address, serialized)
}

#[no_mangle]
pub extern "C" fn wire_read_viewmodel(
    item_address: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_read_viewmodel_impl(item_address)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MutexEndpointsOnRustThread() -> wire_MutexEndpointsOnRustThread {
    wire_MutexEndpointsOnRustThread::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_serialized_0() -> *mut wire_Serialized {
    support::new_leak_box_ptr(wire_Serialized::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_MutexEndpointsOnRustThread(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<EndpointsOnRustThread>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexEndpointsOnRustThread(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<EndpointsOnRustThread>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Mutex<EndpointsOnRustThread>>> for wire_MutexEndpointsOnRustThread {
    fn wire2api(self) -> RustOpaque<Mutex<EndpointsOnRustThread>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Serialized> for *mut wire_Serialized {
    fn wire2api(self) -> Serialized {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Serialized>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Serialized> for wire_Serialized {
    fn wire2api(self) -> Serialized {
        Serialized {
            data: self.data.wire2api(),
            formula: self.formula.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexEndpointsOnRustThread {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Serialized {
    data: *mut wire_uint_8_list,
    formula: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MutexEndpointsOnRustThread {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_Serialized {
    fn new_with_null_ptr() -> Self {
        Self {
            data: core::ptr::null_mut(),
            formula: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_Serialized {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
