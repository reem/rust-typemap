#![deny(missing_docs, warnings)]

//! A type-based key value store where one value type is allowed for each key.

extern crate unsafe_any as uany;

use uany::{UnsafeAny, UnsafeAnyExt};
use std::any::{Any, TypeId};
use std::collections::{hash_map, HashMap};
use std::marker::PhantomData;

use Entry::{Occupied, Vacant};

use internals::{Implements, CloneAny};

/// A map keyed by types.
///
/// Can contain one value of any type for each key type, as defined
/// by the Assoc trait.
///
/// You usually do not need to worry about the A type parameter, but it
/// can be used to add bounds to the possible value types that can
/// be stored in this map. Usually, you are looking for `ShareMap`, which
/// is `Send + Sync`.
#[derive(Default)]
pub struct TypeMap<A: ?Sized = UnsafeAny>
where A: UnsafeAnyExt {
    data: HashMap<TypeId, Box<A>>
}

impl<A: ?Sized> Clone for TypeMap<A>
where A: UnsafeAnyExt, Box<A>: Clone { // We are a bit cleverer than derive.
    fn clone(&self) -> TypeMap<A> {
        TypeMap { data: self.data.clone() }
    }
}

/// A version of `TypeMap` containing only `Send` types.
pub type SendMap = TypeMap<UnsafeAny + Send>;

/// A version of `TypeMap` containing only `Sync` types.
pub type SyncMap = TypeMap<UnsafeAny + Sync>;

/// A version of `TypeMap` containing only `Send + Sync` types.
pub type ShareMap = TypeMap<UnsafeAny + Send + Sync>;

/// A version of `TypeMap` containing only `Clone` types.
pub type CloneMap = TypeMap<CloneAny>;

/// A version of `TypeMap` containing only `Clone + Send + Sync` types.
pub type ShareCloneMap = TypeMap<CloneAny + Send + Sync>;

// Assert some properties on SyncMap, SendMap and ShareMap.
fn _assert_types() {
    fn _assert_send<T: Send>() { }
    fn _assert_sync<T: Sync>() { }
    fn _assert_clone<T: Clone>() { }

    _assert_send::<SendMap>();
    _assert_sync::<SyncMap>();
    _assert_send::<ShareMap>();
    _assert_sync::<ShareMap>();
    _assert_clone::<CloneMap>();
}

/// This trait defines the relationship between keys and values in a TypeMap.
///
/// It is implemented for Keys, with a phantom associated type for the values.
pub trait Key: Any {
    /// The value type associated with this key type.
    type Value: Any;
}

impl TypeMap {
    /// Create a new, empty TypeMap.
    pub fn new() -> TypeMap {
        TypeMap::custom()
    }
}

impl<A: UnsafeAnyExt + ?Sized> TypeMap<A> {
    /// Create a new, empty TypeMap.
    ///
    /// Can be used with any `A` parameter; `new` is specialized to get around
    /// the required type annotations when using this function.
    pub fn custom() -> TypeMap<A> {
        TypeMap {
            data: HashMap::new()
        }
    }

    /// Insert a value into the map with a specified key type.
    pub fn insert<K: Key>(&mut self, val: K::Value) -> Option<K::Value>
    where K::Value: Any + Implements<A> {
        self.data.insert(TypeId::of::<K>(), val.into_object()).map(|v| unsafe {
            *v.downcast_unchecked::<K::Value>()
        })
    }

