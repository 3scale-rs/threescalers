use std::{
    collections::{
        hash_map::{
            IntoIter,
            Iter,
            IterMut,
        },
        HashMap,
    },
    iter::FromIterator,
};

#[derive(Debug, Clone)]
pub struct Extensions(HashMap<String, String>);

impl Extensions {
    pub fn new() -> Self {
        Extensions(HashMap::new())
    }

    pub fn insert<S: Into<String>>(&mut self, param: S, value: S) -> Option<String> {
        self.0.insert(param.into(), value.into())
    }

    pub fn remove<R: AsRef<String>, S: Into<R>>(&mut self, param: S) -> Option<String> {
        self.0.remove(param.into().as_ref())
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
    type IntoIter = Iter<'a, String, String>;
    type Item = (&'a String, &'a String);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Extensions {
    type IntoIter = IterMut<'a, String, String>;
    type Item = (&'a String, &'a mut String);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for Extensions {
    type IntoIter = IntoIter<String, String>;
    type Item = (String, String);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
