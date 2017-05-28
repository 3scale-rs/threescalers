use request::ToParams;

#[derive(Debug)]
pub enum Credentials {
    ProviderKey(String),
    ServiceToken(String),
}

impl Credentials {
    pub fn from_key(key: String) -> Self {
        Credentials::ProviderKey(key)
    }

    pub fn from_token(token: String) -> Self {
        Credentials::ServiceToken(token)
    }
}

impl ToParams for Credentials {
    fn to_params(&self) -> Vec<(&str, &str)> {
        use self::Credentials::*;

        let (field, value) = match *self {
            ProviderKey(ref key) => ("provider_key", key),
            ServiceToken(ref token) => ("service_token", token)
        };

        vec![(field, value)]
    }
}

#[derive(Debug)]
pub struct Service {
    service_id: String,
    creds: Credentials,
}

impl Service {
    pub fn new(service_id: String, creds: Credentials) -> Self {
        Self { service_id, creds }
    }
}

impl ToParams for Service {
    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut res = vec![("service_id", self.service_id.as_str())];
        let creds = self.creds.to_params();
        res.extend(creds);
        res
    }
}
