use request::ToParams;

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
    fn to_params(&self) -> String {
        use self::Application::*;

        let (field, value) = match *self {
            AppId(ref app_id, None) => {
                ("app_id=", app_id)
            },
            UserKey(ref user_key) => ("user_key=", user_key),
            OAuthToken(ref token) => ("access_token=", token),
            // TODO case where there's an app_key
            // This will most likely make us redesign how the interface works
            // I have planned to make `to_params` return a Vec of tuples.
            _ => unimplemented!()
        };

        let mut params = field.to_owned();
        params.push_str(value.as_str());
        params
    }
}