    /// Find a value in the map and get a reference to it.
    pub fn get<K: Key>(&self) -> Option<&K::Value>
    where K::Value: Any + Implements<A> {
        self.data.get(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_ref_unchecked::<K::Value>()
        })
    }

    /// Find a value in the map and get a mutable reference to it.
    pub fn get_mut<K: Key>(&mut self) -> Option<&mut K::Value>
    where K::Value: Any + Implements<A> {
        self.data.get_mut(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_mut_unchecked::<K::Value>()
        })
    }

    /// Check if a key has an associated value stored in the map.
    pub fn contains<K: Key>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<K>())
    }

    /// Remove a value from the map.
    ///
    /// Returns `true` if a value was removed.
    pub fn remove<K: Key>(&mut self) -> Option<K::Value>
    where K::Value: Any + Implements<A> {
        self.data.remove(&TypeId::of::<K>()).map(|v| unsafe {
            *v.downcast_unchecked::<K::Value>()
        })
    }

    /// Get the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<'a, K: Key>(&'a mut self) -> Entry<'a, K, A>
    where K::Value: Any + Implements<A> {
        match self.data.entry(TypeId::of::<K>()) {
            hash_map::Entry::Occupied(e) => Occupied(OccupiedEntry { data: e, _marker: PhantomData }),
            hash_map::Entry::Vacant(e) => Vacant(VacantEntry { data: e, _marker: PhantomData })
        }
    }

    /// Read the underlying HashMap
    pub unsafe fn data(&self) -> &HashMap<TypeId, Box<A>> {
        &self.data
    }

    /// Get a mutable reference to the underlying HashMap
    pub unsafe fn data_mut(&mut self) -> &mut HashMap<TypeId, Box<A>> {
        &mut self.data
    }

    /// Get the number of values stored in the map.
    pub fn len(&self) -> usize {
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
pub enum Entry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
    /// A view onto an occupied entry in a TypeMap.
    Occupied(OccupiedEntry<'a, K, A>),
    /// A view onto an unoccupied entry in a TypeMap.
    Vacant(VacantEntry<'a, K, A>)
}

/// A view onto an occupied entry in a TypeMap.
pub struct OccupiedEntry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
    data: hash_map::OccupiedEntry<'a, TypeId, Box<A>>,
    _marker: PhantomData<K>
}

/// A view onto an unoccupied entry in a TypeMap.
pub struct VacantEntry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
    data: hash_map::VacantEntry<'a, TypeId, Box<A>>,
    _marker: PhantomData<K>
}

impl<'a, K: Key, A: UnsafeAnyExt + ?Sized> OccupiedEntry<'a, K, A> {
    /// Get a reference to the entry's value.
    pub fn get(&self) -> &K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            self.data.get().downcast_ref_unchecked()
        }
    }

    /// Get a mutable reference to the entry's value.
    pub fn get_mut(&mut self) -> &mut K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            self.data.get_mut().downcast_mut_unchecked()
        }
    }

    /// Transform the entry into a mutable reference with the same lifetime as the map.
    pub fn into_mut(self) -> &'a mut K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            self.data.into_mut().downcast_mut_unchecked()
        }
    }

    /// Set the entry's value and return the previous value.
    pub fn insert(&mut self, value: K::Value) -> K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            *self.data.insert(value.into_object()).downcast_unchecked()
        }
    }

    /// Move the entry's value out of the map, consuming the entry.
    pub fn remove(self) -> K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            *self.data.remove().downcast_unchecked()
        }
    }
}

impl<'a, K: Key, A: ?Sized + UnsafeAnyExt> VacantEntry<'a, K, A> {
    /// Set the entry's value and return a mutable reference to it.
    pub fn insert(self, value: K::Value) -> &'a mut K::Value
    where K::Value: Any + Implements<A> {
        unsafe {
            self.data.insert(value.into_object()).downcast_mut_unchecked()
        }
    }
}

mod internals;

#[cfg(test)]
mod test {
    use super::{TypeMap, SendMap, Key};
    use super::Entry::{Occupied, Vacant};

    #[derive(Debug, PartialEq)]
    struct KeyType;

    #[derive(Debug, PartialEq)]
    struct Value(u8);

    impl Key for KeyType { type Value = Value; }

    #[test] fn test_pairing() {
        let mut map = TypeMap::new();
        map.insert::<KeyType>(Value(100));
        assert_eq!(*map.get::<KeyType>().unwrap(), Value(100));
        assert!(map.contains::<KeyType>());
    }

    #[test] fn test_remove() {
        let mut map = TypeMap::new();
        map.insert::<KeyType>(Value(10));
        assert!(map.contains::<KeyType>());
        map.remove::<KeyType>();
        assert!(!map.contains::<KeyType>());
    }

    #[test] fn test_entry() {
        let mut map = TypeMap::new();
        map.insert::<KeyType>(Value(20));
        match map.entry::<KeyType>() {
            Occupied(e) => {
                assert_eq!(e.get(), &Value(20));
                assert_eq!(e.remove(), Value(20));
            },
            _ => panic!("Unable to locate inserted item.")
        }
        assert!(!map.contains::<KeyType>());
        match map.entry::<KeyType>() {
            Vacant(e) => {
                e.insert(Value(2));
            },
            _ => panic!("Found non-existant entry.")
        }
        assert!(map.contains::<KeyType>());
    }

    #[test] fn test_custom_bounds() {
        let mut map: SendMap = TypeMap::custom();
        map.insert::<KeyType>(Value(10));
        assert!(map.contains::<KeyType>());
        map.remove::<KeyType>();
        assert!(!map.contains::<KeyType>());
    }
}
