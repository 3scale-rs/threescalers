use std::prelude::v1::*;

use std::{
    borrow::Cow,
    iter::FromIterator,
    vec::IntoIter,
};

use super::Extension;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct List<'s>(Vec<Extension<'s>>);

impl<'s> From<Vec<Extension<'s>>> for List<'s> {
    fn from(v: Vec<Extension<'s>>) -> Self {
        Self(v)
    }
}

impl<'s> List<'s> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn into_inner(self) -> Vec<Extension<'s>> {
        self.0
    }

    pub fn as_vec(&self) -> &Vec<Extension<'s>> {
        self.0.as_ref()
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<Extension<'s>> {
        self.0.as_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) -> usize {
        let cleared = self.len();
        self.0.clear();
        cleared
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn reserve(mut self, additional: usize) -> Self {
        self.0.reserve(additional);
        self
    }

    pub fn shrink_to_fit(mut self) -> Self {
        self.0.shrink_to_fit();
        self
    }

    pub fn push(mut self, e: Extension<'s>) -> Self {
        self.0.push(e);
        self
    }

    pub fn push_other(self, key: Cow<'s, str>, value: Cow<'s, str>) -> Self {
        self.push(Extension::Other(key, value))
    }

    pub fn remove_item(&mut self, e: &Extension<'s>) -> Option<Extension<'s>> {
        match self.0.iter().position(|elem| elem == e) {
            Some(idx) => Some(self.0.remove(idx)),
            _ => None,
        }
    }

    pub fn remove_all(&mut self, e: &Extension<'s>) -> usize {
        let before = self.len();
        self.0.retain(|elem| elem != e);
        self.len() - before
    }

    pub fn no_body(self) -> Self {
        self.push(Extension::NoBody)
    }

    pub fn hierarchy(self) -> Self {
        self.push(Extension::Hierarchy)
    }

    pub fn flat_usage(self, level: u32) -> Self {
        self.push(Extension::FlatUsage(level.to_string().into()))
    }
}

impl ToString for List<'_> {
    fn to_string(&self) -> String {
        self.0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("&")
    }
}

impl<'s> Extend<Extension<'s>> for List<'s> {
    fn extend<T: IntoIterator<Item = Extension<'s>>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<'s> FromIterator<Extension<'s>> for List<'s> {
    fn from_iter<T: IntoIterator<Item = Extension<'s>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<'v, 's> IntoIterator for &'v List<'s> {
    type IntoIter = <&'v Vec<Extension<'s>> as IntoIterator>::IntoIter;
    type Item = <&'v Vec<Extension<'s>> as IntoIterator>::Item;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'v, 's> IntoIterator for &'v mut List<'s> {
    type IntoIter = <&'v mut Vec<Extension<'s>> as IntoIterator>::IntoIter;
    type Item = <&'v mut Vec<Extension<'s>> as IntoIterator>::Item;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<'s> IntoIterator for List<'s> {
    type IntoIter = IntoIter<Extension<'s>>;
    type Item = Extension<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
