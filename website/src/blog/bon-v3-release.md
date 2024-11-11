---
title: Next-gen builder macro Bon 3.0 release 🎉. Revolutional typestate design 🚀
date: 2024-11-13
author: Veetaha
outline: deep
hidden: true
---

[`bon`][bon-github] is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you don't know about [`bon`][bon-github], then see the [motivational blog post](./how-to-do-named-function-arguments-in-rust) and [the crate overview](../guide/overview).

## Snippet of This Release :cat:

```rust
#[bon::builder]
fn greet(level: Option<&str>, name: &str) -> String {
    format!("[{}] {name} says hello!", level.unwrap_or("DEBUG"))
}

// Import type states from the generated module (private by default)
use greet_builder::{SetName, SetLevel};

// Builder type states have stable readable names 🎉
let builder: GreetBuilder<SetName>           = greet().name("Bon");
let builder: GreetBuilder<SetLevel<SetName>> = builder.level("INFO");
//                                                     ^^^^^ optional to set

assert_eq!("[INFO] Bon says hello!", builder.call());
```

## Community Update

It's been two months since the previous [2.3](./bon-builder-v2-3-release) release, and a lot happened. `bon` has breached **1000** ⭐ stars on [Github][bon-github] and **150_000** downloads on [crates.io](https://crates.io/crates/bon) 📈. Also, some big repositories started using `bon`: [`crates.io` backend](https://github.com/rust-lang/crates.io), [ractor](https://github.com/slawlor/ractor), [comrak](https://github.com/kivikakk/comrak), etc. Thank you so much 🥳!

## What's New

This is technically a major release, but [breaking changes](../changelog) are very minor. 99% of users should be able to update without any migration. The dominating part of this release is actually big new features that extend existing API 🚀.

## Typestate API

The main feature of this release is the redesign and stabilization of the builder's typestate.
It is now possible to denote 📝 the builder's type as shown in the example snippet at the beginning of this post.

The builder's typestate signature was simplified to the extent that it became human-readable and even maintainable by hand 👐. It's composed of simple type state transitions that wrap each other on every setter call.

