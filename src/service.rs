use std::prelude::v1::*;

use crate::{
    credentials::{Credentials, ServiceId},
    ToParams,
};

#[derive(Debug)]
pub struct Service {
    service_id: ServiceId,
    creds: Credentials,
}

impl Service {
    /// Creates a new `Service` from a `ServiceId` and `Credentials`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::credentials::*;
    /// use threescalers::service::*;
    ///
    /// let creds = Credentials::from_token("my_token");
    /// let service = Service::new("my_service_id", creds);
    /// ```
    pub fn new<T: Into<ServiceId>>(service_id: T, creds: Credentials) -> Self {
        Self {
            service_id: service_id.into(),
            creds,
        }
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Service
where
    'this: 'k + 'v,
    E: Extend<(Cow<'k, str>, &'v str)>,
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(
        &'this self,
        extendable: &mut E,
        key_mangling: &mut F,
    ) {
        let key = key_mangling("service_id".into());

        extendable.extend([(key, self.service_id.as_ref())].iter().cloned());

        self.creds.to_params_with_mangling(extendable, key_mangling);
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;

    #[test]
    fn transforms_service_id_and_key_into_params() {
        let service_id = "my_service_id";
        let provider_key = "my_provider_key";
        let creds = Credentials::from_key(provider_key);
        let service = Service::new(service_id, creds);

        let mut result = Vec::new();
        service.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![
            ("service_id".into(), service_id),
            ("provider_key".into(), provider_key),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_service_id_and_token_into_params() {
        let service_id = "my_service_id";
        let token = "my_token";
        let creds = Credentials::from_token(token);
        let service = Service::new(service_id, creds);

        let mut result = Vec::new();
        service.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![
            ("service_id".into(), service_id),
            ("service_token".into(), token),
        ];
        assert_eq!(expected, result);
    }
}
