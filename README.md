# `TypeMap`

> A typesafe store keyed by types and containing different types of values.

It provides functionality similar to AnyMap, but is more flexible because it
allows for key-value pairs, rather than enforcing that keys and values are the
same type.

Key-value associations are defined through the `Key` trait, which uses an
associated type parameter and trait coherence rules to enforce the invariants
of `TypeMap`.

## Example

```rust
extern crate typemap;
use typemap::{TypeMap, Key};

struct KeyType;

#[derive(Debug, PartialEq)]
struct Value(i32);

impl Key for KeyType { type Value = Value; }

#[test] fn test_pairing() {
    let mut map = TypeMap::new();
    map.insert::<KeyType>(Value(42));
    assert_eq!(*map.get::<KeyType>().unwrap(), Value(42));
}
```

