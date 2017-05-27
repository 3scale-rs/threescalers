use request::ToParams;

#[derive(Debug)]
pub enum User {
    UserId(String),
    OAuthToken(String)
}

impl User {
    pub fn from_user_id(user_id: String) -> Self {
        User::UserId(user_id)
    }

    pub fn from_oauth_token(token: String) -> Self {
        User::OAuthToken(token)
    }
}

impl ToParams for User {
    fn to_params(&self) -> String {
        let (field, value) = match *self {
            User::UserId(ref user_id) => ("user_id=", user_id),
            User::OAuthToken(ref token) => ("access_token=", token)
        };
        let mut params = field.to_owned();
        params.push_str(value.as_str());
        params
    }
}