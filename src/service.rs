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

#[cfg(test)]
mod credentials_tests {
    use super::*;

    #[test]
    fn transforms_service_id_and_key_into_params() {
        let service_id = "my_service_id";
        let provider_key = "my_provider_key";
        let creds = Credentials::from_key(provider_key.to_owned());
        let service = Service::new(service_id.to_owned(), creds);

        let result = service.to_params();

        let expected = vec![("service_id", service_id),
                            ("provider_key", provider_key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_service_id_and_token_into_params() {
        let service_id = "my_service_id";
        let token = "my_token";
        let creds = Credentials::from_token(token.to_owned());
        let service = Service::new(service_id.to_owned(), creds);

        let result = service.to_params();

        let expected = vec![("service_id", service_id),
                            ("service_token", token)];
        assert_eq!(expected, result);
    }
}
