use std::prelude::v1::*;

mod ffi_cow;
pub use ffi_cow::fficow_free;
pub use ffi_cow::{FFICow, FFIStr, FFIString};

mod c_slice;
pub use c_slice::{CSlice, CSliceMut};

pub mod encoding;
pub mod version;

#[cfg(feature = "rest-mappings")]
pub mod mapping_rule;
