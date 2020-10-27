use std::prelude::v1::*;

use crate::{
    application::Application,
    extensions::List,
    service::Service,
    transaction::Transaction,
    usage::Usage,
    user::User,
};

use crate::ToParams;

use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Authorize,
    AuthRep,
    Report,
}

impl Kind {
    // report requires specific treatment due to being the only call supporting
    // multiple transactions.
    pub fn is_report(self) -> bool {
        matches!(self, Kind::Report)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ApiCall<'service, 'tx, 'app, 'user, 'usage, 'extensions> {
    kind:         Kind,
    service:      &'service Service,
    transactions: &'tx [Transaction<'app, 'user, 'usage>],
    extensions:   Option<&'extensions List<'extensions>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Builder<'service, 'tx, 'app, 'user, 'usage, 'extensions> {
    service:      &'service Service,
    kind:         Option<Kind>,
    transactions: &'tx [Transaction<'app, 'user, 'usage>],
    extensions:   Option<&'extensions List<'extensions>>,
}

// TODO: we can improve this with a state machine of types so that we are required to set svc, app,
// user and kind before being able to set (required) the usage to build the call
impl<'service, 'tx, 'app, 'user, 'usage, 'extensions>
    Builder<'service, 'tx, 'app, 'user, 'usage, 'extensions>
{
    pub fn new(service: &'service Service) -> Self {
        Builder { service,
                  kind: Default::default(),
                  transactions: Default::default(),
                  extensions: Default::default() }
    }

    pub fn service(&mut self, s: &'service Service) -> &mut Self {
        self.service = s;
        self
    }

    pub fn kind(&mut self, t: Kind) -> &mut Self {
        self.kind = Some(t);
        self
    }

    pub fn transactions(&mut self, txns: &'tx [Transaction<'app, 'user, 'usage>]) -> &mut Self {
        self.transactions = txns;
        self
    }

    pub fn extensions(&mut self, extensions: &'extensions List) -> &mut Self {
        self.extensions = Some(extensions);
        self
    }

    pub fn build(&self) -> Result<ApiCall, Box<dyn Error>> {
        let kind = self.kind.ok_or_else(|| "kind error".to_string())?;
        Ok(ApiCall::new(kind,
                        self.service,
                        self.transactions,
                        self.extensions))
    }
}

use std::borrow::Cow;

impl<'service, 'tx: 'app + 'user + 'usage, 'app, 'user, 'usage, 'extensions>
    ApiCall<'service, 'tx, 'app, 'user, 'usage, 'extensions>
{
    pub fn builder(service: &'service Service) -> Builder {
        Builder::new(service)
    }

    pub fn new(kind: Kind,
               service: &'service Service,
               transactions: &'tx [Transaction<'app, 'user, 'usage>],
               extensions: Option<&'extensions List>)
               -> Self {
        Self { kind,
               service,
               transactions,
               extensions }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn service(&self) -> &'service Service {
        self.service
    }

    pub fn transactions(&self) -> &'tx [Transaction<'app, 'user, 'usage>] {
        self.transactions
    }

    // helper to get a transaction only if it's the only one
    // useful for non-report calls
    pub fn transaction(&self) -> Option<&'tx Transaction<'app, 'user, 'usage>> {
        let txns = self.transactions();

        if txns.len() == 1 { Some(&txns[0]) } else { None }
    }

    pub fn application(&self) -> Option<&'app Application> {
        self.transaction().map(Transaction::application)
    }

    pub fn user(&self) -> Option<&'user User> {
        self.transaction().and_then(Transaction::user)
    }

    pub fn usage(&self) -> Option<&'usage Usage> {
        self.transaction().and_then(Transaction::usage)
    }

    pub fn extensions(&self) -> Option<&'extensions List> {
        self.extensions
    }

    pub fn params(&self) -> Vec<(Cow<'_, str>, &str)> {
        let mut params = Vec::with_capacity(8);

        self.to_params(&mut params);
        params
    }
}

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for ApiCall<'_, '_, '_, '_, '_, '_>
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)>
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self,
                                                                       extendable: &mut E,
                                                                       key_mangling: &mut F) {
        self.service.to_params_with_mangling(extendable, key_mangling);

        // keep the borrowck happy about stack closures living long enough
        let mut txfn_storage_report;
        let mut txfn_storage_rest;

        let key_mangling: &mut dyn FnMut(usize, Cow<'k, str>) -> Cow<'k, str> = if self.kind().is_report() {
            txfn_storage_report = |n, c: Cow<'k, str>| {
                // 3scale Apisonator takes arguments using the Rack format
                key_mangling(format!("transactions[{}]{}", n, c).into())
            };
            &mut txfn_storage_report
        } else {
            txfn_storage_rest = |_n, c: Cow<'k, str>| key_mangling(c);
            &mut txfn_storage_rest
        };

        // having multiple transactions with non-report endpoints
        // is not allowed, but we can't fail in this trait impl
        // (plus it would make sense to allow transactions in the
        // other endpoints).
        for (e, tx) in self.transactions().iter().enumerate() {
            tx.to_params_with_mangling(extendable, &mut |c| key_mangling(e, c));
        }
    }
}
