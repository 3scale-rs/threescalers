use request::ToParams;
use errors::*;

use std::str::FromStr;

#[derive(Debug)]
pub struct AppId(String);
#[derive(Debug)]
pub struct AppKey(String);
#[derive(Debug)]
pub struct UserKey(String);
#[derive(Debug)]
pub struct OAuthToken(String);

// These trait impls provide a way to reference our types as &str
impl AsRef<str> for AppId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for AppKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for UserKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for OAuthToken {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// These trait impls provide a way to &str#parse() our Application type
impl FromStr for AppId {
    type Err = Error;

    fn from_str(s: &str) -> Result<AppId> {
        Ok(AppId(s.to_owned()))
    }
}

impl FromStr for AppKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<AppKey> {
        Ok(AppKey(s.to_owned()))
    }
}

impl FromStr for UserKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<UserKey> {
        Ok(UserKey(s.to_owned()))
    }
}

impl FromStr for OAuthToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<OAuthToken> {
        Ok(OAuthToken(s.to_owned()))
    }
}

// These trait impls are similar to FromStr (but are infallible)
impl<'a> From<&'a str> for AppId where Self: FromStr {
    fn from(s: &'a str) -> AppId {
        s.parse().unwrap()
    }
}

impl<'a> From<&'a str> for AppKey where Self: FromStr {
    fn from(s: &'a str) -> AppKey {
        s.parse().unwrap()
    }
}

impl<'a> From<&'a str> for UserKey where Self: FromStr {
    fn from(s: &'a str) -> UserKey {
        s.parse().unwrap()
    }
}

impl<'a> From<&'a str> for OAuthToken where Self: FromStr {
    fn from(s: &'a str) -> OAuthToken {
        s.parse().unwrap()
    }
}

// These trait impls take ownership of a given String and also provide Into<AppId> for String
impl From<String> for AppId {
    fn from(s: String) -> AppId {
        AppId(s)
    }
}

impl From<String> for AppKey {
    fn from(s: String) -> AppKey {
        AppKey(s)
    }
}

impl From<String> for UserKey {
    fn from(s: String) -> UserKey {
        UserKey(s)
    }
}

impl From<String> for OAuthToken {
    fn from(s: String) -> OAuthToken {
        OAuthToken(s)
    }
}

#[derive(Debug)]
pub enum Application {
    AppId(AppId, Option<AppKey>),
    UserKey(UserKey),
    OAuthToken(OAuthToken)
}

// These trait impls build an Application variant out of its required types
impl Into<Application> for AppId {
    fn into(self) -> Application {
        Application::AppId(self, None)
    }
}

impl Into<Application> for (AppId, AppKey) {
    fn into(self) -> Application {
        Application::AppId(self.0, Some(self.1))
    }
}

impl Into<Application> for UserKey {
    fn into(self) -> Application {
        Application::UserKey(self)
    }
}

impl Into<Application> for OAuthToken {
    fn into(self) -> Application {
        Application::OAuthToken(self)
    }
}

impl Application {
    /// Creates a new `Application` from an `AppId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::application::*;
    ///
    /// let app = Application::from_app_id("my_app_id");
    /// ```
    pub fn from_app_id<T: Into<AppId>>(app_id: T) -> Self {
        Application::AppId(app_id.into(), None)
    }

    /// Creates a new `Application` from an `AppId` and an `AppKey`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::application::*;
    ///
    /// let app = Application::from_app_id_and_key("my_app_id", "my_app_key");
    /// ```
    pub fn from_app_id_and_key<T: Into<AppId>, U: Into<AppKey>>(app_id: T, app_key: U) -> Self {
        Application::AppId(app_id.into(), Some(app_key.into()))
    }

    /// Creates a new `Application` from a `UserKey`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::application::*;
    ///
    /// let app = Application::from_user_key("my_user_key");
    /// ```
    pub fn from_user_key<T: Into<UserKey>>(user_key: T) -> Self {
        Application::UserKey(user_key.into())
    }

    /// Creates a new `Application` from an `AppId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::application::*;
    ///
    /// let app = Application::from_oauth_token("my_token");
    /// ```
    pub fn from_oauth_token<T: Into<OAuthToken>>(token: T) -> Self {
        Application::OAuthToken(token.into())
    }
}

impl ToParams for Application {
    fn to_params(&self) -> Vec<(&str, &str)>{
        use self::Application::*;

        let mut v = Vec::<(&str, &str)>::with_capacity(2);
        match *self {
            AppId(ref app_id, None) => {
                v.push(("app_id", app_id.as_ref()));
            },
            AppId(ref app_id, Some(ref app_key)) => {
                v.push(("app_id", app_id.as_ref()));
                v.push(("app_key", app_key.as_ref()));
            },
            UserKey(ref user_key) => v.push(("user_key", user_key.as_ref())),
            OAuthToken(ref token) => v.push(("access_token", token.as_ref())),
        };

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_app_id_into_params() {
        let app_id = "my_app_id";
        let app = Application::from_app_id(app_id.to_owned());

        let result = app.to_params();

        let expected = vec![("app_id", app_id)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_app_id_and_key_into_params() {
        let app_id = "my_app_id";
        let key = "my_key";
        let app = Application::from_app_id_and_key(app_id.to_owned(),
                                                   key.to_owned());

        let result = app.to_params();

        let expected = vec![("app_id", app_id), ("app_key", key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_user_key_into_params() {
        let user_key = "my_user_key";
        let app = Application::from_user_key(user_key.to_owned());

        let result = app.to_params();

        let expected = vec![("user_key", user_key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_oauth_token_into_params() {
        let oauth_token = "my_token";
        let app = Application::from_oauth_token(oauth_token.to_owned());

        let result = app.to_params();

        let expected = vec![("access_token", oauth_token)];
        assert_eq!(expected, result);
    }
}
