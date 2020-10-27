use std::prelude::v1::*;

use super::{
    Request,
    SetupRequest,
};

macro_rules! reqwest_impl {
    { $C:ty, $B:ty } => {
        // By default reqwest won't allow us to build a request without a base URI, ie:
        //
        // https://a_host
        //
        impl<URI: ToString> SetupRequest<'_, URI, Result<$B, Error>> for $C {
            fn setup_request(&mut self, r: Request, params: URI) -> Result<$B, Error> {
                use core::convert::TryInto;

                let (uri, body) = r.parameters.uri_and_body(r.path);
                let uri_base = params;
                let uri = uri_base.to_string() + uri.as_ref();

                let rb = self.request(r.method.into(), uri.as_str()).headers(r.headers.try_into()?);

                Ok(match body {
                    // when there is a body just consume it from the request's
                    // parameters to avoid cloning it unnecessarily.
                    Some(_) => rb.body(r.parameters.into_inner()),
                    _ => rb
                })
            }
        }
    }
}

#[cfg(feature = "reqwest-async")]
reqwest_impl!(reqwest::Client, reqwest::RequestBuilder);
#[cfg(feature = "reqwest-sync")]
reqwest_impl!(reqwest::blocking::Client, reqwest::blocking::RequestBuilder);
