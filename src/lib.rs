#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A type-based key value store where one value type is allowed for each key.

use std::any::{Any, AnyRefExt, AnyMutRefExt};
use std::intrinsics::TypeId;
use std::collections::HashMap;

pub struct TypeMap {
    data: HashMap<TypeId, Box<Any>>
}

pub trait Assoc<Value> {}

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap {
            data: HashMap::new()
        }
    }

    pub fn insert<V: 'static, K: Assoc<V> + 'static>(&mut self, val: V) -> bool {
        self.data.insert(TypeId::of::<K>(), box val as Box<Any>)
    }

    pub fn find<V: 'static, K: Assoc<V> + 'static>(&self) -> Option<&V> {
        self.data.find(&TypeId::of::<K>()).and_then(|v| v.downcast_ref::<V>())
    }

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

