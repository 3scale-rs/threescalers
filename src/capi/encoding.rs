// MSRV 1.55+: move to refutable let bindings: let Some(c_slice) = ... else { return null(); };
use std::prelude::v1::*;

use std::os::raw::{c_char, c_int};

use super::c_slice::{CSlice, CSliceMut};
use super::ffi_cow::FFICow;

use crate::encoding;

#[no_mangle]
pub unsafe extern "C" fn encoding_encode(s: *const c_char, len: usize) -> *const FFICow {
    let c_slice = if let Some(c_slice) = CSlice::new(s, len) {
        c_slice
    } else {
        return core::ptr::null();
    };

    let ffi_cow = FFICow::from(encoding::encode(&c_slice));

    Box::into_raw(Box::new(ffi_cow)) as *const _
}

#[no_mangle]
pub unsafe extern "C" fn encoding_encode_buffer(
    src: *const c_char,
    srclen: usize,
    dst: *mut c_char,
    dstlen_ptr: *mut usize,
) -> c_int {
    let c_slice_dst = if let Some(c_slice_mut) = CSliceMut::new_from_size_ptr(dst, dstlen_ptr) {
        c_slice_mut
    } else {
        return -1;
    };

    let c_slice_src = if let Some(c_slice) = CSlice::new(src, srclen) {
        c_slice
    } else {
        return -1;
    };

    let cow = encoding::encode(&c_slice_src);

    let required_len = cow.len();
    unsafe { *dstlen_ptr = required_len };

    if required_len > c_slice_dst.len() {
        return -1;
    }

    unsafe {
        // Note: we are not NUL-terminating the buffer
        // Even if it is a string, that's for the other side to decide on, as length is known
        core::ptr::copy_nonoverlapping(cow.as_ptr(), dst as *mut _, required_len);
    }

    0
}
