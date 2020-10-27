use std::prelude::v1::*;

use curl::easy::List;

use super::HeaderMap;
use core::convert::TryFrom;

impl TryFrom<&HeaderMap> for List {
    type Error = Box<dyn std::error::Error>;

    fn try_from(hm: &HeaderMap) -> Result<Self, Self::Error> {
        let mut list = List::new();

        for (k, v) in hm.iter() {
            let header = [k.as_str(), ": ", v.as_str()].concat();
            list.append(header.as_str())?;
        }

        Ok(list)
    }
}

// Common functions for curl clients
pub fn copy_data(offset: &mut usize, source: &[u8], dst: &mut [u8]) -> usize {
    use std::io::Read;

    let mut bytes = &source[*offset..];
    let newcount = bytes.read(dst).expect("error while copying body data to buffer");
    *offset += newcount;
    newcount
}

#[cfg(feature = "curl-easy")]
mod easy;
#[cfg(feature = "curl-easy2")]
mod easy2;

#[cfg(feature = "curl-easy")]
pub use easy::CurlEasyClient;
#[cfg(feature = "curl-easy2")]
pub use easy2::{
    BodyHandle,
    SetBody,
};
