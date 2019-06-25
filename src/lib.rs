#![warn(clippy::all)]
#![cfg_attr(feature = "nightly", feature(never_type))]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#[cfg(all(test, feature = "nightly"))]
extern crate test;

// Define a never type useful for some traits (ie. FromRequest)
#[cfg(feature = "nightly")]
pub type Never = !;
#[cfg(not(feature = "nightly"))]
pub type Never = core::convert::Infallible;

pub mod api_call;
pub mod application;
pub mod credentials;
pub mod encoding;
pub mod errors;
pub mod extensions;
pub mod http;
pub mod service;
pub mod transaction;
pub mod usage;
pub mod user;
pub mod version;

pub mod timestamp;
#[cfg(feature = "xml-response")]
pub mod response;


use std::borrow::Cow;

// This is the trait to be implemented by structures that can set parameters to API calls.
//
// Note that the 'this lifetime requires a lifetime long enough to keep keys and values.
// The keys can potentially be modified to adapt them to the specific call circumstances,
// so in order to take advantage of single allocations when modifications are not needed,
// a copy-on-write type is used. Values are always kept as is, so just references are ok.
pub trait ToParams<'k, 'v, 'this, E>
where
    'this: 'k + 'v,
    E: Extend<(Cow<'k, str>, &'v str)>,
{
    fn to_params(&'this self, extendable: &mut E) {
        self.to_params_with_prefix(extendable, None);
    }

    fn to_params_with_prefix(&'this self, extendable: &mut E, prefix: Option<&'k str>) {
        self.to_params_with_mangling(extendable, &mut |c| match prefix {
            Some(p) => c + p,
            _ => c,
        });
    }

    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(
        &'this self,
        extendable: &mut E,
        key_mangling: &mut F,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
