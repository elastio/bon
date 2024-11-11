---
title: Next-gen builder macro Bon 3.0 release 🎉. Revolutional typestate design 🚀
date: 2024-11-13
author: Veetaha
outline: deep
---

[`bon`][bon-github] is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you don't know about [`bon`][bon-github], then see the [motivational blog post](./how-to-do-named-function-arguments-in-rust) and [the crate overview](../guide/overview).

## Snippet of This Release 🐱

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

## New Typestate API

The main feature of this release is the redesign and stabilization of the builder's typestate.
It is now possible to denote 📝 the builder's type as shown on the example snippet at the beginning of this post.

The builder's typestate signature was simplified to the extent that it became human-readable and even maintainable by hand 👐. It's composed of simple type state transitions that wrap each other on every setter call.

This is in essence revolutionary, no other typestate-based builder crate has offered this feature before 🚀. For example, [`typed-builder`](https://docs.rs/typed-builder/latest/typed_builder/) doesn't document its builder's signature, presumably because of its complexity and abstraction/privacy leaks.

See an example comparison between `bon` and `typed-builder` to understand what it means.

```rust
// Private
struct Private(u32);

#[derive(bon::Builder)]
pub struct BonExample {
    // This attribute is also a new feature of this release 🎉.
    // It allows you to do a conversion in the setter, e.g. to hide a private type.
    #[builder(with = |value: u32| Private(value))]
    x1: Private,
    x2: i32,
    x3: i32,
}

#[derive(typed_builder::TypedBuilder)]
pub struct TbExample {
    #[builder(setter(transform = |value: u32| Private(value)))]
    x1: Private,
    x2: i32,
    x3: i32,
}

// Import bon's typestate components
use bon_example_builder::{SetX1, SetX2};

let a: BonExampleBuilder<SetX2<SetX1>> = BonExample::builder().x1(1).x2(2);
let b: TbExampleBuilder<((Private,), (i32,), ())> = TbExample::builder().x1(1).x2(2);
//                       ^^^^^^^            ^^ empty tuple for unset `x3` field
// typed-builder leaked a private type
```

### Privacy

`typed-builder` uses a tuple to represent the typestate. That tuple mentions the types of the private fields of the struct. Obviously, this is a privacy violation 🚨. Users outside of your crate are now exposed to the types of your private fields.

`bon` uses a layered typestate that doesn't mention the field's type anywhere in its signature, so the types of your fields can feel safe with `bon` 🔐.

### Fields Order Leak

`typed-builder`'s typestate tuple contains a type for every field of the struct, including ones that were not set yet. Every item in the tuple corresponds to a field. If you reorder the fields or add new ones, then the typestate tuple changes.

For example, if you add a new field of type `T` between `x2` and `x3` in example above, the signature of a "complete" builder will change:

```rust
TbExampleBuilder<((Private,), (i32,), (i32,))>       // [!code --]
TbExampleBuilder<((Private,), (i32,), (T,), (i32,))> // [!code ++]
```

`bon`'s typestate is not dependent on the order of fields, it also doesn't mention fields that were not set yet. So if you reorder/add fields to your struct, the type annotation will stay stable. So if you add a new field between `x2` and `x3` in `bon`'s case, the signature will stay the same:

```rust
BonExampleBuilder<SetX2<SetX1>>
```

However, `bon`'s signature is dependent on the order of setter calls. If you called `x2(2).x1(1)` instead of `x1(1).x2(2)`, the signature would be different:

```rust
BonExampleBuilder<SetX1<SetX2>>
```

However, this is still better than the tuple approach. The setter calls order is controlled by the caller, so this isn't private to them anyway.

### Noisiness

If there were 5 fields in the struct, there would be a tuple with 5 items. The initial state of the builder would be `((), (), (), (), ())`, which becomes very noisy in IDE type hints and the generated `rustdoc` documentation for the builder.

Compare the docs for a builder for a struct with 4 fields:

#### [Docs by `typed-builder`](https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/typed_builder/struct.StructBuilder.html)

![typed-builder-docs-example](https://github.com/user-attachments/assets/a91332cb-638f-44a6-802f-6c046369d1e7)

#### [Docs by `bon`](https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/bon/struct.StructBuilder.html)

![bon-builder-docs-example](https://github.com/user-attachments/assets/115e93e4-095a-4322-ad1d-3c1b13e60763)

## Summary

Huge thank you for 1150 stars ⭐ [on Github](https://github.com/elastio/bon)! Consider giving [`bon`] a star if you haven't already. Share it with your friends/colleagues to help others discover it 🔭. Your support and feedback are a big motivation and together we can build a better builder 🐱!

Bon's goal is to empower everyone to build beautiful APIs with great flexibility and extensibility. If you have any feedback or ideas for improvement consider joining [our Discord server](https://bon-rs.com/discord) to discuss them, or just [open an issue/discussion on Github](https://github.com/elastio/bon/issues).

<!-- ::: tip

You can leave comments for this post on the platform of your choice:

-   [Reddit](https://www.reddit.com/r/rust/comments/1fgmbo7/media_nextgen_builder_macro_bon_23_release/)
-   [X (Twitter)](https://x.com/veetaha/status/1834951093559648544)

::: -->

[bon-github]: https://github.com/elastio/bon
