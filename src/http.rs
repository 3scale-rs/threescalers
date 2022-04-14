use std::prelude::v1::*;

use std::collections::{btree_map::Iter as InnerIter, BTreeMap};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderMap(BTreeMap<String, String>);

impl HeaderMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    // There's no provision for this method in the inner map
    pub fn with_capacity(_: usize) -> Self {
        Self::new()
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
        Iter {
            iter: self.0.iter(),
        }
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    iter: InnerIter<'a, String, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = <InnerIter<'a, String, String> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for HeaderMap {
    type IntoIter = <BTreeMap<String, String> as IntoIterator>::IntoIter;
    type Item = <BTreeMap<String, String> as IntoIterator>::Item;

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

impl From<BTreeMap<String, String>> for HeaderMap {
    fn from(map: BTreeMap<String, String>) -> Self {
        HeaderMap(map)
    }
}

mod parameters;
pub use self::parameters::Parameters;
pub mod endpoints;
pub mod request;
pub use self::request::Request;

#[cfg(feature = "rest-mappings")]
pub mod mapping_rule;
