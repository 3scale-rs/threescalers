use crate::request::{Request, ToRequest};
use crate::service::Service;
use crate::application::Application;
use crate::user::User;
use crate::usage::Usage;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    Authorize,
    AuthRep,
    Report
}

impl Type {
    pub fn method(&self) -> &str {
        use self::Type::*;
        match self {
            Report => "POST",
            AuthRep | Authorize => "GET",
        }
    }
}

type Extensions = HashMap<String, String>;

#[derive(Debug)]
pub struct Info<'service, 'app, 'user, 'usage, 'extensions> {
    kind: Type,
    service: &'service Service,
    application: &'app Application,
    user: Option<&'user User>,
    usage: Option<&'usage Usage>,
    extensions: Option<&'extensions Extensions>,
}

impl<'service, 'app, 'user, 'usage, 'extensions> Info<'service, 'app, 'user, 'usage, 'extensions> {
    pub fn new(kind: Type, service: &'service Service, application: &'app Application,
               user: Option<&'user User>, usage: Option<&'usage Usage>,
               extensions: Option<&'extensions Extensions>) -> Self {
        Self { kind, service, application, user, usage, extensions }
    }

    pub fn kind(&self) -> &Type {
        &self.kind
    }

    pub fn service(&self) -> &'service Service {
        self.service
    }

    pub fn application(&self) -> &'app Application {
        self.application
    }

    pub fn user(&self) -> Option<&'user User> {
        self.user
    }

    pub fn extensions(&self) -> Option<&'extensions Extensions> {
        self.extensions
    }

    pub fn params(&self) -> Vec<(&str, &str)> {
        use crate::request::ToParams;

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.extend(self.service.to_params());
        params.extend(self.application.to_params());

        if let Some(user_params) = self.user.as_ref() {
            params.extend(user_params.to_params());
        }

        if let Some(usage_params) = self.usage.as_ref() {
            params.extend(usage_params.to_params());
        }

        params
    }
}

//impl<'service, 'app, 'user, 'usage, 'extensions> ToRequest for Info<'service, 'app, 'user, 'usage, 'extensions> {
impl ToRequest for Info<'_, '_, '_, '_, '_> {
//impl ToRequest for Info {
    fn to_request(&self) -> Request {
        Request::new(self.kind.method(), "obsolete_endpoint_str", self.params(), None)
    }
}
