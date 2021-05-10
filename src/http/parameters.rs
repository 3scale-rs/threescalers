use std::prelude::v1::*;

use super::Method;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub enum Parameters {
    Query(String),
    Body(String),
}

use std::{iter::Map, slice::Iter};

// This newtype is currently only used internally here, but we might want to move it elsewhere where
// it could be more useful because of genericity. We could also aim at reducing the amount of
// conversions in requests by having a type that only maps parameters once unless changed.
type ParamsMapper<'a, 'p, S, T> = Map<Iter<'a, (Cow<'p, str>, S)>, fn(&(Cow<'p, str>, S)) -> T>;

impl Parameters {
    pub fn new<S: AsRef<str>>(method: &Method, params: &[(Cow<str>, S)]) -> Self {
        let params_s = Self::params_to_query(params);

        if Self::method_requires_body(method) {
            Parameters::Body(params_s)
        } else {
            Parameters::Query(params_s)
        }
    }

    #[inline]
    fn method_requires_body(method: &Method) -> bool {
        !matches!(*method, Method::GET | Method::HEAD | Method::DELETE)
    }

    pub fn path_and_query<'p>(&self, path: &'p str) -> Cow<'p, str> {
        self.query().map_or_else(
            || Cow::Borrowed(path),
            |q| {
                let mut url = path.to_string();
                url.push('?');
                url.push_str(q);
                Cow::Owned(url)
            },
        )
    }

    pub fn uri_and_body<'p>(&self, path: &'p str) -> (Cow<'p, str>, Option<&str>) {
        (self.path_and_query(path), self.body())
    }

    pub fn query(&self) -> Option<&str> {
        match self {
            Parameters::Query(query) => Some(query.as_str()),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&str> {
        match self {
            Parameters::Body(body) => Some(body.as_str()),
            _ => None,
        }
    }

    pub fn into_inner(self) -> String {
        match self {
            Parameters::Query(s) => s,
            Parameters::Body(s) => s,
        }
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        match self {
            Parameters::Query(ref mut query) => query,
            Parameters::Body(ref mut body) => body,
        }
    }

    pub fn query_as_mut_string(&mut self) -> Option<&mut String> {
        match self {
            Parameters::Query(ref mut query) => Some(query),
            _ => None,
        }
    }

    pub fn body_as_mut_string(&mut self) -> Option<&mut String> {
        match self {
            Parameters::Body(ref mut body) => Some(body),
            _ => None,
        }
    }

    pub fn push<S: AsRef<str>>(&mut self, extra_params: &[(Cow<str>, S)]) {
        let q = Self::params_to_query(extra_params);
        let s = self.as_mut_string();

        if !s.is_empty() {
            s.push('&');
        }

        s.push_str(q.as_str());
    }

    fn params_to_string_collection<'p, 'a: 'p, S: AsRef<str>>(
        params: &'a [(Cow<str>, S)],
    ) -> ParamsMapper<'a, 'p, S, String> {
        params
            .iter()
            .map(|(k, v)| [k.as_ref(), "=", v.as_ref()].concat())
    }

    fn params_to_vec<S: AsRef<str>>(params: &[(Cow<str>, S)]) -> Vec<String> {
        Self::params_to_string_collection(params).collect()
    }

    fn params_to_query<S: AsRef<str>>(params: &[(Cow<str>, S)]) -> String {
        Self::params_to_vec(params).join("&")
    }
}

// can't test directly for test::Bencher because autocfg lacks support for now,
// so use feature_test which is already a guarantee of running nightly.
#[cfg(all(test, feature_test))]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_params_to_query(b: &mut Bencher) {
        let params_str = (1..10)
            .map(|i| (format!("unakey{}", i), format!("unvalor{}", i)))
            .collect::<Vec<_>>();

        let params = params_str
            .iter()
            .map(|(k, v)| (k.as_str().into(), v.as_str()))
            .collect::<Vec<(Cow<str>, &str)>>();

        b.iter(|| Parameters::params_to_query(&params));
    }
}
