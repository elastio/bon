# `on`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Applies the given builder attributes to all members that match a type pattern. For example, you can automatically apply `#[builder(into)]` to all members of type `String` this way:

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))]
struct Example {
    id: String,
    name: String,
    level: u32,
}

Example::builder()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .build();
```

```rust [Free function]
use bon::builder;

#[builder(on(String, into))]
fn example(
    id: String,
    name: String,
    level: u32,
) {}

example()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .call();
```

```rust [Associated method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(on(String, into))]
    fn example(
        id: String,
        name: String,
        level: u32,
    ) {}
}

Example::example()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .call();
```

:::

This attribute must be of form `on(type_pattern, attributes)`.

- `type_pattern` - type that will be compared with the types of the members. The types are compared textually. For example, `String` doesn't match `std::string::String`. You can use `_` to mark parts of the type to ignore when matching. For example, `Vec<_>` matches `Vec<u32>` or `Vec<String>`. Lifetimes are ignored during matching.

- `attributes` - for now, the only attribute supported in the `attributes` position is [`into`](../member/into). It sets `#[builder(into)]` for members that match the `type_pattern`.

If you want to apply the `attributes` to all members, you can use the `_` type pattern that matches any type. For example, `#[builder(on(_, into))]`.

For optional members the underlying type is matched ignoring the `Option` wrapper.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))]
struct Example {
    name: String,
    description: Option<String>,

    #[builder(default)]
    alias: String
}

Example::builder()
    .name("Bon")
    // These members also match the `String` type pattern,
    // so `#[builder(into)]` was applied to them
    .description("accepts an `impl Into<String>` here")
    .alias("builder")
    .build();
```

You can specify `on(...)` multiple times.

**Example:**

```rust
use bon::Builder;
use std::path::PathBuf;

#[derive(Builder)]
#[builder(on(String, into), on(PathBuf, into))]
struct Example {
    name: String,
    path: PathBuf,
    level: u32,
}

Example::builder()
    .name("accepts `impl Into<String>`")
    .path("accepts/impl/into/PathBuf")
    // This member doesn't match either `String` or `PathBuf`,
    // and thus #[builder(into)] was not applied to it
    .level(100)
    .build();
```
