# Optional setters

Setters generated for function arguments or struct fields of `Option<T>` type are themselves optional to call. If they aren't invoked, then `None` will be used as the default.

**Example:**

```rust
use bon::builder;

#[builder]
fn example(level: Option<u32>) {}

// We can call it without specifying `level`
example().call();
```

The setters API generated for this is the following (simplified):

```rust ignore
impl ExampleBuilder {
    // Accepts the underlying value. Wraps it in `Some()` internally
    fn level(value: u32) -> Self { /* */ }

    // Accepts the `Option` directly.
    fn maybe_level(value: Option<u32>) -> Self { /* */ }
}
```

This allows for the following call patterns.

If you need to pass a simple literal value, then the syntax is very short

```rust ignore
example().level(42).call();
```

If you have an `Option` variable somewhere or you need to dynamically decide if the value should be `Some` or `None`, then you can use the `maybe_` variant of the setter.

```rust ignore
let value = if 1 + 1 == 2 {
    Some(42)
} else {
    None
};

example().maybe_level(value).call();
```

---

To make a non-`Option` function argument or struct field optional you may add `#[builder(default)]` to it, which generates the equivalent builder API as if the type was `Option<T>`. See [`#[builder(default)]` docs](../reference/builder#default) for details.

## Interaction with `Into` conversions

The inner type `T` of the `Option<T>` is subject to [`Into` conversions](./into-conversions). For example, if `T` by default qualifies for an automatic `Into` conversion or `#[builder(into)]` was used to force it, then the generated builder API will use `impl Into<T>`.

**Example:**

```rust
use bon::builder;

#[builder]
fn example(level: Option<String>) {}

example()
    .level("Accepts `impl Into<String>`, which allows for passing `&str`")
    .call();

example()
    .maybe_level(Some(
        "Accepts `Option<impl Into<String>>`, \
        which allows for passing `Option<&str>`"
    ))
    .call();

```
