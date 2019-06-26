use crate::api_call::{Kind::*, *};
use crate::application::*;
use crate::user::*;
use crate::version::USER_AGENT;
use crate::ToParams;

use super::Parameters;

#[cfg(feature = "http-types")]
mod http;
#[cfg(feature = "reqwest-types")]
mod reqwest;

#[derive(Clone, Debug)]
pub struct Request {
    pub method: http_types::Method,
    pub path: &'static str,
    pub parameters: Parameters,
    pub headers: http_types::HeaderMap,
}

use http_types::Method;
use std::borrow::Cow;

impl Request {
    pub fn endpoint(
        kind: Kind,
        application: Option<&Application>,
        user: Option<&User>,
    ) -> (Method, &'static str) {
        use super::endpoints::*;

        match (kind, application, user) {
            (Authorize, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, _) => AUTHORIZE_ENDPOINT,
            (AuthRep, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, _) => AUTHREP_ENDPOINT,
            (Report, _, _) => REPORT_ENDPOINT,
        }
    }

    pub fn uri_and_body(&self) -> (Cow<str>, Option<&str>) {
        (
            self.parameters.path_and_query(self.path),
            self.parameters.body(),
        )
    }
}

pub trait FromRequest<P> {
    fn from_request(r: Request, params: P) -> Self;
}

impl From<&ApiCall<'_, '_, '_, '_, '_, '_>> for Request {
    fn from(apicall: &ApiCall) -> Self {
        use http_types::header::{HeaderName, HeaderValue};
        use http_types::HeaderMap;

        let (method, path) =
            Request::endpoint(apicall.kind(), apicall.application(), apicall.user());

        let mut params = Vec::with_capacity(8);
        apicall.to_params(&mut params);

        let parameters = Parameters::new(&method, params.as_slice());

        let mut headers = apicall.extensions().map_or_else(
            || HeaderMap::with_capacity(1),
            |e| {
                let options = e.to_string();
                let mut h = HeaderMap::with_capacity(2);
                let val = HeaderValue::from_str(options.as_str());

                if let Ok(val) = val {
                    h.insert(HeaderName::from_static("3scale-options"), val);
                }

                h
            },
        );

        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

        Request {
            method,
            path,
            parameters,
            headers,
        }
    }
}
