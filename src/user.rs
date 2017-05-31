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
    fn to_params(&self) -> Vec<(&str, &str)> {
        let (field, value) = match *self {
            User::UserId(ref user_id) => ("user_id", user_id),
            User::OAuthToken(ref token) => ("access_token", token)
        };
        vec![(field, value)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_user_id_into_params() {
        let user_id = "my_user_id";
        let user = User::from_user_id(user_id.to_owned());

        let result = user.to_params();

        let expected = vec![("user_id", user_id)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_oauth_token_into_params() {
        let oauth_token = "my_oauth_token";
        let user = User::from_oauth_token(oauth_token.to_owned());

        let result = user.to_params();

        let expected = vec![("access_token", oauth_token)];
        assert_eq!(expected, result);
    }
}
