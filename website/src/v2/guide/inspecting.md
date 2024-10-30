# Inspecting

If you want to inspect the values set in the builder for debugging purposes you can leverage the [`#[builder(derive(...))]`](../reference/builder#derive) attribute to derive the [`Debug`](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html) trait for your builder.

**Example:**

```rust
use bon::builder;

#[builder(derive(Debug))] // [!code highlight]
fn example(
    name: String,
    is_admin: bool,
    level: Option<u32>,
) {}

let builder = example().name("Bon".to_owned());

// This will output the current state of the builder to `stderr`
dbg!(&builder);

// You can also format the debug output to `String`:
assert_eq!(
    format!("{builder:?}"),
    // Only the fields that were set will be output
    r#"ExampleBuilder { name: "Bon" }"#
);

// Finish building
builder.is_admin(true).call();
```

You can also derive the [`Clone`](https://doc.rust-lang.org/stable/std/clone/trait.Clone.html) trait for your builder using this same attribute. See more details in the [reference for the `#[builder(derive(...))]` attribute](../reference/builder#derive).
