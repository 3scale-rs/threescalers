use super::{
    application::Application,
    timestamp::Timestamp,
    usage::Usage,
    user::User,
    ToParams,
};

#[derive(Copy, Clone, Debug)]
pub struct Transaction<'app, 'user, 'usage, 'ts> {
    application: &'app Application,
    user:        Option<&'user User>,
    usage:       Option<&'usage Usage<'usage>>,
    timestamp:   Option<&'ts Timestamp>,
}

impl<'app, 'user, 'usage, 'ts> Transaction<'app, 'user, 'usage, 'ts> {
    pub fn new(application: &'app Application,
               user: Option<&'user User>,
               usage: Option<&'usage Usage>,
               timestamp: Option<&'ts Timestamp>)
               -> Self {
        Self { application,
               user,
               usage,
               timestamp }
    }

    pub fn application(&self) -> &Application {
        self.application
    }

    pub fn user(&self) -> Option<&User> {
        self.user
    }

    pub fn usage(&self) -> Option<&Usage> {
        self.usage
    }

    pub fn timestamp(&self) -> Option<&Timestamp> {
        self.timestamp
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Timestamp
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)>
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self,
                                                                       extendable: &mut E,
                                                                       key_mangling: &mut F) {
        let field = key_mangling("timestamp".into());
        extendable.extend([(field, self.as_str())].iter().cloned());
    }
}

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Transaction<'_, '_, '_, '_>
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)>
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self,
                                                                       extendable: &mut E,
                                                                       key_mangling: &mut F) {
        if let Some(ts) = self.timestamp {
            ts.to_params_with_mangling(extendable, key_mangling);
        }

        self.application.to_params_with_mangling(extendable, key_mangling);

        if let Some(user_params) = self.user {
            user_params.to_params_with_mangling(extendable, key_mangling);
        }

        if let Some(usage_params) = self.usage {
            usage_params.to_params_with_mangling(extendable, key_mangling);
        }
    }
}
