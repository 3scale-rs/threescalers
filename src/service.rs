use request::ToParams;

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
    fn to_params(&self) -> String {
        let (field, value) = match *self {
            Credentials::ProviderKey(ref key) => ("provider_key=", key),
            Credentials::ServiceToken(ref token) => ("service_token=", token)
        };
        let mut param = field.to_owned();
        param.push_str(value.as_str());
        param
    }
}

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
    fn to_params(&self) -> String {
        let mut param = "service_id=".to_owned();
        param.push_str(self.service_id.as_str());
        param.push_str(self.creds.to_params().as_str());
        param
    }
}