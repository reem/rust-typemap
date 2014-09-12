#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! A type-based key value store where one value type is allowed for each key.

extern crate "unsafe-any" as uany;

use std::any::Any;
use std::intrinsics::TypeId;
use std::collections::HashMap;

// These traits are faster when we know the type is correct already.
use uany::{UncheckedAnyDowncast, UncheckedAnyMutDowncast};

/// A map keyed by types.
///
/// Can contain one value of any type for each key type, as defined
/// by the Assoc trait.
pub struct TypeMap {
    data: HashMap<TypeId, Box<Any + 'static>>
}

/// This trait defines the relationship between keys and values in a TypeMap.
///
/// It is implemented for Keys, with a phantom type parameter for values.
pub trait Assoc<Value: 'static>: 'static {}

impl TypeMap {
    /// Create a new, empty TypeMap.
    pub fn new() -> TypeMap {
        TypeMap {
            data: HashMap::new()
        }
    }

    /// Insert a value into the map with a specified key type.
    pub fn insert<K: Assoc<V>, V: 'static>(&mut self, val: V) -> bool {
        self.data.insert(TypeId::of::<K>(), box val as Box<Any>)
    }

    /// Find a value in the map and get a reference to it.
    pub fn find<K: Assoc<V>, V: 'static>(&self) -> Option<&V> {
        self.data.find(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_ref_unchecked::<V>()
        })
    }

    /// Find a value in the map and get a mutable reference to it.
    pub fn find_mut<K: Assoc<V>, V: 'static>(&mut self) -> Option<&mut V> {
        self.data.find_mut(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_mut_unchecked::<V>()
        })
    }

    /// Check if a key has an associated value stored in the map.
    pub fn contains<K: Assoc<V>, V: 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<K>())
    }

    /// Remove a value from the map.
    ///
    /// Returns `true` if a value was removed.
    pub fn remove<K: Assoc<V>, V: 'static>(&mut self) -> bool {
        self.data.remove(&TypeId::of::<K>())
    }

    /// Read the underlying HashMap
    pub unsafe fn data(&self) -> &HashMap<TypeId, Box<Any + 'static>> { &self.data }

    /// Get a mutable reference to the underlying HashMap
    pub unsafe fn data_mut(&mut self) -> &mut HashMap<TypeId, Box<Any + 'static>> { &mut self.data }
}

impl Collection for TypeMap {
    fn len(&self) -> uint {
        self.data.len()
    }
}

impl Mutable for TypeMap {
    fn clear(&mut self) {
        self.data.clear()
    }
}

#[cfg(test)]
mod test {
    use super::{TypeMap, Assoc};

    #[deriving(Show, PartialEq)]
    struct Key;

    #[deriving(Show, PartialEq)]
    struct Value;

    impl Assoc<Value> for Key {}

    #[test] fn test_pairing() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Value);
        assert_eq!(*map.find::<Key, Value>().unwrap(), Value);
        assert!(map.contains::<Key, Value>());
    }

    #[test] fn test_remove() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Value);
        assert!(map.contains::<Key, Value>());
        map.remove::<Key, Value>();
        assert!(!map.contains::<Key, Value>());
    }
}

