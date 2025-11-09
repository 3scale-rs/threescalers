use std::prelude::v1::*;

use crate::{Error, ToParams};

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuthToken(String);

// These trait impls provide a way to reference our types as &str
impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for OAuthToken {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// These trait impls provide a way to &str#parse()
impl FromStr for UserId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl FromStr for OAuthToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

// These trait impls are similar to FromStr (but are infallible)
impl From<&str> for UserId
where
    Self: FromStr,
{
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for OAuthToken
where
    Self: FromStr,
{
    #[allow(clippy::unwrap_used)]
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

// These trait impls take ownership of a given String
impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for OAuthToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum User {
    UserId(UserId),
    OAuthToken(OAuthToken),
}

impl From<UserId> for User {
    fn from(uid: UserId) -> Self {
        Self::UserId(uid)
    }
}

impl From<OAuthToken> for User {
    fn from(token: OAuthToken) -> Self {
        Self::OAuthToken(token)
    }
}

impl User {
    /// Creates a `User` from a `UserId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::user::*;
    ///
    /// let user = User::from_user_id("my_id");
    /// ```
    pub fn from_user_id<T: Into<UserId>>(user_id: T) -> Self {
        Self::UserId(user_id.into())
    }

    /// Creates a `User` from an `OAuthToken`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::user::*;
    ///
    /// let user = User::from_oauth_token("my_token");
    /// ```
    pub fn from_oauth_token<T: Into<OAuthToken>>(token: T) -> Self {
        Self::OAuthToken(token.into())
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for User
where
    'this: 'k + 'v,
    E: Extend<(Cow<'k, str>, &'v str)>,
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(
        &'this self,
        extendable: &mut E,
        key_mangling: &mut F,
    ) {
        let (field, value) = match self {
            Self::UserId(user_id) => ("user_id", user_id.as_ref()),
            Self::OAuthToken(token) => ("access_token", token.as_ref()),
        };

        extendable.extend(core::iter::once(&(key_mangling(field.into()), value)).cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_user_id_into_params() {
        let user_id = "my_user_id";
        let user = User::from_user_id(user_id.to_owned());

        let mut result = Vec::new();
        user.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![("user_id".into(), user_id)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_oauth_token_into_params() {
        let oauth_token = "my_oauth_token";
        let user = User::from_oauth_token(oauth_token.to_owned());

        let mut result = Vec::new();
        user.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![("access_token".into(), oauth_token)];
        assert_eq!(expected, result);
    }
}
