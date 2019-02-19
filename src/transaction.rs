use std::time::SystemTime;

use super::application::Application;
use super::user::User;
use super::usage::Usage;
use super::ToParams;

#[derive(Clone, Debug)]
pub struct Timestamp {
    ts: String,
}

impl Timestamp {
    pub fn none() -> Option<&'static Timestamp> {
        None
    }

    pub fn new(ts: u64) -> Self {
        Self { ts: ts.to_string() }
    }

    pub fn into_inner(self) -> String {
        self.ts
    }

    pub fn as_str(&self) -> &str {
        self.ts.as_str()
    }
}

impl<'ts> AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        self.ts.as_str()
    }
}

impl From<u64> for Timestamp {
    fn from(ts: u64) -> Self {
        Self::new(ts)
    }
}

impl From<SystemTime> for Timestamp {
    fn from(st: SystemTime) -> Self {
        let ts = match st.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(ts) => ts.as_secs(),
            _ => 0,
        };

        Self::new(ts)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp::from(SystemTime::now())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Transaction<'app, 'user, 'usage, 'ts> {
    application: &'app Application,
    user: Option<&'user User>,
    usage: Option<&'usage Usage>,
    timestamp: Option<&'ts Timestamp>,
}

impl<'app, 'user, 'usage, 'ts> Transaction<'app, 'user, 'usage, 'ts> {
    pub fn new(application: &'app Application,
               user: Option<&'user User>,
               usage: Option<&'usage Usage>,
               timestamp: Option<&'ts Timestamp>) -> Self {
        Self { application, user, usage, timestamp }
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
          E: Extend<(Cow<'k, str>, &'v str)> {
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self, extendable: &mut E, key_mangling: &mut F) {
        let field = key_mangling("timestamp".into());
        extendable.extend([(field, self.as_str())].iter().cloned());
    }
}

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Transaction<'_, '_, '_, '_>
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)> {
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self, extendable: &mut E, key_mangling: &mut F) {
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
