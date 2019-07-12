use super::{
    FromRequest,
    Request,
};

macro_rules! reqwest_impl {
    { $C:ty, $B:ty } => {
        // By default reqwest won't allow us to build a request without a base URI, ie:
        //
        // https://a_host
        //
        impl<URI: ToString> FromRequest<(&$C, URI)> for $B {
            fn from_request(r: Request, params: (&$C, URI)) -> Self {
                let (uri, body) = r.parameters.uri_and_body(r.path);
                let (client, uri_base) = params;
                let uri = uri_base.to_string() + uri.as_ref();

                let rb = client.request(r.method, uri.as_str()).headers(r.headers);

                match body {
                    // when there is a body just consume it from the request's
                    // parameters to avoid cloning it unnecessarily.
                    Some(_) => rb.body(r.parameters.into_inner()),
                    _ => rb
                }
            }
        }
    }
}

#[cfg(feature = "reqwest-async")]
reqwest_impl!(reqwest::r#async::Client, reqwest::r#async::RequestBuilder);
#[cfg(feature = "reqwest-sync")]
reqwest_impl!(reqwest::Client, reqwest::RequestBuilder);
