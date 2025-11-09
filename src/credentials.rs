use std::prelude::v1::*;

use crate::ToParams;

use crate::Error;

use core::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderKey(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceToken(String);

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl FromStr for ServiceToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

// These trait impls are similar to FromStr (but are infallible)
#[expect(clippy::fallible_impl_from, clippy::unwrap_used)]
impl From<&str> for ProviderKey
where
    Self: FromStr,
{
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

#[expect(clippy::fallible_impl_from, clippy::unwrap_used)]
impl From<&str> for ServiceToken
where
    Self: FromStr,
{
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

// These trait impls take ownership of a given String
impl From<String> for ProviderKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for ServiceToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<ProviderKey> for Credentials {
    fn from(pk: ProviderKey) -> Self {
        Self::ProviderKey(pk)
    }
}

impl From<ServiceToken> for Credentials {
    fn from(token: ServiceToken) -> Self {
        Self::ServiceToken(token)
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
        Self::ProviderKey(key.into())
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
        Self::ServiceToken(token.into())
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Credentials
where
    'this: 'k + 'v,
    E: Extend<(Cow<'k, str>, &'v str)>,
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(
        &'this self,
        extendable: &mut E,
        key_mangling: &mut F,
    ) {
        use self::Credentials::*;

        let (field, value) = match self {
            ProviderKey(key) => (key_mangling("provider_key".into()), key.as_ref()),
            ServiceToken(token) => (key_mangling("service_token".into()), token.as_ref()),
        };

        extendable.extend(core::iter::once(&(field, value)).cloned());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceId(String);

impl AsRef<str> for ServiceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for ServiceId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

#[expect(clippy::fallible_impl_from, clippy::unwrap_used)]
impl From<&str> for ServiceId
where
    Self: FromStr,
{
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl From<String> for ServiceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}
