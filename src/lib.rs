#![feature(associated_types)]
#![deny(missing_docs, warnings)]

//! A type-based key value store where one value type is allowed for each key.

extern crate "unsafe-any" as uany;

use uany::{UnsafeAny};
use std::intrinsics::TypeId;
use std::collections::{hash_map, HashMap};

use Entry::{Occupied, Vacant};

/// A map keyed by types.
///
/// Can contain one value of any type for each key type, as defined
/// by the Assoc trait.
#[derive(Default)]
pub struct TypeMap {
    data: HashMap<TypeId, Box<UnsafeAny + 'static>>
}

/// This trait defines the relationship between keys and values in a TypeMap.
///
/// It is implemented for Keys, with a phantom associated type for the values.
pub trait Key: 'static { type Value: 'static; }

impl TypeMap {
    /// Create a new, empty TypeMap.
    pub fn new() -> TypeMap {
        TypeMap {
            data: HashMap::new()
        }
    }

    /// Insert a value into the map with a specified key type.
    pub fn insert<K: Key>(&mut self, val: <K as Key>::Value) -> Option<<K as Key>::Value> {
        self.data.insert(TypeId::of::<K>(), box val).map(|v| unsafe {
            *v.downcast_unchecked::<<K as Key>::Value>()
        })
    }

    /// Find a value in the map and get a reference to it.
    #[deprecated = "renamed to `get`"]
    pub fn find<K: Key>(&self) -> Option<&<K as Key>::Value> {
        self.data.get(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_ref_unchecked::<<K as Key>::Value>()
        })
    }

    /// Find a value in the map and get a mutable reference to it.
    #[deprecated = "renamed to `get_mut`"]
    pub fn find_mut<K: Key>(&mut self) -> Option<&mut <K as Key>::Value> {
        self.data.get_mut(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_mut_unchecked::<<K as Key>::Value>()
        })
    }

    /// Find a value in the map and get a reference to it.
    pub fn get<K: Key>(&self) -> Option<&<K as Key>::Value> {
        self.data.get(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_ref_unchecked::<<K as Key>::Value>()
        })
    }

    /// Find a value in the map and get a mutable reference to it.
    pub fn get_mut<K: Key>(&mut self) -> Option<&mut <K as Key>::Value> {
        self.data.get_mut(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_mut_unchecked::<<K as Key>::Value>()
        })
    }

    /// Check if a key has an associated value stored in the map.
    pub fn contains<K: Key>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<K>())
    }

    /// Remove a value from the map.
    ///
    /// Returns `true` if a value was removed.
    pub fn remove<K: Key>(&mut self) -> Option<<K as Key>::Value> {
        self.data.remove(&TypeId::of::<K>()).map(|v| unsafe {
            *v.downcast_unchecked::<<K as Key>::Value>()
        })
    }

    /// Get the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<'a, K: Key>(&'a mut self) -> Entry<'a, K> {
        match self.data.entry(TypeId::of::<K>()) {
            hash_map::Entry::Occupied(e) => Occupied(OccupiedEntry { data: e }),
            hash_map::Entry::Vacant(e) => Vacant(VacantEntry { data: e })
        }
    }

    /// Read the underlying HashMap
    pub unsafe fn data(&self) -> &HashMap<TypeId, Box<UnsafeAny + 'static>> {
        &self.data
    }

    /// Get a mutable reference to the underlying HashMap
    pub unsafe fn data_mut(&mut self) -> &mut HashMap<TypeId, Box<UnsafeAny + 'static>> {
        &mut self.data
    }

    /// Get the number of values stored in the map.
    pub fn len(&self) -> uint {
        self.data.len()
    }

    /// Return true if the map contains no values.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove all entries from the map.
    pub fn clear(&mut self) {
        self.data.clear()
    }
}

