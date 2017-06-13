pub trait ToParams {
    fn to_params(&self) -> Vec<(&str, &str)>;
}

pub trait ToRequest {
    fn to_request(&self) -> Request;
}

#[derive(Debug)]
pub struct Request<'m, 'ep, 'key, 'value> {
    method: &'m str,
    endpoint: &'ep str,
    params: Vec<(&'key str, &'value str)>,
    body: Option<String>
}

impl<'m, 'ep, 'key, 'value> Request<'m, 'ep, 'key, 'value> {
    pub fn new(method: &'m str, endpoint: &'ep str, params: Vec<(&'key str, &'value str)>, body: Option<String>) -> Request<'m, 'ep, 'key, 'value> {
        Request { method, endpoint, params, body }
    }

    pub fn method(&self) -> &str {
        self.method
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint
    }

    pub fn params(&self) -> &Vec<(&str, &str)> {
        self.params.as_ref()
    }

    pub fn url_params(&self) -> String {
        self.params.iter()
            .map(|&(param, value)| param.to_owned() + "=" + value)
            .collect::<Vec<String>>()
            .join("&")
    }

    pub fn path(&self) -> String {
        self.endpoint.to_owned() + "?" + self.url_params().as_str()
    }

    pub fn body(&self) -> Option<&str> {
        match self.body {
            Some(ref body) => Some(body.as_str()),
            _ => None
        }
    }
}
