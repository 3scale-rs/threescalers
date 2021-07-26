use std::prelude::v1::*;

use core::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int};

use super::c_slice::CSlice;
use super::ffi_cow::FFICow;

use crate::http::mapping_rule::{Method, RestRule};

#[no_mangle]
pub unsafe extern "C" fn rest_rule_new(
    method: *const c_char,
    path_n_qs: *const c_char,
) -> *const RestRule {
    let method_s = if let Some(c_slice) = CSlice::new(method, 0) {
        c_slice.as_cow()
    } else {
        return core::ptr::null();
    };
    let path_n_qs = if let Some(c_slice) = CSlice::new(path_n_qs, 0) {
        c_slice.as_cow()
    } else {
        return core::ptr::null();
    };

    let method = Method::from(method_s.as_ref());

    let rule = match RestRule::new(method, path_n_qs) {
        Ok(rr) => rr,
        Err(_) => return core::ptr::null(),
    };

    Box::into_raw(Box::new(rule)) as *const _
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_with_path_n_qs(
    method: *const c_char,
    path: *const c_char,
    qs: *const c_char,
) -> *const RestRule {
    let method_s = if let Some(c_slice) = CSlice::new(method, 0) {
        c_slice.as_cow()
    } else {
        return core::ptr::null();
    };
    let path = if let Some(c_slice) = CSlice::new(path, 0) {
        c_slice.as_cow()
    } else {
        return core::ptr::null();
    };

    let qs = if let Some(c_slice) = CSlice::new(qs, 0) {
        c_slice.as_cow().into()
    } else {
        None
    };

    let method = Method::from(method_s.as_ref());

    let rule = match RestRule::with_path_n_qs(method, path, qs) {
        Ok(rr) => rr,
        Err(_) => return core::ptr::null(),
    };

    Box::into_raw(Box::new(rule)) as *const _
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_free(rule: *const RestRule) {
    if rule.is_null() {
        return;
    }

    let _ = unsafe { Box::<RestRule>::from_raw(rule as *mut _) };
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_matches(
    rule: *const RestRule,
    method: *const c_char,
    path_qs: *const c_char,
) -> c_int {
    if rule.is_null() {
        return c_int::from(-1);
    }

    let method_s = if let Some(c_slice) = CSlice::new(method, 0) {
        c_slice.as_cow()
    } else {
        return c_int::from(-1);
    };
    let path_qs = if let Some(c_slice) = CSlice::new(path_qs, 0) {
        c_slice.as_cow()
    } else {
        return c_int::from(-1);
    };

    let rule = unsafe { std::ptr::read::<RestRule>(rule) };
    let rule = ManuallyDrop::new(rule);

    let method = Method::from(method_s.as_ref());

    if rule.matches(&method, path_qs) {
        c_int::from(1)
    } else {
        c_int::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_matches_path_n_qs(
    rule: *const RestRule,
    method: *const c_char,
    path: *const c_char,
    qs: *const c_char,
) -> c_int {
    if rule.is_null() {
        return c_int::from(-1);
    }

    let method_s = if let Some(c_slice) = CSlice::new(method, 0) {
        c_slice.as_cow()
    } else {
        return c_int::from(-1);
    };
    let path = if let Some(c_slice) = CSlice::new(path, 0) {
        c_slice.as_cow()
    } else {
        return c_int::from(-1);
    };
    let qs = if let Some(c_slice) = CSlice::new(qs, 0) {
        c_slice.as_cow().into()
    } else {
        None
    };

    let rule = unsafe { std::ptr::read::<RestRule>(rule) };
    let rule = ManuallyDrop::new(rule);

    let method = Method::from(method_s.as_ref());

    if &method == rule.method() && rule.matches_path_n_qs(path, qs) {
        c_int::from(1)
    } else {
        c_int::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_matches_request_line(
    rule: *const RestRule,
    http_request_line: *const c_char,
) -> c_int {
    if rule.is_null() {
        return c_int::from(-1);
    }

    let http_request_line = if let Some(c_slice) = CSlice::new(http_request_line, 0) {
        c_slice.as_cow()
    } else {
        return c_int::from(-1);
    };

    let rule = unsafe { std::ptr::read::<RestRule>(rule) };
    let rule = ManuallyDrop::new(rule);

    match rule.matches_request_line(http_request_line) {
        Ok(b) => {
            if b {
                c_int::from(1)
            } else {
                c_int::from(0)
            }
        }
        Err(_) => c_int::from(-1),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rest_rule_method(rule: *const RestRule) -> *const FFICow {
    if rule.is_null() {
        return core::ptr::null();
    }
    let rule = unsafe { std::ptr::read::<RestRule>(rule) };
    let rule = ManuallyDrop::new(rule);

    let method = FFICow::from(rule.method().as_str());

    Box::into_raw(Box::new(method)) as *const _
}
