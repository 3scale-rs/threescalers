#[derive(Debug)]
pub struct Request {
    method: String,
    path: String,
    body: Option<String>
}

pub trait ToParams {
    fn to_params(&self) -> Vec<(&str, &str)>;
}

pub trait ToRequest {
    fn to_request(&self) -> Request;
}

impl Request {
    pub fn new(method: String, path: String, body: Option<String>) -> Request {
        Request { method, path, body }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn body(&self) -> Option<&str> {
        match self.body {
            Some(ref body) => Some(body.as_str()),
            _ => None
        }
    }
}
