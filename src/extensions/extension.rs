use std::prelude::v1::*;

use std::borrow::Cow;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Extension<'s> {
    FlatUsage(Cow<'s, str>),
    Hierarchy,
    NoBody,
    Other(Cow<'s, str>, Cow<'s, str>),
}

impl Extension<'_> {
    pub fn key(&self) -> &'_ str {
        match self {
            Extension::Other(k, _) => k,
            Extension::FlatUsage(..) => "flat_usage",
            Extension::Hierarchy => "hierarchy",
            Extension::NoBody => "no_body",
        }
    }

    pub fn value(&self) -> &'_ str {
        match self {
            Extension::Other(_, v) | Extension::FlatUsage(v) => v,
            Extension::Hierarchy | Extension::NoBody => "1",
        }
    }

    pub fn to_cow(&self) -> Cow<'_, str> {
        use crate::encoding::encode;

        // This avoids encoding known extensions by issuing the final "encoded" form.
        match self {
            Extension::Other(k, v) => encode(k) + "=" + encode(v),
            Extension::FlatUsage(value) => Cow::from("flat_usage=") + value.as_ref(),
            Extension::Hierarchy => "hierarchy=1".into(),
            Extension::NoBody => "no_body=1".into(),
        }
    }
}

#[cfg(test)]
// Place here methods which are only useful for tests
// We need this to ensure we output the right format when "taking shortcuts".
// Ideally we'd be able to ensure this at compile time via const fns.
impl Extension<'_> {
    pub fn to_encoded_string(&self) -> String {
        use crate::encoding::encode;

        [encode(self.key()), "=".into(), encode(self.value().as_ref())].concat()
    }
}

impl ToString for Extension<'_> {
    fn to_string(&self) -> String {
        self.to_cow().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hardcoded known extensions could contain typos and not properly encoded characters, so test them.
    // Perhaps at some point this could be statically guaranteed.
    #[test]
    fn test_to_cow_is_well_encoded() {
        assert_eq!(Extension::NoBody.to_cow(), Extension::NoBody.to_encoded_string());
        assert_eq!(Extension::Hierarchy.to_string(),
                   Extension::Hierarchy.to_encoded_string());
        assert_eq!(Extension::FlatUsage(1.to_string().into()).to_string(),
                   Extension::FlatUsage(1.to_string().into()).to_encoded_string());
        let ext = Extension::Other("some;[]key&%1".into(), "a_^&[]%:;@value".into());
        assert_eq!(ext.to_string(), ext.to_encoded_string());
    }
}
