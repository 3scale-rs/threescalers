use std::borrow::Cow;
use http::Method;

#[derive(Clone, Debug)]
pub enum RequestParameters {
    Query(String),
    Body(String),
}

use std::iter::Map;
use std::slice::Iter;

// This newtype is currently only used internally here, but we might want to move it elsewhere where
// it could be more useful because of genericity. We could also aim at reducing the amount of
// conversions in requests by having a type that only maps parameters once unless changed.
type ParamsMapper<'a, S, T> = Map<Iter<'a, (S, S)>, fn(&(S, S)) -> T>;

impl RequestParameters {
    pub fn new<S: AsRef<str>>(method: &Method, params: &[(S, S)]) -> Self {
        let params_s = Self::params_to_query(params);

        if Self::method_requires_body(method) {
            RequestParameters::Body(params_s)
        } else {
            RequestParameters::Query(params_s)
        }
    }

    #[inline]
    fn method_requires_body(method: &Method) -> bool {
        match *method {
            Method::GET | Method::HEAD | Method::DELETE => false,
            _ => true,
        }
    }

    pub fn path_and_query<'p>(&self, path: &'p str) -> Cow<'p, str> {
        self.query().map_or_else(|| {
            Cow::Borrowed(path)
        }, |q| {
            let mut url = path.to_string();
            url.push('?');
            url.push_str(q);
            Cow::Owned(url)
        })
    }

    pub fn query(&self) -> Option<&str> {
        match self {
            RequestParameters::Query(query) => Some(query.as_str()),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&str> {
        match self {
            RequestParameters::Body(body) => Some(body.as_str()),
            _ => None,
        }
    }

    pub fn into_inner(self) -> String {
        match self {
            RequestParameters::Query(s) => s,
            RequestParameters::Body(s) => s,
        }
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        match self {
            RequestParameters::Query(ref mut query) => query,
            RequestParameters::Body(ref mut body) => body,
        }
    }

    pub fn query_as_mut_string(&mut self) -> Option<&mut String> {
        match self {
            RequestParameters::Query(ref mut query) => Some(query),
            _ => None,
        }
    }

    pub fn body_as_mut_string(&mut self) -> Option<&mut String> {
        match self {
            RequestParameters::Body(ref mut body) => Some(body),
            _ => None,
        }
    }

    pub fn push<S: AsRef<str>>(&mut self, extra_params: &[(S, S)]) {
        let q = Self::params_to_query(extra_params);
        let s = self.as_mut_string();

        if !s.is_empty() {
            s.push('&');
        }

        s.push_str(q.as_str());
    }

    fn params_to_string_collection<S: AsRef<str>>(params: &[(S, S)]) -> ParamsMapper<S, String> {
        params.iter().map(|(k, v)| {
            //[self::encoding::encode(k.as_ref()), Cow::Borrowed("="), self::encoding::encode(v.as_ref())].concat()
            [k.as_ref(), "=", v.as_ref()].concat()
        })
    }

    fn params_to_vec<S: AsRef<str>>(params: &[(S, S)]) -> Vec<String> {
        Self::params_to_string_collection(params).collect()
    }

    fn params_to_query<S: AsRef<str>>(params: &[(S, S)]) -> String {
        Self::params_to_vec(params).join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_params_to_query(b: &mut Bencher) {
        let params = (1..10).map(|i| {
            (format!("unakey{}", i), format!("unvalor{}", i))
        }).collect::<Vec<_>>();

        b.iter(||
            RequestParameters::params_to_query(&params)
        );
    }
}
