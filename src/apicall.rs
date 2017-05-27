use request::{Request, ToRequest};
use service::Service;
use application::Application;
use user::User;

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

pub struct Info<'service, 'app, 'user> {
    kind: Type,
    service: &'service Service,
    application: &'app Application,
    user: Option<&'user User>,
}

impl<'service, 'app, 'user> Info<'service, 'app, 'user> {
    pub fn new(kind: Type, service: &'service Service, application: &'app Application, user: Option<&'user User>) -> Self {
        Self { kind, service, application, user }
    }
}

impl<'service, 'app, 'user> ToRequest for Info<'service, 'app, 'user> {
    fn to_request(&self) -> Request {
        use request::ToParams;

        let mut path: String = self.service.to_params();

        path.push_str(self.application.to_params().as_str());

        if let Some(user_params) = self.user.as_ref() {
            path.push_str(user_params.to_params().as_str());
        }

        Request::new(self.kind.method(), path, None)
    }
}