mod apicall {
    pub struct APICallInfo {
        kind: APICallType,
        service: service::Service,
        app: app::App,
        user: Option<user::User>,
    }

    pub enum APICallType {
        Authorize,
        AuthRep,
        Report
    }

    impl APICallType {
        pub fn method(&self) -> String {
            match self {
                Authorize => "GET".to_owned(),
                _ => "POST".to_owned()
            }
        }
    }

    impl APICallInfo {
        pub fn new(kind: APICallType, service: service::Service, app: app::App, user: Option<user::User>) -> Self {
            Self { kind, service, app, user }
        }
    }

    pub trait ToParams {
        fn to_params(&self) -> String;
    }

    mod service {
        pub struct Service {
            service_id: String,
            creds: Credentials,
        }

        impl super::ToParams for Service {
            fn to_params(&self) -> String {
                let mut param = "service_id=".to_owned();
                param.push_str(self.service_id.as_str());
                param.push_str(self.creds.to_params().as_str());
                param
            }
        }

        pub enum Credentials {
            ProviderKey(String),
            ServiceToken(String),
        }

        impl super::ToParams for Credentials {
            fn to_params(&self) -> String {
                let (field, value) = match *self {
                    Credentials::ProviderKey(ref key) => ("provider_key=", key),
                    Credentials::ServiceToken(ref token) => ("service_token=", token)
                };
                let mut param = field.to_owned();
                param.push_str(value.as_str());
                param
            }
        }
    }

    mod app {
        pub enum App {
            AppId(String, Option<String>),
            UserKey(String),
            OAuthToken(String)
        }
    }

    mod user {
        pub enum User {
            UserId(String),
            OAuthToken(String)
        }
    }

    pub struct Request {
        method: String,
        path: String,
        body: Option<String>
    }

    pub trait ToRequest {
        fn to_request(&self) -> Request;
    }

    impl ToRequest for APICallInfo {
        fn to_request(&self) -> Request {
            Request {
                method: self.kind.method(),
                path: "/".to_owned(),
                body: None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
