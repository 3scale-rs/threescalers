use percent_encoding::{
    define_encode_set,
    utf8_percent_encode,
    PATH_SEGMENT_ENCODE_SET,
};
use std::borrow::Cow;

define_encode_set! {
    pub APISONATOR_EXTENSION_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | { ';', '&', '=', '[', ']' }
}

pub fn encode(s: &str) -> Cow<str> {
    utf8_percent_encode(s, APISONATOR_EXTENSION_ENCODE_SET).into()
}
