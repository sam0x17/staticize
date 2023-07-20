# Staticize

Staticize provides a trait called `Staticize` which provides a handy associated type `Static`
which provides a `'static` version of `T` for all `T` that implement `Staticize`. `Staticize`
is implemented on all primitives, as well as references, tuples up to size 16, arrays, and
slices of any `T` that implements `Staticize`.

## Use Cases

Staticize is useful for situations where you have a `T` but for whatever reason need a
`'static` version of it, such as when you are working with type-erased heap allocations, as is
the case with my [interned](https://crates.io/crates/interned) crate. Another common use-case
is situations where a static version of a type is needed for some generic method, such as
`Typeid::of`.

For example, a method that takes a value of type `T`, stores it in a static on the heap, and
returns a `'static` reference to the static version of the data on the heap, might use the
following signature:

```rust
pub fn heap_allocate<T: Staticize>(val: T) -> T::Static {
  // ...
}
```

## Features

Two convenience methods, `static_type_id` and `static_type_name` are also provided on
`Staticize`. These use the facilities in `core::any` to return the underlying `TypeId` and
name (as a `&'static str`) of the _static_ version of `T`.

Staticize is completely `no_std`, so it can be used in exotic scenarios where the standard
library is not available, such as embedded devices or in WASM.
