use super::Request;
use crate::api_call::ApiCall;
use crate::version::*;
use http_types::Request as HTTPRequest;

impl From<Request> for HTTPRequest<String> {
    fn from(r: Request) -> Self {
        let (uri, body) = r.parameters.uri_and_body(r.path);
        let body = body.unwrap_or("").to_owned();
        let mut rb = HTTPRequest::builder();

        rb.header("User-Agent", USER_AGENT)
            .method(r.method)
            .uri(uri.as_ref())
            .body(body)
            .unwrap()
    }
}

impl From<&ApiCall<'_, '_, '_, '_, '_, '_>> for HTTPRequest<String> {
    fn from(i: &ApiCall) -> Self {
        Request::from(i).into()
    }
}

use super::FromRequest;

impl FromRequest<!> for HTTPRequest<String> {
    fn from_request(r: Request, _params: !) -> Self {
        Self::from(r)
    }
}
