use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    HEAD,
    DELETE,
}

impl Method {
    pub fn as_str(&self) -> &str {
        use Method::*;

        match self {
            GET => "GET",
            POST => "POST",
            PUT => "PUT",
            PATCH => "PATCH",
            HEAD => "HEAD",
            DELETE => "DELETE",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct HeaderMap(HashMap<String, String>);

impl HeaderMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.0.insert(key, value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter { iter: self.0.iter() }
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
pub struct Iter<'a> {
    iter: std::collections::hash_map::Iter<'a, String, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = <std::collections::hash_map::Iter<'a, String, String> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for HeaderMap {
    type IntoIter = <HashMap<String, String> as IntoIterator>::IntoIter;
    type Item = <HashMap<String, String> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S: ToString> std::iter::FromIterator<(S, S)> for HeaderMap {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(iter: T) -> Self {
        let mut map = Self::new();

        map.extend(iter);
        map
    }
}

impl<S: ToString> Extend<(S, S)> for HeaderMap {
    fn extend<T: IntoIterator<Item = (S, S)>>(&mut self, iter: T) {
        for (key, value) in iter.into_iter() {
            self.insert(key.to_string(), value.to_string());
        }
    }
}

impl From<HashMap<String, String>> for HeaderMap {
    fn from(map: HashMap<String, String>) -> Self {
        HeaderMap(map)
    }
}

mod parameters;
pub use self::parameters::Parameters;
pub mod endpoints;
pub mod request;
pub use self::request::Request;
