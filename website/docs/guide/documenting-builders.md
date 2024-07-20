# Documenting builders

In regular Rust, it's not possible to place doc comments on function arguments. But with `#[builder]` it is. Documentation written on the arguments will be placed on the generated setter methods.

**Example:**

```rust
use bon::builder;

/// Function that returns a greeting special-tailored for a given person
#[builder]
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
```

::: details How does this work? 🤔

This works because Rust compiler checks for invalid placement of `#[doc = ...]` attributes only after the macro expansion stage. `#[builder]` makes sure to remove the docs from the function's arguments in the expanded code, and instead moves them to the docs on setter methods.

:::

When `#[builder]` is placed on top of a struct, then documentation on the struct fields will be copied to the docs on the setter methods.
