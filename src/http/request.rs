use crate::{
    api_call::{
        Kind::*,
        *,
    },
    application::*,
    user::*,
    version::USER_AGENT,
    ToParams,
};

use super::Parameters;

#[cfg(any(feature = "curl-easy", feature = "curl-easy2"))]
pub mod curl;
#[cfg(feature = "http-types")]
mod http_types;
#[cfg(any(feature = "reqwest-sync", feature = "reqwest-async"))]
mod reqwest;

pub use super::{
    HeaderMap,
    Method,
};

#[derive(Clone, Debug)]
pub struct Request {
    pub method:     Method,
    pub path:       &'static str,
    pub parameters: Parameters,
    pub headers:    HeaderMap,
}

use std::borrow::Cow;

impl Request {
    pub fn endpoint(kind: Kind,
                    application: Option<&Application>,
                    user: Option<&User>)
                    -> (Method, &'static str) {
        use super::endpoints::*;

        match (kind, application, user) {
            (Authorize, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, ..) => AUTHORIZE_ENDPOINT,
            (AuthRep, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, ..) => AUTHREP_ENDPOINT,
            (Report, ..) => REPORT_ENDPOINT,
        }
    }

    pub fn uri_and_body(&self) -> (Cow<str>, Option<&str>) {
        (self.parameters.path_and_query(self.path), self.parameters.body())
    }
}

/// This trait needs to be implemented by each client to set up a specific request.
///
/// The 'client lifetime will be useful if your Output return value needs to get hold of it. Such
/// is the case of curl's Easy client when sending POST requests via their Transfer<'client, 'data>
/// type, but for other clients which don't need to wrap the original client it's simply elided.
pub trait SetupRequest<'client, P, Output> {
    fn setup_request(&'client mut self, r: Request, params: P) -> Output;
}

impl From<&ApiCall<'_, '_, '_, '_, '_, '_>> for Request {
    fn from(apicall: &ApiCall) -> Self {
        let (method, path) = Request::endpoint(apicall.kind(), apicall.application(), apicall.user());

        let mut params = Vec::with_capacity(8);
        apicall.to_params(&mut params);

        let parameters = Parameters::new(&method, params.as_slice());

        let mut headers = apicall.extensions().map_or_else(|| HeaderMap::with_capacity(1),
                                                           |e| {
                                                               let mut hm = HeaderMap::with_capacity(2);
                                                               let _ = hm.insert("3scale-options".to_owned(),
                                                                                 e.to_string());
                                                               hm
                                                           });

        headers.insert("User-Agent".to_owned(), USER_AGENT.to_owned());

        Request { method,
                  path,
                  parameters,
                  headers }
    }
}
