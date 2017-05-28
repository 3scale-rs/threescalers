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
