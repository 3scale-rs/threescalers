#![warn(clippy::all)]
#![cfg_attr(feature_gate_never_type, feature(never_type))]
#![cfg_attr(feature_gate_const_saturating_int_methods,
            feature(const_saturating_int_methods))]
#![cfg_attr(feature_gate_test, feature(test))]
#![no_std]
extern crate no_std_compat as std;
use std::prelude::v1::*;

#[cfg(all(test, has_test))]
extern crate test;

// Define a never type useful for some traits (ie. SetupRequest)
#[cfg(feature_gate_never_type)]
pub type Never = !;
#[cfg(not(has_never_type))]
pub type Never = core::convert::Infallible;

pub mod api_call;
pub mod application;
pub mod credentials;
pub mod encoding;
pub mod extensions;
pub mod http;
pub mod service;
pub mod transaction;
pub mod usage;
pub mod user;
pub mod version;

#[cfg(feature = "xml-response")]
pub mod response;

pub(crate) mod error {
    pub use anyhow::{
        anyhow,
        Error,
        Result,
    };
}

pub use error::Error;

#[allow(unused_imports)]
pub(crate) use error::anyhow;

use std::borrow::Cow;

/// This is the trait to be implemented by structures that can set parameters to API calls.
///
/// Note that the 'this lifetime requires a lifetime long enough to keep keys and values.
/// The keys can potentially be modified to adapt them to the specific call circumstances,
/// so in order to take advantage of single allocations when modifications are not needed,
/// a copy-on-write type is used. Values are always kept as is, so just references are ok.
pub(crate) trait ToParams<'k, 'v, 'this, E>
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)>
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

    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self,
                                                                       extendable: &mut E,
                                                                       key_mangling: &mut F);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