/// A view onto an entry in a TypeMap.
pub enum Entry<'a, K> {
    /// A view onto an occupied entry in a TypeMap.
    Occupied(OccupiedEntry<'a, K>),
    /// A view onto an unoccupied entry in a TypeMap.
    Vacant(VacantEntry<'a, K>)
}

/// A view onto an occupied entry in a TypeMap.
pub struct OccupiedEntry<'a, K> {
    data: hash_map::OccupiedEntry<'a, TypeId, Box<UnsafeAny + 'static>>
}

/// A view onto an unoccupied entry in a TypeMap.
pub struct VacantEntry<'a, K> {
    data: hash_map::VacantEntry<'a, TypeId, Box<UnsafeAny + 'static>>
}

impl<'a, K: Key> OccupiedEntry<'a, K> {
    /// Get a reference to the entry's value.
    pub fn get(&self) -> &<K as Key>::Value {
        unsafe {
            self.data.get().downcast_ref_unchecked()
        }
    }

    /// Get a mutable reference to the entry's value.
    pub fn get_mut(&mut self) -> &mut <K as Key>::Value {
        unsafe {
            self.data.get_mut().downcast_mut_unchecked()
        }
    }

    /// Transform the entry into a mutable reference with the same lifetime as the map.
    pub fn into_mut(self) -> &'a mut <K as Key>::Value {
        unsafe {
            self.data.into_mut().downcast_mut_unchecked()
        }
    }

    /// Set the entry's value and return the previous value.
    pub fn set(&mut self, value: <K as Key>::Value) -> <K as Key>::Value {
        unsafe {
            *self.data.set(box value).downcast_unchecked()
        }
    }

    /// Move the entry's value out of the map, consuming the entry.
    pub fn take(self) -> <K as Key>::Value {
        unsafe {
            *self.data.take().downcast_unchecked()
        }
    }
}

impl<'a, K: Key> VacantEntry<'a, K> {
    /// Set the entry's value and return a mutable reference to it.
    pub fn set(self, value: <K as Key>::Value) -> &'a mut <K as Key>::Value {
        unsafe {
            self.data.set(box value).downcast_mut_unchecked()
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TypeMap, Assoc};
    use super::Entry::{Occupied, Vacant};

    #[derive(Show, PartialEq)]
    struct Key;

    #[derive(Show, PartialEq)]
    struct Value;

    impl Assoc<Value> for Key {}

    #[test] fn test_pairing() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Value);
        assert_eq!(*map.get::<Key, Value>().unwrap(), Value);
        assert!(map.contains::<Key, Value>());
    }

    #[test] fn test_remove() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Value);
        assert!(map.contains::<Key, Value>());
        map.remove::<Key, Value>();
        assert!(!map.contains::<Key, Value>());
    }

    #[test] fn test_entry() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Value);
        match map.entry::<Key, Value>() {
            Occupied(e) => {
                assert_eq!(e.get(), &Value);
                assert_eq!(e.take(), Value);
            },
            _ => panic!("Unable to locate inserted item.")
        }
        assert!(!map.contains::<Key, Value>());
        match map.entry::<Key, Value>() {
            Vacant(e) => {
                e.set(Value);
            },
            _ => panic!("Found non-existant entry.")
        }
        assert!(map.contains::<Key, Value>());
    }

    #[test] fn test_entry_multi() {
        impl Assoc<f64> for Key {}
        impl Assoc<u32> for Key {}

        let mut map = TypeMap::new();
        map.insert::<Key, u32>(44);
        map.insert::<Key, Value>(Value);
        if let Occupied(_) = map.entry::<Key, f64>() {
            panic!("Unsound")
        }

        assert_eq!(*map.get::<Key, Value>().unwrap(), Value);
        map.remove::<Key, Value>();
        assert!(!map.contains::<Key, Value>());

        assert_eq!(*map.get::<Key, u32>().unwrap(), 44);
        map.remove::<Key, u32>();
        assert!(!map.contains::<Key, u32>());
    }
}

