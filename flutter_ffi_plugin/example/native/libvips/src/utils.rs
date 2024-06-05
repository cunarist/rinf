// (c) Copyright 2019-2024 OLX
use crate::bindings;
use crate::bindings::{VipsArrayDouble, VipsArrayImage, VipsArrayInt};
use crate::error::Error;
use crate::Result;
use crate::VipsImage;
use std::ffi::c_void;
use std::ffi::CString;

pub(crate) struct VipsArrayIntWrapper {
    pub ctx: *mut VipsArrayInt,
}
pub(crate) struct VipsArrayDoubleWrapper {
    pub ctx: *mut VipsArrayDouble,
}
pub(crate) struct VipsArrayImageWrapper {
    pub ctx: *mut VipsArrayImage,
}

impl Drop for VipsArrayIntWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl Drop for VipsArrayDoubleWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl Drop for VipsArrayImageWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl From<&[i32]> for VipsArrayIntWrapper {
    #[inline]
    fn from(array: &[i32]) -> Self {
        VipsArrayIntWrapper {
            ctx: unsafe { bindings::vips_array_int_new(array.as_ptr(), array.len() as i32) },
        }
    }
}

impl From<&[f64]> for VipsArrayDoubleWrapper {
    #[inline]
    fn from(array: &[f64]) -> Self {
        VipsArrayDoubleWrapper {
            ctx: unsafe { bindings::vips_array_double_new(array.as_ptr(), array.len() as i32) },
        }
    }
}

impl From<&[VipsImage]> for VipsArrayImageWrapper {
    #[inline]
    fn from(array: &[VipsImage]) -> Self {
        let len = array.len() as i32;
        let as_vips = array.iter().map(|v| v.ctx).collect::<Vec<_>>().as_mut_ptr();
        VipsArrayImageWrapper {
            ctx: unsafe { bindings::vips_array_image_new(as_vips, len) },
        }
    }
}

#[inline]
pub fn result<T>(res: i32, output: T, error: Error) -> Result<T> {
    if res == 0 {
        Ok(output)
    } else {
        Err(error)
    }
}

#[inline]
pub(crate) fn new_c_string(string: &str) -> Result<CString> {
    CString::new(string).map_err(|_| Error::InitializationError("Error initializing C string."))
}

#[inline]
pub(crate) unsafe fn new_byte_array(buf: *mut c_void, size: u64) -> Vec<u8> {
    Vec::from_raw_parts(buf as *mut u8, size as usize, size as usize)
}

#[inline]
pub unsafe fn new_int_array(array: *mut i32, size: u64) -> Vec<i32> {
    Vec::from(std::slice::from_raw_parts(array as *mut i32, size as usize))
}

#[inline]
pub unsafe fn new_double_array(array: *mut f64, size: u64) -> Vec<f64> {
    Vec::from(std::slice::from_raw_parts(array as *mut f64, size as usize))
}
