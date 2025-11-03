use std::{
    collections::{HashMap, hash_map},
    iter::{Filter, FilterMap},
};

use serde::{Deserialize, Serialize};

use crate::{color::spaces::ColorSpace, geometry};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[repr(C)]
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
#[repr(C)]
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

pub mod ffi {
    use super::*;
    use std::collections::HashMap;
    use std::os::raw::c_char;

    #[repr(C)]
    pub struct PropertyPair {
        key: *const c_char,
        value: *const PropertyValue,
    }

    pub struct PropertyMap(pub HashMap<String, PropertyValue>);

    #[unsafe(no_mangle)]
    pub extern "C" fn create_property_map() -> *mut PropertyMap {
        let map = PropertyMap(HashMap::new());
        Box::into_raw(Box::new(map))
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn insert_property(
        map: *mut PropertyMap,
        key: *const c_char,
        value: *const PropertyValue,
    ) {
        if map.is_null() || key.is_null() || value.is_null() {
            return;
        }
        unsafe {
            let map = &mut *map;
            if let Ok(key_str) = std::ffi::CStr::from_ptr(key).to_str() {
                let value = (*value).clone();
                map.0.insert(key_str.to_string(), value);
            }
        }
    }

    pub extern "C" fn remove_property(map: *mut PropertyMap, key: *const c_char) {
        if map.is_null() || key.is_null() {
            return;
        }
        unsafe {
            let map = &mut *map;
            if let Ok(key_str) = std::ffi::CStr::from_ptr(key).to_str() {
                map.0.remove(key_str);
            }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn clear_property_map(map: *mut PropertyMap) {
        if !map.is_null() {
            unsafe {
                (*map).0.clear();
            }
        }
    }
}
