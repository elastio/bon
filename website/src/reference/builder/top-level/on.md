---
outline: deep
---

# `on`

**Applies to:** <Badge text="structs"/> <Badge text="functions"/> <Badge text="methods"/>

Applies member attributes to all members matching a type pattern. The syntax of this attribute is `on(type_pattern, attributes)`. For example, you can automatically apply `#[builder(into)]` to all members of type `String` this way:

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

```rust [Function]
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

```rust [Method]
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

## Type pattern

`type_pattern` is a type that will be compared with the types of the members. The types are compared textually. For example, `String` doesn't match `std::string::String` because, internally, they are compared just like strings `"String" == "std::string::String"`.

However, you can use `_` to mark parts of the type that should be ignored when matching. For example, `Vec<_>` matches `Vec<u32>` or `Vec<String>`. Lifetimes are ignored during matching.

If you want to apply the attributes to all members, you can use the `_` type pattern that matches any type. For example, `#[builder(on(_, into))]`.

For optional members, the underlying type is matched ignoring the `Option` wrapper.

## Attributes

There are several attributes supported in the `attributes` position listed below.

- [`into`](../member/into)
- [`required`](../member/required) - currently, this attribute can only be used with the `_` type pattern as the first `on(...)` clause
- [`setters(doc(default(skip)))`](../member/setters#doc-default-skip)
- [`overwritable`](../member/overwritable) - 🔬 **experimental**, this attribute is available under the cargo feature `"experimental-overwritable"` (see the issue [#149](https://github.com/elastio/bon/issues/149))

A single `on(...)` clause can contain several of these separated by a comma e.g. `on(_, into, required)`.

## Examples

::: code-group

```rust [into]
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))] // [!code highlight]
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

```rust [required]
use bon::Builder;

#[derive(Builder)]
#[builder(on(_, required))] // [!code highlight]
struct Example {
    name: String,
    level: Option<u32>,
    description: Option<String>,
}

Example::builder()
    .name("regular required member".to_owned())
    .level(Some(99))
    .description(Some("required `Option`".to_owned()))
    .build();
```

```rust [setters(doc(default(skip)))]
use bon::Builder;

#[derive(Builder)]
#[builder(on(_, setters(doc(default(skip)))))]
struct Example {
    // The default value `42` won't appear in the generated docs
    #[builder(default = 42)]
    x1: u32,
}
```

```rust [overwritable]
use bon::Builder;

#[derive(Builder)]
#[builder(on(_, overwritable))] // [!code highlight]
struct Example {
    x: u32,
    y: Option<u32>,
}

Example::builder()
    // Now we can call setters for the same member multiple times
    .x(2)
    .x(99)
    // Same also works for optional members
    .y(1)
    .maybe_y(None)
    .y(2)
    .build();
```

:::

You can specify `on(...)` multiple times. All `on(...)` clauses must be consecutive (no other attributes between them are allowed).

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

## Future Releases

There is an issue [#152](https://github.com/elastio/bon/issues/152) about adding support for [`default`](../member/default.md), [`with`](../member/with) and other non-boolean attributes to the `on(...)` clause. We'll be glad if you take a look at the design proposed in that issue and put a 👍 if you like/want this feature or leave a comment if you have some more feedback.
