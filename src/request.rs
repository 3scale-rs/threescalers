#[derive(Debug)]
pub struct Request {
    method: String,
    path: String,
    body: Option<String>
}

pub trait ToParams {
    fn to_params(&self) -> String;
}

pub trait ToRequest {
    fn to_request(&self) -> Request;
}

impl Request {
    pub fn new(method: String, path: String, body: Option<String>) -> Request {
        Request { method, path, body }
    }
}