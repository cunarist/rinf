use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_start_and_get_viewmodel_update_stream(port_: i64) {
    wire_start_and_get_viewmodel_update_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_read_viewmodel(
    item_address: *mut wire_uint_8_list,
    take_ownership: bool,
) -> support::WireSyncReturn {
    wire_read_viewmodel_impl(item_address, take_ownership)
}

#[no_mangle]
pub extern "C" fn wire_send_user_action(
    task_address: *mut wire_uint_8_list,
    json_string: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_send_user_action_impl(task_address, json_string)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
