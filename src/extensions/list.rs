use std::prelude::v1::*;

use std::{borrow::Cow, iter::FromIterator, vec::IntoIter};

use super::Extension;

/// A list of extensions for a 3scale API call.
///
/// `List` provides a builder-like API for constructing a set of extensions to include
/// in an API request. Most methods are chainable and return `self` to allow fluent construction.
///
/// # Examples
///
/// ```
/// use threescalers::extensions::{List, Extension};
///
/// let extensions = List::new()
///     .hierarchy()
///     .no_body();
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct List<'s>(Vec<Extension<'s>>);

impl<'s> From<Vec<Extension<'s>>> for List<'s> {
    fn from(v: Vec<Extension<'s>>) -> Self {
        Self(v)
    }
}

impl<'s> List<'s> {
    /// Creates a new empty list of extensions.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new list with space for at least `capacity` extensions.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Consumes this list and returns the underlying vector of extensions.
    pub fn into_inner(self) -> Vec<Extension<'s>> {
        self.0
    }

    /// Returns a reference to the underlying vector of extensions.
    pub fn as_vec(&self) -> &Vec<Extension<'s>> {
        self.0.as_ref()
    }

    /// Returns a mutable reference to the underlying vector of extensions.
    pub fn as_mut_vec(&mut self) -> &mut Vec<Extension<'s>> {
        self.0.as_mut()
    }

    /// Returns the number of extensions in this list.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this list contains no extensions.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Removes all extensions from the list and returns the number that were removed.
    pub fn clear(&mut self) -> usize {
        let cleared = self.len();
        self.0.clear();
        cleared
    }

    /// Returns the number of extensions this list can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Reserves space for at least `additional` more extensions.
    pub fn reserve(mut self, additional: usize) -> Self {
        self.0.reserve(additional);
        self
    }

    /// Shrinks the capacity of this list to match its current length.
    pub fn shrink_to_fit(mut self) -> Self {
        self.0.shrink_to_fit();
        self
    }

    /// Appends an extension to the list and returns self for chaining.
    pub fn push(mut self, e: Extension<'s>) -> Self {
        self.0.push(e);
        self
    }

    /// Appends a custom extension with the given key and value.
    pub fn push_other(self, key: Cow<'s, str>, value: Cow<'s, str>) -> Self {
        self.push(Extension::Other(key, value))
    }

    /// Removes the first occurrence of the given extension from the list.
    ///
    /// Returns `Some` containing the removed extension, or `None` if the extension was not found.
    pub fn remove_item(&mut self, e: &Extension<'s>) -> Option<Extension<'s>> {
        match self.0.iter().position(|elem| elem == e) {
            Some(idx) => Some(self.0.remove(idx)),
            _ => None,
        }
    }

    /// Removes all occurrences of the given extension from the list.
    ///
    /// Returns the number of extensions that were removed.
    pub fn remove_all(&mut self, e: &Extension<'s>) -> usize {
        let before = self.len();
        self.0.retain(|elem| elem != e);
        // side-effect free: length before retain is >= self.len()
        before - self.len()
    }

    /// Adds a `no_body` extension to request that the response body be omitted.
    pub fn no_body(self) -> Self {
        self.push(Extension::NoBody)
    }

    /// Adds a `hierarchy` extension to request the metrics hierarchy in the response.
    pub fn hierarchy(self) -> Self {
        self.push(Extension::Hierarchy)
    }

    /// Adds a `flat_usage` extension with the specified nesting level.
    pub fn flat_usage(self, level: u32) -> Self {
        self.push(Extension::FlatUsage(level.to_string().into()))
    }

    /// Adds a `list_app_keys` extension with the specified level.
    pub fn list_app_keys(self, level: u32) -> Self {
        self.push(Extension::ListAppKeys(level.to_string().into()))
    }
}

impl ToString for List<'_> {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("&")
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
