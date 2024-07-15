# Alternatives

There are several other existing alternative crates that generate builders. `bon` was designed as a logical evolution of all those crates. Here we'll review them and discuss how `bon` does some things better or some things differently than other implementations.

TODO: convert the following experiments into a documentation

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
