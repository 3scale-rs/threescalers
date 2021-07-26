use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::borrow::Cow;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const DEFAULT_ENCODE_SET: &AsciiSet = &QUERY_ENCODE_SET.add(b'`').add(b'?').add(b'{').add(b'}');
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &DEFAULT_ENCODE_SET.add(b'%').add(b'/');
const APISONATOR_EXTENSION_ENCODE_SET: &AsciiSet = &PATH_SEGMENT_ENCODE_SET
    .add(b';')
    .add(b'&')
    .add(b'=')
    .add(b'[')
    .add(b']');

pub fn encode<S: AsRef<[u8]> + ?Sized>(s: &S) -> Cow<str> {
    percent_encode(s.as_ref(), APISONATOR_EXTENSION_ENCODE_SET).into()
}
