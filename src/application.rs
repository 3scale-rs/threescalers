use crate::ToParams;
use crate::errors::*;

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct AppId(String);
#[derive(Debug, Clone, PartialEq)]
pub struct AppKey(String);
#[derive(Debug, Clone, PartialEq)]
pub struct UserKey(String);
#[derive(Debug, Clone, PartialEq)]
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
        Ok(AppId(s.into()))
    }
}

impl FromStr for AppKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<AppKey> {
        Ok(AppKey(s.into()))
    }
}

impl FromStr for UserKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<UserKey> {
        Ok(UserKey(s.into()))
    }
}

impl FromStr for OAuthToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<OAuthToken> {
        Ok(OAuthToken(s.into()))
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

#[derive(Debug, Clone, PartialEq)]
pub enum Application {
    AppId(AppId, Option<AppKey>),
    UserKey(UserKey),
    OAuthToken(OAuthToken)
}

// These trait impls build an Application variant out of its required types
impl From<AppId> for Application {
    fn from(a: AppId) -> Self {
        Application::AppId(a, None)
    }
}

impl From<(AppId, AppKey)> for Application {
    fn from(a: (AppId, AppKey)) -> Self {
        Application::AppId(a.0, Some(a.1))
    }
}

impl From<UserKey> for Application {
    fn from(u: UserKey) -> Self {
        Application::UserKey(u)
    }
}

impl From<OAuthToken> for Application {
    fn from(o: OAuthToken) -> Self {
        Application::OAuthToken(o)
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

    /// Creates a new `Application` from an `OAuthToken`.
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

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Application where 'this: 'k + 'v, E: Extend<(Cow<'k, str>, &'v str)> {
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self, extendable: &mut E, key_mangling: &mut F) {
        use self::Application::*;

        let params = match self {
            AppId(app_id, app_key_opt) => [
                Some(("app_id", app_id.as_ref())),
                app_key_opt.as_ref().map(|app_key| ("app_key", app_key.as_ref()))
            ],
            UserKey(user_key) => [Some(("user_key", user_key.as_ref())), None],
            OAuthToken(token) => [Some(("access_token", token.as_ref())), None],
        };

        extendable.extend(params.iter().filter_map(|&param| {
            param.map(|(k, v)| {
                (key_mangling(k.into()), v)
            })
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_application_from_app_id() {
        let app_id = AppId::from("my_app_id");
        let app = Application::from(app_id.clone());

        assert_eq!(Application::AppId(app_id, None), app);
    }

    #[test]
    fn convert_application_from_app_id_app_key() {
        let app_id_key = (AppId::from("my_app_id"), AppKey::from("my_app_key"));
        let app = Application::from(app_id_key.clone());

        assert_eq!(Application::AppId(app_id_key.0, Some(app_id_key.1)), app);
    }

    #[test]
    fn convert_application_from_user_key() {
        let user_key = UserKey::from("my_user_key");
        let app = Application::from(user_key.clone());

        assert_eq!(Application::UserKey(user_key), app);
    }

    #[test]
    fn convert_application_from_oauth_token() {
        let token = OAuthToken::from("my_oauth_token");
        let app = Application::from(token.clone());

        assert_eq!(Application::OAuthToken(token), app);
    }

    #[test]
    fn transforms_app_id_into_params() {
        let app_id = "my_app_id";
        let app = Application::from_app_id(app_id);

        let mut result = Vec::new();
        app.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![("app_id".into(), app_id)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_app_id_and_key_into_params() {
        let app_id = "my_app_id";
        let key = "my_key";
        let app = Application::from_app_id_and_key(app_id, key);

        let mut result = Vec::new();
        app.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)>  = vec![("app_id".into(), app_id), ("app_key".into(), key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_user_key_into_params() {
        let user_key = "my_user_key";
        let app = Application::from_user_key(user_key);

        let mut result = Vec::new();
        app.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)>  = vec![("user_key".into(), user_key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_oauth_token_into_params() {
        let oauth_token = "my_token";
        let app = Application::from_oauth_token(oauth_token);

        let mut result = Vec::new();
        app.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)>  = vec![("access_token".into(), oauth_token)];
        assert_eq!(expected, result);
    }
}
