# `TypeMap`

> A typesafe store keyed by types and containing different types of values.

It provides functionality similar to AnyMap, but is more flexible because it
allows for key-value pairs, rather than enforcing that keys and values are the
same type.

Key-value associations are defined through the `Assoc` trait, which uses a
phantom type parameter and trait coherence rules to enforce the invariants
of `TypeMap`.

## Example

```rust
#[deriving(Show, PartialEq)]
struct Key;

#[deriving(Show, PartialEq)]
struct Value;

impl Assoc<Value> for Key {}

#[test] fn test_pairing() {
    let mut map = TypeMap::new();
    map.insert::<Key, Value>(Value);
    assert_eq!(*map.find::<Key, Value>().unwrap(), Value);
}
```

