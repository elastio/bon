# Alternatives

There are several other existing alternative crates that generate builders. `bon` was designed as a logical evolution of all those crates. Here we'll review them and discuss how `bon` does some things better or some things differently than other implementations.

## [`derive-builder`](https://docs.rs/derive_builder)

### Feature coverage

- 🟢 Builder for structs
- 🔴 Builder for free functions
- 🔴 Builder for associated methods

### Panic safety

The main problem of the `derive-builder` crate is that it generates panic-unsafe builders that may panic at runtime if the developer doesn't fill a required field, for example.

The [first code example](https://github.com/colin-kiegel/rust-derive-builder#how-it-works) in the repo that you see features a `.build().unwrap()`. This design makes it prone to hard-to-detect bugs where the builder is used in cold code paths (such as error handling) which aren't covered by tests.

### Ergonomics

`Option` fields aren't optional by default. You need to add `#[builder(default)]` for them explicitly.

There are no automatic `Into` conversions. You need to add `#[builder(setter(into))]` to each field explicitly.

There is no `T::builder()` method generated. To create a builder the caller need to use `Builder::default()` instead.

By default generates a builder that passes `self` via `&mut` reference through the setters. The `build()` method takes `&self` and requires all fields to be `Clone`. This doesn't look like a good choice by default because there is a default requirement of `Clone` for all fields. Plus, this may have a performance impact, although [the docs](https://docs.rs/derive_builder/latest/derive_builder/#-performance-considerations) assure that compiler can optimize all the clones.

There is a way to make the builder take `self` by value and avoid cloning, but it's opt-in via `#[builder(pattern = "owned")]`.

### Compatibility

Changing field from required to optional is not backwards compatible in
the setter signature. There is just one setter generated for every field, so when you make a required field optional, the setter changes its parameter type from `T` to `Option<T>`, which is breaking API change.

### Advantages

The generated builder is **very** simple. It's just a struct equivalent to the input struct but with all fields wrapped in `Option`. There are no additional typestate generics attached to the builder struct.

This design makes it possible to add own custom methods to the generated builder struct by writing an `impl Builder {}` block, or even defining custom setters on the builder that override the default generated ones.


The fact that setters don't change the type of the builder allows for using `&mut self` to set the values without consuming the builder.


```rust
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone, Debug)]
struct NodeTypedBuilder<'a, T> {
    /// My docs
    u32: u32,

    #[builder(setter(into))]
    string: String,

    #[builder(default)]
    optional_string: Option<String>,

    #[builder(default)]
    next: Option<Box<NodeTypedBuilder<'a, T>>>,

    val: &'a T,
}

/// My docs on struct
struct NodeBuildstructor {
    /// My docs
    u32: u32,
    string: String,
    optional_string: Option<String>,
    next: Option<Box<NodeBuildstructor>>,
    val: bool,
}

#[buildstructor::buildstructor]
impl NodeBuildstructor {
    #[builder(entry = "func_syntax", visibility = "pub")]
    fn func_syntax_positional<'a>(
        u32: &'a u32,
        string: String,
        optional_string: Option<String>,
        val: Option<bool>,
    ) -> u32 {
        let val = val.unwrap_or(false);

        32
    }
}

fn main() {
    let _output = NodeTypedBuilder::builder()
        .u32(32)
        .string("hello")
        // .optional_string(optional_string)
        .val(&false)
        .build();

    let _output = NodeBuildstructor::func_syntax()
        .u32(&32)
        // .string()
        // .val(false)
        .string("")
        .build();
}

/*
# `derive-builder`

1. Errors in `build()`, checking of required fields moved to runtime.
May result in panics.
2. Non-default `#[builder(default)]` for `Option` fields
3. Non-default `#[builder(setter(into))]` for `String` fields
4. Changing field from required to optional is not backwards compatible in
the setter signature `required(T)` -> `optional(Option<T>)`
5. Incorrect handling of `Self` within the struct declaration.
6. Doesn't generate `T::builder()`
7. Supports only structs as input.

# `typed-builder`

Solves only 1, 6 from above.

# `buildstructor`

Solves only 1, 2, 3, 4

8. Doesn't support free functions as builders.
9. Doesn't properly work with references and doesn't normalize `Self` usages.
10. Doesn't support documentation for setters.
11. Positional function is still available under the same visibility by default.
12. Doesn't support impl Trait syntax in functions.

*/
```
