use std::prelude::v1::*;

use std::borrow::Cow;

/// Represents an extension parameter for 3scale API calls.
///
/// Extensions modify the behavior or output of API calls. Each variant represents a different
/// type of extension that can be added to an API request.
///
/// # Variants
///
/// - `FlatUsage`: Changes usage reporting format to a flat structure
/// - `Hierarchy`: Requests the metrics hierarchy in the response
/// - `NoBody`: Requests that the response body be omitted
/// - `ListAppKeys`: Requests the list of application keys for an app
/// - `Other`: Any custom extension with arbitrary key-value pairs
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Extension<'s> {
    FlatUsage(Cow<'s, str>),
    Hierarchy,
    NoBody,
    ListAppKeys(Cow<'s, str>),
    Other(Cow<'s, str>, Cow<'s, str>),
}

impl Extension<'_> {
    /// Returns the key name for this extension.
    ///
    /// The key is used when serializing the extension as a parameter.
    pub fn key(&self) -> &'_ str {
        match self {
            Extension::Other(k, _) => k,
            Extension::FlatUsage(..) => "flat_usage",
            Extension::Hierarchy => "hierarchy",
            Extension::ListAppKeys(..) => "list_app_keys",
            Extension::NoBody => "no_body",
        }
    }

    /// Returns the value for this extension.
    ///
    /// For known extensions, this returns the configured value.
    /// For extensions without values, this returns "1".
    pub fn value(&self) -> &'_ str {
        match self {
            Extension::Other(_, v) | Extension::FlatUsage(v) | Extension::ListAppKeys(v) => v,
            Extension::Hierarchy | Extension::NoBody => "1",
        }
    }

    /// Returns this extension as a URL-encoded key-value pair.
    ///
    /// For known extensions, the key is not encoded. For custom extensions,
    /// both key and value are URL-encoded.
    pub fn to_cow(&self) -> Cow<'_, str> {
        use crate::encoding::encode;

        // This avoids encoding known extensions by issuing the final "encoded" form.
        match self {
            Extension::Other(k, v) => encode(k) + "=" + encode(v),
            Extension::FlatUsage(value) => Cow::from("flat_usage=") + value.as_ref(),
            Extension::Hierarchy => "hierarchy=1".into(),
            Extension::ListAppKeys(value) => Cow::from("list_app_keys=") + value.as_ref(),
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

        [encode(self.key()), "=".into(), encode(self.value())].concat()
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
        assert_eq!(
            Extension::NoBody.to_cow(),
            Extension::NoBody.to_encoded_string()
        );
        assert_eq!(
            Extension::Hierarchy.to_string(),
            Extension::Hierarchy.to_encoded_string()
        );
        assert_eq!(
            Extension::FlatUsage(1.to_string().into()).to_string(),
            Extension::FlatUsage(1.to_string().into()).to_encoded_string()
        );
        assert_eq!(
            Extension::ListAppKeys(1.to_string().into()).to_string(),
            Extension::ListAppKeys(1.to_string().into()).to_encoded_string()
        );
        let ext = Extension::Other("some;[]key&%1".into(), "a_^&[]%:;@value".into());
        assert_eq!(ext.to_string(), ext.to_encoded_string());
    }
}
