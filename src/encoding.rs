use std::borrow::Cow;
use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, define_encode_set};

define_encode_set! {
    pub APISONATOR_EXTENSION_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | { ';', '&', '=', '[', ']' }
}

pub fn encode(s: &str) -> Cow<str> {
    utf8_percent_encode(s, APISONATOR_EXTENSION_ENCODE_SET).into()
}
