#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

pub mod version;
pub mod errors;
pub mod credentials;
pub mod service;
pub mod transaction;
pub mod application;
pub mod user;
pub mod usage;
pub mod extensions;
pub mod api_call;
pub mod encoding;
pub mod http;

pub mod timestamp;

use std::borrow::Cow;

// This is the trait to be implemented by structures that can set parameters to API calls.
//
// Note that the 'this lifetime requires a lifetime long enough to keep keys and values.
// The keys can potentially be modified to adapt them to the specific call circumstances,
// so in order to take advantage of single allocations when modifications are not needed,
// a copy-on-write type is used. Values are always kept as is, so just references are ok.
pub trait ToParams<'k, 'v, 'this, E> where 'this: 'k + 'v, E: Extend<(Cow<'k, str>, &'v str)> {
    fn to_params(&'this self, extendable: &mut E) {
        self.to_params_with_prefix(extendable, None);
    }

    fn to_params_with_prefix(&'this self, extendable: &mut E, prefix: Option<&'k str>) {
        self.to_params_with_mangling(extendable, &mut |c| {
            match prefix {
                Some(p) => c + p,
                _ => c,
            }
        });
    }

    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self, extendable: &mut E, key_mangling: &mut F);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
