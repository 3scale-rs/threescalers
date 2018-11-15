use crate::request::{Request, ToRequest};
use crate::service::Service;
use crate::application::Application;
use crate::user::User;

#[derive(Debug)]
pub enum Type {
    Authorize,
    AuthRep,
    Report
}

impl Type {
    pub fn method(&self) -> &str {
        use self::Type::*;
        match *self {
            Report => "POST",
            AuthRep | Authorize => "GET",
        }
    }
}

#[derive(Debug)]
pub struct Info<'service, 'app, 'user> {
    kind: Type,
    service: &'service Service,
    application: &'app Application,
    user: Option<&'user User>,
}

const AUTHORIZE_ENDPOINT: &str = "/transactions/authorize.xml";
const AUTHREP_ENDPOINT: &str = "/transactions/authrep.xml";
const REPORT_ENDPOINT: &str = "/transactions.xml";
const OAUTH_AUTHORIZE_ENDPOINT: &str = "/transactions/oauth_authorize.xml";
const OAUTH_AUTHREP_ENDPOINT: &str = "/transactions/oauth_authrep.xml";

impl<'service, 'app, 'user> Info<'service, 'app, 'user> {
    pub fn new(kind: Type, service: &'service Service, application: &'app Application, user: Option<&'user User>) -> Self {
        Self { kind, service, application, user }
    }

    fn endpoint(&self) -> &str {
        use self::Type::*;

        match (&self.kind, self.application, self.user) {
            (&Authorize, &Application::OAuthToken(_), _) => OAUTH_AUTHORIZE_ENDPOINT,
            (&Authorize, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHORIZE_ENDPOINT,
            (&Authorize, _, _) => AUTHORIZE_ENDPOINT,
            (&AuthRep, &Application::OAuthToken(_), _) => OAUTH_AUTHREP_ENDPOINT,
            (&AuthRep, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHREP_ENDPOINT,
            (&AuthRep, _, _) => AUTHREP_ENDPOINT,
            (&Report, _, _) => REPORT_ENDPOINT,
        }
    }

    fn params(&self) -> Vec<(&str, &str)> {
        use crate::request::ToParams;

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.extend(self.service.to_params());
        params.extend(self.application.to_params());

        if let Some(user_params) = self.user.as_ref() {
            params.extend(user_params.to_params());
        }

        params
    }
}

impl<'service, 'app, 'user> ToRequest for Info<'service, 'app, 'user> {
    fn to_request(&self) -> Request {
        Request::new(self.kind.method(), self.endpoint(), self.params(), None)
    }
}
