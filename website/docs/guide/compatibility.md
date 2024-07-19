# Compatibility

## Moving `#[builder]` from the struct the `new()` method

`#[builder]` on a struct generates builder API that is fully compatible with placing `#[builder]` on the `new()` method with the signature similar to struct's fields

This means, for example, it's preferable to place the `#[builder]` attribute on top of your struct in most cases because it's convenient. However, if you need to have some custom logic during the construction of your type, you may simply create a `new()` method annotated with `#[builder]` where you can do anything you want to create an instance of your type.

To keep type's public API compatible with the time when `#[builder]` was on the struct directly, the `new()` method must accept the same parameters as there were fields on the struct.

**Example:**

```rust
use bon::bon;

struct User {
    // Suppose we decided to change the internal representation // [!code highlight]
    // of the `id` field of the struct to use `String`          // [!code highlight]
    id: String,                                                 // [!code highlight]
    name: String,
}

#[bon] // [!code highlight]
impl User {
    #[builder] // [!code highlight]
    fn new(id: u32, name: String) -> Self {
        Self {
            id: format!("u-{id}"),
            name: String,
        }
    }
}

// This code still compiles since the API of the builder didn't change // [!code highlight]
let user = User::builder()
    // `id` is still accepted as a `u32` here
    .id(1)
    .name("Bon")
    .build();

assert_eq!(user.id, "u-1");
assert_eq!(user.name, "Bon");
```

## Adding #[builder] to existing code

If your existing code defines functions with positional parameters in its public API that you'd like to change to use builder syntax, but you want to keep the old code compatible with the positional functions API, then you may use `#[builder(expose_positional_fn)]` attribute to keep both syntaxes available. See [this attribute's docs](../reference/builder#expose-positional-fn) for details.
