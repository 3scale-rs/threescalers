use request::ToParams;

#[derive(Debug)]
pub enum Application {
    AppId(String, Option<String>),
    UserKey(String),
    OAuthToken(String)
}

impl Application {
    pub fn from_app_id(app_id: String) -> Self {
        Application::AppId(app_id, None)
    }

    pub fn from_app_id_and_key(app_id: String, app_key: String) -> Self {
        Application::AppId(app_id, Some(app_key))
    }

    pub fn from_user_key(user_key: String) -> Self {
        Application::UserKey(user_key)
    }

    pub fn from_oauth_token(token: String) -> Self {
        Application::OAuthToken(token)
    }
}

impl ToParams for Application {
    fn to_params(&self) -> Vec<(&str, &str)>{
        use self::Application::*;

        let mut v = Vec::<(&str, &str)>::with_capacity(2);
        match *self {
            AppId(ref app_id, None) => {
                v.push(("app_id", app_id));
            },
            AppId(ref app_id, Some(ref app_key)) => {
                v.push(("app_id", app_id));
                v.push(("app_key", app_key));
            },
            UserKey(ref user_key) => v.push(("user_key", user_key)),
            OAuthToken(ref token) => v.push(("access_token", token)),
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
