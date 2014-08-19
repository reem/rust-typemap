# `TypeMap`

> A typesafe store keyed by types and containing different types of values.

It provides functionality similar to AnyMap, but is more flexible because it
allows for key-value pairs, rather than enforcing that keys and values are the
same type.

Key-value associations are defined through the `Assoc` trait, which uses a
phantom type parameter and trait coherence rules to enforce the invariants
of `TypeMap`.

