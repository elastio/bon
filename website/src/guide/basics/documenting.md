# Documenting

In regular Rust, it's not possible to place doc comments on function arguments. But with `#[builder]` it is. Documentation written on the arguments will be placed on the generated setter methods.

````rust
/// Function that returns a greeting special-tailored for a given person
#[bon::builder]
fn greet(
    /// Name of the person to greet.
    ///
    /// **Example:**
    /// ```
    /// greet().name("John");
    /// ```
    name: &str,

    /// Age expressed in full years passed since the birth date.
    age: u32
) -> String {
    format!("Hello {name} with age {age}!")
}
````

::: details How does this work? 🤔

This works because Rust compiler checks for invalid placement of `#[doc = ...]` attributes only after the macro expansion stage. `#[builder]` removes the docs from the function's arguments in the expanded code, and instead moves them to the docs on setter methods.

:::

When `#[derive(Builder)]` is placed on top of a struct, then documentation on the struct fields will be copied to the docs on the setter methods.

## Custom `doc` Attributes

You can override documentation on other items generated by builder macros. There are multiple attributes accepting a `doc { ... }` block.

::: code-group

```rust [Struct]
#[derive(bon::Builder)]
#[builder(
    builder_type(doc {
        /// Custom docs on the builder struct itself
    }),
    finish_fn(doc {
        /// Custom docs on the finishing function
    }),
    // ...
)]
struct Example {}
```

```rust [Function]
#[bon::builder(
    builder_type(doc {
        /// Custom docs on the builder struct itself
    }),
    finish_fn(doc {
        /// Custom docs on the finishing function
    }),
    // ...
)]
fn example() {}
```

```rust [Method]
struct Example;

#[bon::bon]
impl Example {
    #[builder(
        builder_type(doc {
            /// Custom docs on the builder struct itself
        }),
        finish_fn(doc {
            /// Custom docs on the finishing function
        }),
        // ...
    )]
    fn example() {}
}
```

:::

You can document the following items this way:

| Attribute                                                            | Documentation target                                                         |
| -------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| [`builder_type`](../../reference/builder/top-level/builder_type#doc) | Builder struct                                                               |
| [`start_fn`](../../reference/builder/top-level/start_fn#doc)         | Starting function                                                            |
| [`finish_fn`](../../reference/builder/top-level/finish_fn#doc)       | Finishing function                                                           |
| [`state_mod`](../../reference/builder/top-level/state_mod#doc)       | Builder state API module (more details in [Typestate API](../typestate-api)) |
| [`setters`](../../reference/builder/member/setters#doc)              | Custom docs for setters. Prevents copying them from the field/argument       |
| [`getter`](../../reference/builder/member/getter#doc)                | Custom docs for a getter. Prevents copying them from the field/argument      |

## Positional Members

Documentation comments are allowed on [positional members](./positional-members). However, since there are no separate setter methods generated for them, the docs on these members will not be copied anywhere, and thus they won't appear in `rustdoc`. Instead, it's recommended to write documentation for these members on the top level of the struct or function.
