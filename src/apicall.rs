use request::{Request, ToRequest};
use service::Service;
use application::Application;
use user::User;

#[derive(Debug)]
pub enum Type {
    Authorize,
    AuthRep,
    Report
}

impl Type {
    pub fn method(&self) -> String {
        use self::Type::*;
        match *self {
            Report => "POST".to_owned(),
            AuthRep | Authorize => "GET".to_owned(),
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

// TODO: define endpoints for oauth
const AUTH_ENDPOINT: &str = "/transactions/authorize.xml";
const AUTHREP_ENDPOINT: &str = "/transactions/authrep.xml";
const REPORT_ENDPOINT: &str = "/transactions.xml";

impl<'service, 'app, 'user> Info<'service, 'app, 'user> {
    pub fn new(kind: Type, service: &'service Service, application: &'app Application, user: Option<&'user User>) -> Self {
        Self { kind, service, application, user }
    }

    fn endpoint(&self) -> &str {
        use self::Type::*;

        match self.kind {
            Authorize => AUTH_ENDPOINT,
            AuthRep => AUTHREP_ENDPOINT,
            Report => REPORT_ENDPOINT,
        }
    }

    fn path(&self) -> String {
        use request::ToParams;

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.extend(self.service.to_params().iter());
        params.extend(self.application.to_params().iter());

        if let Some(user_params) = self.user.as_ref() {
            params.extend(user_params.to_params().iter());
        }

        let params = params.iter()
            .map(|&(param, value)| param.to_owned() + "=" + value)
            .collect::<Vec<String>>()
            .join("&");

        self.endpoint().to_owned() + "?" + params.as_str()
    }
}

impl<'service, 'app, 'user> ToRequest for Info<'service, 'app, 'user> {
    fn to_request(&self) -> Request {
        Request::new(self.kind.method(), self.path(), None)
    }
}
