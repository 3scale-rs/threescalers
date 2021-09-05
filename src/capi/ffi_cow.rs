use std::prelude::v1::*;

use core::mem::ManuallyDrop;
use std::os::raw::c_char;

use std::borrow::Cow;

#[derive(Debug)]
#[repr(C)]
pub struct FFIStr {
    len: usize,
    ptr: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct FFIString {
    len: usize,
    cap: usize,
    ptr: *const c_char,
}

impl Drop for FFIString {
    fn drop(&mut self) {
        let _ = unsafe { String::from_raw_parts(self.ptr as *mut _, self.len, self.cap) };
    }
}

#[derive(Debug)]
#[repr(u8, C)]
pub enum FFICow {
    Borrowed(FFIStr),
    Owned(FFIString),
}

impl FFICow {
    pub fn len(&self) -> usize {
        match self {
            Self::Borrowed(FFIStr { len, .. }) | Self::Owned(FFIString { len, .. }) => *len,
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        match self {
            Self::Borrowed(FFIStr { ptr, .. }) | Self::Owned(FFIString { ptr, .. }) => *ptr,
        }
    }
}

impl From<&str> for FFICow {
    fn from(s: &str) -> Self {
        Self::Borrowed(s.into())
    }
}

impl From<Cow<'_, str>> for FFICow {
    fn from(c: Cow<'_, str>) -> Self {
        if let Cow::Owned(s) = c {
            FFICow::Owned(s.into())
        } else {
            FFICow::Borrowed(c.as_ref().into())
        }
    }
}

impl From<FFICow> for Cow<'_, str> {
    fn from(fc: FFICow) -> Self {
        match fc {
            FFICow::Borrowed(b) => {
                let s: &str = From::from(b);
                s.into()
            }
            FFICow::Owned(o) => {
                let s = String::from(o);
                s.into()
            }
        }
    }
}

impl From<FFIStr> for FFICow {
    fn from(fs: FFIStr) -> Self {
        FFICow::Borrowed(fs)
    }
}

impl From<FFIString> for FFICow {
    fn from(fs: FFIString) -> Self {
        FFICow::Owned(fs)
    }
}

impl From<&str> for FFIStr {
    fn from(s: &str) -> Self {
        Self {
            len: s.len(),
            ptr: s.as_ptr() as *const _,
        }
    }
}

impl From<String> for FFIString {
    fn from(s: String) -> Self {
        let s = ManuallyDrop::new(s);
        Self {
            len: s.len(),
            cap: s.capacity(),
            ptr: s.as_ptr() as *const _,
        }
    }
}

impl From<FFIStr> for &str {
    fn from(fs: FFIStr) -> Self {
        let s = unsafe { std::slice::from_raw_parts(fs.ptr as *const _, fs.len) };
        unsafe { std::str::from_utf8_unchecked(s) }
    }
}

impl From<FFIString> for String {
    fn from(fs: FFIString) -> Self {
        // FFIString will be dropped by first converting it to an
        // owned String so we need to ManuallyDrop it.
        let fs = ManuallyDrop::new(fs);
        unsafe { String::from_raw_parts(fs.ptr as *mut _, fs.len, fs.cap) }
    }
}

#[no_mangle]
pub extern "C" fn fficow_ptr_len(c: *const FFICow, ptr: *mut *const c_char) -> usize {
    if c.is_null() || ptr.is_null() {
        return 0;
    }

    let ffi_cow = unsafe { std::ptr::read::<FFICow>(c) };
    let ffi_cow = ManuallyDrop::new(ffi_cow);

    unsafe { *ptr = ffi_cow.as_ptr() };

    ffi_cow.len()
}

#[no_mangle]
pub extern "C" fn fficow_len(c: *const FFICow) -> usize {
    if c.is_null() {
        return 0;
    }
    let ffi_cow = unsafe { std::ptr::read::<FFICow>(c) };
    let ffi_cow = ManuallyDrop::new(ffi_cow);

    ffi_cow.len()
}

#[no_mangle]
pub extern "C" fn fficow_free(c: *const FFICow) {
    if c.is_null() {
        return;
    }

    let _ = unsafe { Box::<FFICow>::from_raw(c as *mut _) };
}
