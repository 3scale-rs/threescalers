use std::prelude::v1::*;

use std::os::raw::c_char;

pub static VERSION: &str = concat!(crate::version!(), "\0");
pub static USER_AGENT: &str = concat!(crate::description!(), "/", crate::version!(), "\0");

#[no_mangle]
pub unsafe extern "C" fn version() -> *const c_char {
    VERSION.as_ptr() as *const _
}

#[no_mangle]
pub unsafe extern "C" fn user_agent() -> *const c_char {
    USER_AGENT.as_ptr() as *const _
}
