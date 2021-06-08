use std::prelude::v1::*;

use std::{
    collections::{
        btree_map::{Iter, IterMut},
        BTreeMap,
    },
    fmt,
};

use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize,
};

// We might want to consider moving from a BTreeMap to a Vec, as most of the time this btreemap will
// contain a (very) small number of entries.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetricsHierarchy(BTreeMap<String, Vec<String>>);

impl MetricsHierarchy {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert<S: Into<String>, V: Into<Vec<String>>>(
        &mut self,
        parent_metric: S,
        children_metrics: V,
    ) -> Option<Vec<String>> {
        self.0.insert(parent_metric.into(), children_metrics.into())
    }

    pub fn remove<S: AsRef<str>>(&mut self, parent_metric: S) -> Option<Vec<String>> {
        self.0.remove(parent_metric.as_ref())
    }

    pub fn iter(&self) -> Iter<String, Vec<String>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<String, Vec<String>> {
        self.0.iter_mut()
    }

    pub fn into_inner(self) -> BTreeMap<String, Vec<String>> {
        self.0
    }

    /// Retrieves the parent metric of a given metric. Note that Apisonator metrics have 0 or
    /// 1 parent metrics, not multiple.
    pub fn parent_of(&self, metric_name: &str) -> Option<&str> {
        self.iter().find_map(|(parent, v)| {
            if v.iter().any(|child| metric_name == child) {
                Some(parent.as_str())
            } else {
                None
            }
        })
    }
}

struct MetricsHierarchyVisitor;

impl<'de> Visitor<'de> for MetricsHierarchyVisitor {
    type Value = MetricsHierarchy;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a structure that represents a hierarchy of metrics")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut hierarchy = MetricsHierarchy::new();

        // The key in the hierarchy structure is always "metric". It is not
        // used, but we need to read it to get the value.
        while map.next_key::<String>()?.is_some() {
            let val: BTreeMap<String, String> = map.next_value()?;

            let parent_metric = val
                .get("name")
                .ok_or_else(|| de::Error::missing_field("name"))?;
            let children_metrics = val
                .get("children")
                .ok_or_else(|| de::Error::missing_field("children"))?
                .split(' ')
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            hierarchy.insert(parent_metric.to_owned(), children_metrics);
        }

        Ok(hierarchy)
    }
}

impl<'de> Deserialize<'de> for MetricsHierarchy {
    fn deserialize<D>(deserializer: D) -> Result<MetricsHierarchy, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MetricsHierarchyVisitor)
    }
}
