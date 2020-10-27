use super::{
    HeaderMap,
    Method,
    Request,
};
use crate::{
    api_call::ApiCall,
    version::*,
    Error,
};
use core::convert::TryFrom;
use http_types::{
    header::{
        HeaderName,
        HeaderValue,
    },
    request::Builder,
    HeaderMap as HTTPHeaderMap,
    Method as HTTPMethod,
    Request as HTTPRequest,
};

impl From<Method> for HTTPMethod {
    fn from(m: Method) -> Self {
        use Method::*;

        match m {
            GET => Self::GET,
            POST => Self::POST,
            PUT => Self::PUT,
            DELETE => Self::DELETE,
            PATCH => Self::PATCH,
            HEAD => Self::HEAD,
        }
    }
}

// internal - probably best to use some combination of iterators using Extend?
trait FillFrom {
    type Error;

    fn fill_from(&mut self, hm: &HeaderMap) -> Result<(), Self::Error>;
}

impl FillFrom for HTTPHeaderMap {
    type Error = Error;

    fn fill_from(&mut self, hm: &HeaderMap) -> Result<(), Self::Error> {
        use core::str::FromStr;

        let it = hm.iter();
        for (key, value) in it {
            let key = HeaderName::from_str(key.as_str())?;
            let value = HeaderValue::try_from(value)?;
            self.append(key, value);
        }

        Ok(())
    }
}

impl TryFrom<HeaderMap> for HTTPHeaderMap {
    type Error = Error;

    fn try_from(hm: HeaderMap) -> Result<Self, Self::Error> {
        let mut map = HTTPHeaderMap::with_capacity(hm.len());

        map.fill_from(&hm)?;

        Ok(map)
    }
}

impl TryFrom<Request> for HTTPRequest<String> {
    type Error = Error;

    fn try_from(r: Request) -> Result<Self, Self::Error> {
        let (uri, body) = r.parameters.uri_and_body(r.path);
        let body = body.unwrap_or("").to_owned();
        let rb = HTTPRequest::builder();

        let mut rb = rb.header("User-Agent", USER_AGENT)
                       .method(HTTPMethod::from(r.method))
                       .uri(uri.as_ref());

        let map = rb.headers_mut().unwrap();

        map.fill_from(&r.headers)?;

        Ok(rb.body(body)?)
    }
}

impl TryFrom<&ApiCall<'_, '_, '_, '_, '_, '_>> for HTTPRequest<String> {
    type Error = Error;

    fn try_from(i: &ApiCall) -> Result<Self, Self::Error> {
        HTTPRequest::try_from(Request::from(i))
    }
}

use super::SetupRequest;
use crate::Never;

impl SetupRequest<'_, Never, HTTPRequest<String>> for Builder {
    fn setup_request(&mut self, r: Request, _params: Never) -> HTTPRequest<String> {
        HTTPRequest::try_from(r).unwrap()
    }
}
