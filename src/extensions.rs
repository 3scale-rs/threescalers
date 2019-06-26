use std::collections::hash_map::{IntoIter, Iter, IterMut};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Extensions(HashMap<String, String>);

impl Extensions {
    pub fn insert(&mut self, param: String, value: String) -> Option<String> {
        self.0.insert(param, value)
    }

    pub fn remove(&mut self, param: String) -> Option<String> {
        self.0.remove(param.as_str())
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<String, String> {
        self.0.iter_mut()
    }

    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

impl ToString for Extensions {
    fn to_string(&self) -> String {
        use crate::encoding::encode;
        use std::borrow::Cow;

        self.iter()
            .map(|(k, v)| [encode(k), Cow::Borrowed("="), encode(v)].concat())
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl From<HashMap<String, String>> for Extensions {
    fn from(h: HashMap<String, String>) -> Self {
        Self(h)
    }
}

impl Extend<(String, String)> for Extensions {
    fn extend<T: IntoIterator<Item = (String, String)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<(String, String)> for Extensions {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<'a> IntoIterator for &'a Extensions {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Extensions {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for Extensions {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