This is in essence revolutionary, no other typestate-based builder crate has offered this feature before 🚀. For example, [`typed-builder`](https://docs.rs/typed-builder/latest/typed_builder/) doesn't document its builder's signature, presumably because of its complexity and abstraction/privacy leaks.

Let's actually compare `bon` and `typed-builder` to understand what it means.

```rust
struct Private(u32);

#[derive(bon::Builder)]
pub struct BonExample {
    // This attribute is also a new feature of this release 🎉.
    // It allows you to perform a conversion in the setter, e.g. to hide a private type.
    #[builder(with = |value: u32| Private(value))]
    x1: Private,
    x2: i32,
    x3: i32,
}

#[derive(typed_builder::TypedBuilder)]
pub struct TbExample {
    // Analogous attribute in typed-builder to do a conversion in the setter
    #[builder(setter(transform = |value: u32| Private(value)))]
    x1: Private,
    x2: i32,
    x3: i32,
}

// Import bon's typestate components
use bon_example_builder::{SetX1, SetX2};

// Bon's builder type
let a: BonExampleBuilder<SetX2<SetX1>> = BonExample::builder().x1(1).x2(2);
//                       ^^^^^^^^^^^^ mentions only fields that were set

// Typed-builder's builder type
let b: TbExampleBuilder<((Private,), (i32,), ())> = TbExample::builder().x1(1).x2(2);
//                       ^^^^^^^             ^^ empty tuple for unset `x3` field
// typed-builder leaked a private type
```

### `typed-builder`'s typestate

`typed-builder` uses a tuple to represent the typestate with the following rules:

-   The number of items in the tuple corresponds to the number of fields in the struct.
-   `()` item in the tuple represents a field that was not set yet.
-   `(T,)` item in the tuple represents a field that was already set; `T` is the type of that field.

`typed-builder`'s approach violates privacy by exposing the internals of the struct:

-   🚨 Types of the struct's fields
-   🚨 Order of struct's fields' declaration
-   🚨 Number of struct's fields

If the users of `typed-builder` ever write a type annotation for the builder, then their code becomes fragile to any changes in the struct's private fields.

### `bon`'s typestate

Starting with this release, `bon` uses a layered typestate that doesn't mention the field's type anywhere in its signature, and it is independent of the number and order of the struct's fields 🔐.

However, `bon`'s signature depends on the order of setter calls. For example:

-   if you call `x1(1).x2(2)`, the type state is `SetX2<SetX1>`
-   if you call `x2(2).x1(1)`, the type state is `SetX1<SetX2>`

This is still better than the tuple approach. The setter calls order is controlled by the caller, so this isn't private to them anyway.

### Cleanness

If there were 4 fields in the struct, `typed-builder`'s initial state of the builder would be <span class="nobr">`((), (), (), ())`</span>, which is very noisy in IDE type hints and the generated `rustdoc` documentation for the builder.

Compare the docs generated by `typed-builder` and `bon`:

::: details Docs by `typed-builder` ([docs.rs link](https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/typed_builder/struct.StructBuilder.html))

![typed-builder-docs-example](https://github.com/user-attachments/assets/a91332cb-638f-44a6-802f-6c046369d1e7){data-zoomable}

:::

::: details Docs by `bon` ([docs.rs link](https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/bon/struct.StructBuilder.html))

![bon-builder-docs-example](https://github.com/user-attachments/assets/115e93e4-095a-4322-ad1d-3c1b13e60763){data-zoomable}

:::

### Custom Methods

Now, enough comparing, let's see how this feature is actually useful. `bon` allows you to work with the builder's typestate via the new [Typestate API](../guide/typestate-api). It is documented and fairly simple.

The following example shows how you can add a custom setter method to the builder. You can probably understand what's going on in the following code snippet without studying the [Typestate API docs](../guide/typestate-api) so closely.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32
}

use example_builder::{IsUnset, State, SetX1};

impl<S: State> ExampleBuilder<S> {
    fn x1_doubled(self, value: u32) -> ExampleBuilder<SetX1<S>>
    where
        S::X1: IsUnset,
    {
        self.x1(value * 2)
    }
}

let value = Example::builder()
    .x1_doubled(3)
    .build();

assert_eq!(value.x1, 6);
```

Here, we've added a new setter `x1_doubled()` to the builder by directly writing an `impl` block for it. We used some types and traits from the generated module to do that. If you want to learn more about this API, then check out the new [Typestate API](../guide/typestate-api) section in the guide.

This allows you to add arbitrary methods to the builder, not just setters. You can make them fallible, `async`, `unsafe`, or whatever you want.

Note, however, that for a simple setter like the one demonstrated above, you could use the new [`#[builder(with)]`][with] annotation also added in this release.

## `#[builder(with)]`

While the [Typestate API](../guide/typestate-api) provides you with maximum flexibility, it may be too verbose for simple cases.

The new [`#[builder(with)]`][with] attribute allows you to override the setters' signature and do a custom conversion much easier. Here is a simple example, that uses a closure syntax.

```rust
use bon::Builder;

struct Point {
    x: u32,
    y: u32,
}

#[derive(Builder)]
struct Example {
    #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
    point: Point,
}

let value = Example::builder()
    .point(2, 3) // [!code highlight]
    .build();

assert_eq!(value.point.x, 2);
assert_eq!(value.point.y, 3);
```

With this attribute, you can also define fallible setters if you specify a [fallible closure](../reference/builder/member/with#fallible-closure). There is an even shorter syntax for some [well-known conversions](../reference/builder/member/with#well-known-functions), for example, `#[builder(with = FromIterator::from_iter)]` can be applied to a member of a collection type to make the setter accept an `impl IntoIterator<...>` hiding the underlying collection's type.

## Better Rustdoc Output

The documentation generated for the builder was significantly improved in this release. This is all thanks to the [typestate redesign](#typestate-api) and cleanup. You've already seen a comparison of the new look of the docs generated for `bon`'s builders, but there is more.

Now, if you use the [`#[builder(default)]`](../reference/builder/member/default) attribute, the default value will be automatically inserted into the documentation. Big `default = ...` expressions are formatted as code blocks.

![image](https://github.com/user-attachments/assets/390858df-1614-42b1-859c-cd70871054bc){data-zoomable}

::: tip Note

`bon` uses [`prettyplease`](https://docs.rs/prettyplease/latest/prettyplease/) for the formatting of these snippets, and it doesn't prettify macros such as `vec![]`, unfortunately. `Vec::from` works in the meantime.

:::

There are also **Required**/**Optional** hints in the generated setters' documentation.

There are no more generics with leading `__` in the docs. Builder macros now handle name conflicts automatically and use simpler names for generated generic parameters to make docs cleaner.

Here is [the link](https://docs.rs/bon-sandbox/latest/bon_sandbox/attr_default/struct.ExampleBuilder.html) to the example docs shown above.

## Granular Docs and Visibility Overrides

There were many new attributes added to override the visibility and the documentation of various items generated by the builder macros:

-   [`builder_type(vis = "...", docs { ... })`](../reference/builder/top-level/builder_type)
-   [`start_fn(vis = "...", docs { ... })`](../reference/builder/top-level/start_fn)
-   [`finish_fn(vis = "...", docs { ... })`](../reference/builder/top-level/finish_fn)
-   [`state_mod(vis = "...", docs { ... })`](../reference/builder/top-level/state_mod)
-   [`setters(vis = "...", docs { ... })`](../reference/builder/member/setters)

See the updated [Documenting](../guide/basics/documenting#custom-doc-attributes) page.

## `#[builder(required)]`

`bon` treats members of type `Option<T>` as optional by default, however sometimes it doesn't make sense. In rare cases, people would like `Option<T>` not to be treated specially. The new attribute [`#[builder(required)]`](../reference/builder/member/required) opts out from that behavior.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(required)]
    required: Option<u32>,

    optional: Option<u32>,
}

Example::builder()
    .required(Some(2)) // Not calling this setter wouldn't compile
    .optional(2)       // We could omit to call this setter
    .build();
```

## `#[builder(overwritable)]` :microscope:

[`#[builder(overwritable)]`](../reference/builder/member/overwritable) is a new experimental attribute that allows you to disable overwrite protection in setters.

```rust
#[derive(bon::Builder)]
struct Example {
    #[builder(overwritable)] // [!code highlight]
    x: u32,
}

Example::builder()
    .x(1)
    // Setting the value for `x` second time is allowed with `overwritable` // [!code highlight]
    .x(2) // [!code highlight]
    .build();
```

This attribute is available under the `experimental-overwritable` cargo feature. It is intended to be used in tests to assist in [dummy data creation](../reference/builder/member/overwritable#dummy-values-in-tests) and [compile times reduction](../reference/builder/member/overwritable#improving-compile-times) since it removes some type state transitions. Consult this [attribute's reference](../reference/builder/member/overwritable) for details.

We are seeking feedback for this feature and would be glad if you could leave a comment under the issue [#149](https://github.com/elastio/bon/issues/149) if you have a use case for it.

## `#[builder(crate)]`

If you want to wrap `bon`'s macros with your own, the [`#[builder(crate)]`](../reference/builder/top-level/crate) attribute will help you reexport `bon` and tell it to reference symbols from the given path instead of the default `::bon`.

## Other Changes

This post doesn't cover everything. See the [full changelog here](../changelog#300---2024-11-13).

## Future Work

Now, that the [Typestate API](../guide/typestate-api) is in place, and you can add custom methods to the builder, the missing piece is the ability to add custom fields to the builder that you could use in those methods ([#189](https://github.com/elastio/bon/issues/189)).

There are ideas for a new `#[builder(flag)]` attribute ([#142](https://github.com/elastio/bon/issues/142)) that would generate a pair of setters:

-   `member()` - doesn't accept any arguments, sets the member to `true`
-   `with_member(bool)` - accepts a boolean value like a usual setter

These features are on the next priority list for `bon`, so stay tuned for more updates!

## Summary

Huge thank you for 1150 stars ⭐ [on Github](https://github.com/elastio/bon)! Consider giving [`bon`][bon-github] a star if you haven't already. Share it with your friends/colleagues to help others discover it 🔭. Your support and feedback are a big motivation and together we can build a better builder 🐱!

Bon's goal is to empower everyone to build beautiful APIs with great flexibility and extensibility. If you have any feedback or ideas for improvement, consider joining [our Discord server](https://bon-rs.com/discord) to discuss them, or just [open an issue/discussion on Github](https://github.com/elastio/bon/issues).

<!-- ::: tip

You can leave comments for this post on the platform of your choice:

-   [Reddit](https://www.reddit.com/r/rust/comments/1fgmbo7/media_nextgen_builder_macro_bon_23_release/)
-   [X (Twitter)](https://x.com/veetaha/status/1834951093559648544)

::: -->

[bon-github]: https://github.com/elastio/bon
[with]: ../reference/builder/member/with
