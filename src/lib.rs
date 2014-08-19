#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! A type-based key value store where one value type is allowed for each key.

use std::any::{Any, AnyRefExt, AnyMutRefExt};
use std::intrinsics::TypeId;
use std::collections::HashMap;

/// A map keyed by types.
///
/// Can contain one value of any type for each key type, as defined
/// by the Assoc trait.
pub struct TypeMap {
    data: HashMap<TypeId, Box<Any>>
}

/// This trait defines the relationship between keys and values in TypeMap.
///
/// It is implemented for Keys, with a phantom type parameter for values.
pub trait Assoc<Value> {}

impl TypeMap {
    /// Create a new, empty TypeMap.
    pub fn new() -> TypeMap {
        TypeMap {
            data: HashMap::new()
        }
    }

    /// Insert a value into the map with a specified key type.
    pub fn insert<V: 'static, K: Assoc<V> + 'static>(&mut self, val: V) -> bool {
        self.data.insert(TypeId::of::<K>(), box val as Box<Any>)
    }

    /// Find a value in the map and get a reference to it.
    pub fn find<V: 'static, K: Assoc<V> + 'static>(&self) -> Option<&V> {
        self.data.find(&TypeId::of::<K>()).and_then(|v| v.downcast_ref::<V>())
    }

    /// Find a value in the map and get a mutable reference to it.
    pub fn find_mut<V: 'static, K: Assoc<V> + 'static>(&mut self) -> Option<&mut V> {
        self.data.find_mut(&TypeId::of::<K>()).and_then(|v| v.downcast_mut::<V>())
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
        map.insert::<Value, Key>(Value);
        assert_eq!(*map.find::<Value, Key>().unwrap(), Value);
    }
}

