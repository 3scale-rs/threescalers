use curl::easy::List;
use http_types::header::HeaderValue;

// Common functions for curl clients
fn headermap_to_curl_list(headermap: &http_types::HeaderMap<HeaderValue>) -> List {
    let mut list = List::new();
    headermap.iter().for_each(|(k, v)| {
                        // this will scan for printable US-ASCII only bytes
                        let header = v.to_str()
                                      .map(|hval| [k.as_str(), ": ", hval].concat())
                                      .expect("found header value without a displayable US-ASCII string");
                        list.append(header.as_str())
                            .expect("failed to allocate node for curl list of headers");
                    });
    list
}

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
