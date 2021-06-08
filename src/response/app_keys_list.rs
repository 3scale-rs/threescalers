use std::prelude::v1::*;

use std::fmt;

use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize,
};

use crate::{
    application::{AppId, AppKey},
    credentials::ServiceId,
};

#[derive(Debug, PartialEq, Eq)]
pub struct AppKeysList {
    service_id: Option<ServiceId>,
    app_id: Option<AppId>,
    keys: Vec<AppKey>,
}

impl AppKeysList {
    pub fn new<S: Into<ServiceId>, A: Into<AppId>, K: Into<AppKey>, I: IntoIterator<Item = K>>(
        service_id: Option<S>,
        app_id: Option<A>,
        keys: I,
    ) -> Self {
        Self {
            service_id: service_id.map(Into::into),
            app_id: app_id.map(Into::into),
            keys: keys.into_iter().map(Into::into).collect(),
        }
    }

    pub fn service_id(&self) -> Option<&ServiceId> {
        self.service_id.as_ref()
    }

    pub fn app_id(&self) -> Option<&AppId> {
        self.app_id.as_ref()
    }

    pub fn keys(&self) -> &[AppKey] {
        self.keys.as_slice()
    }
}

struct AppKeysListVisitor;

impl<'de> Visitor<'de> for AppKeysListVisitor {
    type Value = AppKeysList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a structure that represents the application's keys")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct AppKeyWithId {
            id: String,
        }

        let mut app_id: Option<AppId> = None;
        let mut service_id: Option<ServiceId> = None;
        // the usual maximum capacity is 5 entries
        let mut keys: Vec<AppKey> = Vec::with_capacity(5);

        while let Some(ref attr) = map.next_key::<String>()? {
            match attr.as_str() {
                "app" => {
                    app_id = Some(map.next_value::<String>()?.into());
                }
                "svc" => {
                    service_id = Some(map.next_value::<String>()?.into());
                }
                "key" => {
                    let appkeyid = map.next_value::<AppKeyWithId>()?;
                    keys.push(AppKey::from(appkeyid.id));
                }
                // unknown keys are just ignored
                _ => (),
            }
        }

        let list_app_keys = AppKeysList::new(service_id, app_id, keys);

        Ok(list_app_keys)
    }
}

impl<'de> Deserialize<'de> for AppKeysList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AppKeysListVisitor)
    }
}
