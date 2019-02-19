use crate::ToParams;
use crate::errors::*;

use std::str::FromStr;

#[derive(Debug)]
pub struct UserId(String);
#[derive(Debug)]
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

    fn from_str(s: &str) -> Result<UserId> {
        Ok(UserId(s.to_owned()))
    }
}

impl FromStr for OAuthToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<OAuthToken> {
        Ok(OAuthToken(s.to_owned()))
    }
}

// These trait impls are similar to FromStr (but are infallible)
impl From<&str> for UserId where Self: FromStr {
    fn from(s: &str) -> UserId {
        s.parse().unwrap()
    }
}

impl From<&str> for OAuthToken where Self: FromStr {
    fn from(s: &str) -> OAuthToken {
        s.parse().unwrap()
    }
}

// These trait impls take ownership of a given String
impl From<String> for UserId {
    fn from(s: String) -> UserId {
        UserId(s)
    }
}

impl From<String> for OAuthToken {
    fn from(s: String) -> OAuthToken {
        OAuthToken(s)
    }
}

#[derive(Debug)]
pub enum User {
    UserId(UserId),
    OAuthToken(OAuthToken)
}

impl Into<User> for UserId {
    fn into(self) -> User {
        User::UserId(self)
    }
}

impl Into<User> for OAuthToken {
    fn into(self) -> User {
        User::OAuthToken(self)
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
        User::UserId(user_id.into())
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
        User::OAuthToken(token.into())
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for User where 'this: 'k + 'v, E: Extend<(Cow<'k, str>, &'v str)> {
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self, extendable: &mut E, key_mangling: &mut F) {
        let (field, value) = match self {
            User::UserId(user_id) => ("user_id", user_id.as_ref()),
            User::OAuthToken(token) => ("access_token", token.as_ref())
        };

        extendable.extend([(key_mangling(field.into()), value)].iter().cloned());
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

        let expected = vec![("user_id", user_id)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_oauth_token_into_params() {
        let oauth_token = "my_oauth_token";
        let user = User::from_oauth_token(oauth_token.to_owned());

        let mut result = Vec::new();
        user.to_params(&mut result);

        let expected = vec![("access_token", oauth_token)];
        assert_eq!(expected, result);
    }
}
