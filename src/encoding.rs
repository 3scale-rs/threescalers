use percent_encoding::{
    utf8_percent_encode,
    AsciiSet,
    CONTROLS,
};
use std::borrow::Cow;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const DEFAULT_ENCODE_SET: &AsciiSet = &QUERY_ENCODE_SET.add(b'`').add(b'?').add(b'{').add(b'}');
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &DEFAULT_ENCODE_SET.add(b'%').add(b'/');
const APISONATOR_EXTENSION_ENCODE_SET: &AsciiSet = &PATH_SEGMENT_ENCODE_SET.add(b';')
                                                                           .add(b'&')
                                                                           .add(b'=')
                                                                           .add(b'[')
                                                                           .add(b']');

pub fn encode(s: &str) -> Cow<str> {
    utf8_percent_encode(s, APISONATOR_EXTENSION_ENCODE_SET).into()
}
