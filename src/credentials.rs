use crate::ToParams;
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
impl From<&str> for ProviderKey where Self: FromStr {
    fn from(s: &str) -> ProviderKey {
        s.parse().unwrap()
    }
}

impl From<&str> for ServiceToken where Self: FromStr {
    fn from(s: &str) -> ServiceToken {
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
    /// use threescalers::credentials::*;
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
    /// use threescalers::credentials::*;
    ///
    /// let creds = Credentials::from_token("my_token");
    /// ```
    pub fn from_token<T: Into<ServiceToken>>(token: T) -> Self {
        Credentials::ServiceToken(token.into())
    }
}

impl<'k, 'v, E> ToParams<'k, 'v, E> for Credentials where E: Extend<(&'k str, &'v str)> {
    fn to_params<'s: 'k + 'v>(&'s self, extendable: &mut E) {
        use self::Credentials::*;

        let (field, value) = match self {
            ProviderKey(key) => ("provider_key", key.as_ref()),
            ServiceToken(token) => ("service_token", token.as_ref())
        };

        extendable.extend([(field, value)].iter().cloned());
    }
}

#[derive(Debug)]
pub struct ServiceId(String);

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

impl From<&str> for ServiceId where Self: FromStr {
    fn from(s: &str) -> ServiceId {
        s.parse().unwrap()
    }
}

impl From<String> for ServiceId {
    fn from(s: String) -> ServiceId {
        ServiceId(s)
    }
}

