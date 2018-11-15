use crate::request::ToParams;
use crate::errors::*;

use std::str::FromStr;

#[derive(Debug)]
pub struct ProviderKey(String);
#[derive(Debug)]
pub struct ServiceToken(String);

#[derive(Debug)]
pub enum Credentials {
    ProviderKey(ProviderKey),
    ServiceToken(ServiceToken),
}

// These trait impls provide a way to reference our types as &str
impl AsRef<str> for ProviderKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ServiceToken {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// These trait impls provide a way to &str#parse()
impl FromStr for ProviderKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<ProviderKey> {
        Ok(ProviderKey(s.into()))
    }
}

impl FromStr for ServiceToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<ServiceToken> {
        Ok(ServiceToken(s.into()))
    }
}

// These trait impls are similar to FromStr (but are infallible)
impl<'a> From<&'a str> for ProviderKey where Self: FromStr {
    fn from(s: &'a str) -> ProviderKey {
        s.parse().unwrap()
    }
}

impl<'a> From<&'a str> for ServiceToken where Self: FromStr {
    fn from(s: &'a str) -> ServiceToken {
        s.parse().unwrap()
    }
}

// These trait impls take ownership of a given String
impl From<String> for ProviderKey {
    fn from(s: String) -> ProviderKey {
        ProviderKey(s)
    }
}

impl From<String> for ServiceToken {
    fn from(s: String) -> ServiceToken {
        ServiceToken(s)
    }
}

impl Into<Credentials> for ProviderKey {
    fn into(self) -> Credentials {
        Credentials::ProviderKey(self)
    }
}

impl Into<Credentials> for ServiceToken {
    fn into(self) -> Credentials {
        Credentials::ServiceToken(self)
    }
}

impl Credentials {
    /// Creates `Credentials` from a `ProviderKey`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::service::*;
    ///
    /// let creds = Credentials::from_key("my_key");
    /// ```
    pub fn from_key<T: Into<ProviderKey>>(key: T) -> Self {
        Credentials::ProviderKey(key.into())
    }

    /// Creates `Credentials` from a `ServiceToken`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::service::*;
    ///
    /// let creds = Credentials::from_token("my_token");
    /// ```
    pub fn from_token<T: Into<ServiceToken>>(token: T) -> Self {
        Credentials::ServiceToken(token.into())
    }
}

impl ToParams for Credentials {
    fn to_params(&self) -> Vec<(&str, &str)> {
        use self::Credentials::*;

        let (field, value) = match *self {
            ProviderKey(ref key) => ("provider_key", key.as_ref()),
            ServiceToken(ref token) => ("service_token", token.as_ref())
        };

        vec![(field, value)]
    }
}

#[derive(Debug)]
pub struct ServiceId(String);

#[derive(Debug)]
pub struct Service {
    service_id: ServiceId,
    creds: Credentials,
}

impl AsRef<str> for ServiceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for ServiceId {
    type Err = Error;

    fn from_str(s: &str) -> Result<ServiceId> {
        Ok(ServiceId(s.into()))
    }
}

impl<'a> From<&'a str> for ServiceId where Self: FromStr {
    fn from(s: &'a str) -> ServiceId {
        s.parse().unwrap()
    }
}

impl From<String> for ServiceId {
    fn from(s: String) -> ServiceId {
        ServiceId(s)
    }
}

impl Service {
    /// Creates a new `Service` from a `ServiceId` and `Credentials`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::service::*;
    ///
    /// let creds = Credentials::from_token("my_token");
    /// let service = Service::new("my_service_id", creds);
    /// ```
    pub fn new<T: Into<ServiceId>>(service_id: T, creds: Credentials) -> Self {
        Self { service_id: service_id.into(), creds }
    }
}

impl ToParams for Service {
    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut res = vec![("service_id", self.service_id.as_ref())];
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
        let creds = Credentials::from_key(provider_key);
        let service = Service::new(service_id, creds);

        let result = service.to_params();

        let expected = vec![("service_id", service_id),
                            ("provider_key", provider_key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_service_id_and_token_into_params() {
        let service_id = "my_service_id";
        let token = "my_token";
        let creds = Credentials::from_token(token);
        let service = Service::new(service_id, creds);

        let result = service.to_params();

        let expected = vec![("service_id", service_id),
                            ("service_token", token)];
        assert_eq!(expected, result);
    }
}
