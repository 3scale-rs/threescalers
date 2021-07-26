use std::prelude::v1::*;

use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use std::borrow::Cow;

pub struct CSlice<'a>(&'a [u8]);

impl<'a> CSlice<'a> {
    pub fn new(buf: *const c_char, len: usize) -> Option<Self> {
        if len == 0 {
            Self(parse_c_str(buf)?)
        } else {
            Self(parse_buffer(buf as *const _, len)?)
        }
        .into()
    }

    pub fn new_from_size_ptr(buf: *const c_char, len_ptr: *const usize) -> Option<Self> {
        if len_ptr.is_null() {
            return None;
        }

        Self::new(buf, unsafe { *len_ptr })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn into_bytes(self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn into_cow(self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }

    #[inline]
    pub fn as_cow(&self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }
}

pub struct CSliceMut<'a>(&'a mut [u8]);

impl<'a> CSliceMut<'a> {
    pub fn new(buf: *mut c_char, len: usize) -> Option<Self> {
        if len == 0 {
            Self(parse_c_str_mut(buf)?)
        } else {
            Self(parse_buffer_mut(buf as *mut _, len)?)
        }
        .into()
    }

    pub fn new_from_size_ptr(buf: *mut c_char, len_ptr: *const usize) -> Option<Self> {
        if len_ptr.is_null() {
            return None;
        }

        Self::new(buf, unsafe { *len_ptr })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn into_inner(self) -> &'a mut [u8] {
        self.0
    }

    #[inline]
    pub fn into_bytes(self) -> &'a mut [u8] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&'a self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn as_bytes_mut(&'a mut self) -> &'a mut [u8] {
        self.0
    }

    #[inline]
    pub fn into_cow(self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }

    #[inline]
    pub fn as_cow(&'a self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }
}

impl<'a> AsRef<[u8]> for CSlice<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> AsRef<[u8]> for CSliceMut<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

fn parse_buffer(buf: *const c_void, len: usize) -> Option<&'static [u8]> {
    if buf.is_null() {
        return None;
    }

    unsafe { std::slice::from_raw_parts(buf as *const _, len) }.into()
}

fn parse_buffer_mut(buf: *mut c_void, len: usize) -> Option<&'static mut [u8]> {
    if buf.is_null() {
        return None;
    }

    unsafe { std::slice::from_raw_parts_mut(buf as *mut _, len) }.into()
}

fn parse_c_str(s: *const c_char) -> Option<&'static [u8]> {
    if s.is_null() {
        return None;
    }

    unsafe { CStr::from_ptr(s) }.to_bytes().into()
}

fn parse_c_str_mut(s: *mut c_char) -> Option<&'static mut [u8]> {
    if s.is_null() {
        return None;
    }

    let len = unsafe { CStr::from_ptr(s) }.to_bytes().len();
    unsafe { std::slice::from_raw_parts_mut(s as *mut _, len) }.into()
}
