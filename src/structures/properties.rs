use std::{
    collections::{HashMap, hash_map},
    iter::{Filter, FilterMap},
};

use serde::{Deserialize, Serialize};

use crate::{color::spaces::ColorSpace, geometry};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Value not set
    #[default]
    None,
    Int(i32),
    Int64(i64),
    Double(f32),
    String(String),
    Position(geometry::Position),
    Rect(geometry::Rect),
    Buffer(Vec<u8>),
    Color(ColorSpace),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Properties {
    inner: HashMap<String, PropertyValue>,
}

impl Properties {
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.inner.get(key)
    }

    pub fn keys(&self) -> hash_map::Keys<'_, String, PropertyValue> {
        self.inner.keys()
    }

    pub fn values(&self) -> hash_map::Values<'_, String, PropertyValue> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, String, PropertyValue> {
        self.inner.values_mut()
    }

    pub fn find<Predicate>(&self, predicate: Predicate) -> Option<(&String, &PropertyValue)>
    where
        Predicate: for<'a> FnMut(&'a (&String, &PropertyValue)) -> bool,
    {
        self.inner.iter().find(predicate)
    }

    pub fn filter<Predicate>(
        &self,
        predicate: Predicate,
    ) -> Filter<hash_map::Iter<'_, String, PropertyValue>, Predicate>
    where
        Predicate: for<'a> FnMut(&'a (&String, &PropertyValue)) -> bool,
    {
        self.inner.iter().filter(predicate)
    }

    pub fn filter_mut<Predicate>(
        &self,
        predicate: Predicate,
    ) -> FilterMap<hash_map::Iter<'_, String, PropertyValue>, Predicate>
    where
        Predicate: for<'a> FnMut((&String, &PropertyValue)) -> Option<PropertyValue>,
    {
        self.inner.iter().filter_map(predicate)
    }

    pub fn set(&mut self, key: &str, value: PropertyValue) -> &Self {
        self.inner.insert(key.to_string(), value);
        self
    }

    pub fn remove(&mut self, key: &str) -> Option<PropertyValue> {
        self.inner.remove(key)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
