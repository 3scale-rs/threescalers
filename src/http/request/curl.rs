use std::prelude::v1::*;

use curl::easy::List;

use super::HeaderMap;
use crate::{
    anyhow,
    Error,
};
use core::convert::TryFrom;

impl TryFrom<&HeaderMap> for List {
    type Error = Error;

    fn try_from(hm: &HeaderMap) -> Result<Self, Self::Error> {
        let mut list = List::new();

        for (k, v) in hm.iter() {
            let header = [k.as_str(), ": ", v.as_str()].concat();
            list.append(header.as_str())
                .map_err(|e| anyhow!("failed to add a node to a curl List: {:#?}", e))?;
        }

        Ok(list)
    }
}

// Common functions for curl clients
pub fn copy_data(offset: &mut usize, source: &[u8], dst: &mut [u8]) -> usize {
    let bytes = &source[*offset..];
    let len = bytes.len();
    dst[..len].copy_from_slice(bytes);
    *offset += len;
    len
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
