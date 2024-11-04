---
title: Next-gen builder macro Bon 2.2 release 🎉. Derive syntax and cfg support 🚀
date: 2024-09-08
author: Veetaha
outline: deep
---

[`bon`] is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you don't know about [`bon`], then see the [motivational blog post](./how-to-do-named-function-arguments-in-rust) and [the crate overview](../guide/overview).

## New features

### `#[derive(Builder)]` syntax for structs

A new `#[derive(Builder)]` API is now exposed by `bon`, which is destined to replace the raw `#[bon::builder]` macro when a builder is derived for a struct. The reasons for this change are described in ["Why using a `#[derive(Builder)]` syntax?"](#why-using-a-derive-builder-syntax).

```rust ignore
use bon::builder; // [!code --]
use bon::Builder; // [!code ++]


#[builder]         // [!code --]
#[derive(Builder)] // [!code ++]
struct User {
    name: String,
    level: Option<u32>
}

User::builder()
    .name("Bon".to_owned())
    .level(100)
    .build();
```

::: warning It's not a breaking change

The usage of `#[bon::builder]` on a struct is still supported in this minor release, and all it does is just [forward](https://github.com/elastio/bon/blob/7294312bbc7ad7c612104d31d65251dc2c7f2d8d/bon-macros/src/builder/mod.rs#L43-L53) to the `#[derive(Builder)]` under the hood. Starting with the _next_ minor release (`2.3`) of `bon` it'll emit a deprecation warning suggesting a migration to `#[derive(Builder)]`. If we ever make a `bon 3.0` (which we have no reason to yet), we'll remove support for `#[bon::builder]` on structs at that point.

:::

Note that `#[bon::builder]` is still supported on functions and associated methods, and it's still the only way to generate a builder for these use cases. The only change is that for structs `bon` now propagates the usage of the `#[derive(Builder)]` syntax instead.

To assist in this migration there is a CLI tool that can update all usages of `#[builder]` on structs to the new `#[derive(Builder)]` syntax in your existing code. Install and run it in your repository like this:

::: warning

Make sure your working directory is clean from files not committed to git before you run this script because it modifies your Rust source files.

:::

```bash
cargo install --git https://github.com/elastio/bon bon-cli
bon migrate

# Prettify the resulting code
cargo fmt
```

### Derive `Clone` and `Debug` for the builder

A new attribute is now supported at the top level. You can add [`#[builder(derive(...))]`](../reference/builder/top-level/derive) to ask `bon` to generate implementations of `Clone` or `Debug` for the builder.

This helps with reusing [partial builders](../guide/patterns/conditional-building#shared-partial-builder), because now you can clone the builder where only part of the fields are set.

The `Debug` derive allows you to [inspect](../guide/basics/inspecting) the builder state for debugging purposes.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(derive(Clone, Debug))] // [!code highlight]
struct Example {
    name: String,
    level: u32,
}

let builder = Example::builder()
    .name("Bon".to_owned());

// We can get the debug output of the builder
assert_eq!(
    format!("{builder:?}"),
    r#"ExampleBuilder { name: "Bon" }"#
);

let _ = builder
    // We can clone the builder
    .clone()
    .level(99)
    .build();

// Because we cloned the builder, it's still available here
let _ = builder
    .level(100)
    .build();
```

### Guaranteed MSRV

`bon` now has an official minimum supported Rust version (MSRV) `1.70.0`. It is guaranteed to compile on all versions of Rust starting with `1.70.0` and higher. Note that this isn't the lowest MSRV we can provide. We are planning to lower the MSRV even more in the future ([bon/#102](https://github.com/elastio/bon/issues/102)), but this first step of setting the MSRV at `1.70.0` is already useful enough that we released it.

## Why using a `#[derive(Builder)]` syntax?

Let's start with a bit of history 🐱.

When I started `bon`, my first focus was on developing a macro that generates a builder for functions and associated methods. The only reasonable way to implement such a macro was using the proc macro attribute syntax. There isn't a `derive` syntax for functions and impl blocks. The `derive` syntax is supported only with structs, enums and unions in Rust.

When the time came to add support for generating builders from structs, there already was a `#[builder]` macro working with functions and impl blocks. At that point, it was a no-brainer decision for me to continue extending that macro to support structs and thus have a single API to generate builders from any syntax in Rust.

However, after the initial `bon`'s release, I started receiving feedback from the community.

### Foreignness of the syntax

People generally understood the goal of having a single `#[builder]` macro, but it felt quite foreign to them to use it on structs. Developers are accustomed to using `derive(...)` with structs much more, while the `#[builder]` syntax on structs stands out like the ugly duckling.

For example, suppose you had an existing struct with a bunch of derives on it, and then you decided to generate a builder for that struct using `bon`:

```rust ignore
#[bon::builder] // [!code ++]
#[derive(Debug, Clone, serde::Serialize)]
struct Example {
    // ...
}
```

Notice how `#[bon::builder]` needs to be on a separate line and looks more magical than it should. When people read such code they may think as if `#[bon::builder]` modifies the struct it's placed on, because proc macro attributes have all the power to do that. But `bon` doesn't do that. It leaves the struct as it was and just adds new items (the builder struct and impl blocks) to this code.

Instead, a more natural change would be the extension to the existing list of derives:

```rust ignore
#[derive(Debug, Clone, serde::Serialize)] // [!code --]
#[derive(Debug, Clone, serde::Serialize, bon::Builder)] // [!code ++]
struct Example {
    // ...
}
```

### Lack of support for conditional compilation

When developing `#[bon::builder]` I didn't account for the small technical detail that `#[cfg(...)]` and `#[cfg_attr(...)]` attributes aren't automatically expanded for proc macro attributes.

It means when the `#[bon::builder]` runs against this code it still sees the `#[cfg/cfg_attr(...)]` attributes on the struct fields as they are written:

```rust
#[cfg_attr(feature = "my-feature", bon::builder)]
struct Example {
    #[cfg_attr(feature = "my-feature", builder(into))]
    name: String,

    #[cfg(feature = "my-feature-cache")]
    #[builder(skip)]
    cache: Vec<u32>
}
```

When this code is compiled the `#[bon::builder]` macro needs to do something about the `#[cfg/cfg_attr(...)]` attributes it sees, but it doesn't know whether the predicates used in the `cfg` expressions are `true` or `false`. Thus, the `#[bon::builder]` macro can't decide whether to add `#[builder(into)]` to the `name` field or generate a skipped `cache` field at all.

On the other hand, the compiler expands all the `#[cfg/cfg_attr(...)]` attributes before invoking the `derive` macros. Therefore, derive macros generally never have to even think about conditional compilation because the compiler handles it for them automatically.

So before this `2.2` version of `bon` there was no support for conditional compilation, but people needed that feature ([#bon/68](https://github.com/elastio/bon/issues/68)).

---

But what about conditional compilation with the function and associated method syntax? The answer is... it's supported as well 🐱! But it's supported with a hack which basically involves reinventing the ~~wheel~~ `#[cfg/cfg_attr(...)]` attributes, evaluating them and expanding manually. This hack uses a [long-to-explain macro trickery](https://github.com/elastio/bon/blob/7294312bbc7ad7c612104d31d65251dc2c7f2d8d/bon/src/private/mod.rs#L59-L106). This same hack could be adopted for the struct syntax but there is already a `derive(...)` syntax that solves this problem much better.

::: info Acknowledgements ❤️

This hack is an evolution of [the idea](https://users.rust-lang.org/t/supporting-or-evaluating-cfg-in-proc-macro-parameters/93240/2) shared by [@recatek] on the Rust Forum. Huge thanks for that!

:::

### Worse IDE experience

I've got feedback from the developer using Rust Rover ([#bon/104](https://github.com/elastio/bon/issues/104)) that `#[bon::builder]` on a struct messed up its syntax highlighting and broke the code actions on the struct like viewing the places where it's used ("usages").

Here is how the code was displayed by Rust Rover before adding the `#[bon::builder]` attribute:

<img
    src="https://github.com/user-attachments/assets/903250f2-6de0-41bc-b134-3ca54f96004f"
    data-zoomable
    width="300px"
/>

and here is how it looked after:

<img
    src="https://github.com/user-attachments/assets/0ae784c4-c9f8-4e1b-813d-94901dd68b2f"
    data-zoomable
    width="300px"
/>

Unfortunately, Rust Rover has yet to catch up with Rust Analyzer in this regard, because no such problems were visible to me in Rust Analyzer. I could do some workarounds to fix at least syntax highlighting in the Rust Rover by reordering the items in the generated code a bit, but that would break the syntax highlighting in Rust Analyzer 🗿.

The other problem that even Rust Analyzer suffers from is that when you use a proc macro attribute syntax the code that is conditionally compiled out via `cfg` attributes is no longer displayed as dimmed.

Here is what it looks like with `#[bon::builder]`:

<img
    src="https://github.com/user-attachments/assets/610974b2-1a6f-46a7-acb4-b5958d39c3e6"
    data-zoomable
    width="300px"
/>

and here is how it looks with the new `derive` syntax:

<img
    src="https://github.com/user-attachments/assets/ff0d5b3e-ad7f-43b6-895f-d38bf0185855"
    data-zoomable
    width="300px"
/>

---

So, given all the above, the decision was made to deprecate the usage of `#[bon::builder]` on structs and use the `#[derive(Builder)]` syntax instead.

If you like or dislike this change in syntax feel free to write a comment on Reddit or message us on Discord (see the Discord server announcement below). It'll be interesting to hear your thoughts!

## Summary

We are listening to your feedback! If you'd like to propose a change in `bon`, or ask a question, or just say "thank you", consider joining our [newly launched Discord server](https://bon-rs.com/discord)!

Also, a huge thank you for 750 stars ⭐ [on Github](https://github.com/elastio/bon)! Consider giving [`bon`] a star if you haven't already. Your support and feedback are a big motivation and together we can build a better builder 🐱!

::: tip

You can leave comments for this post on the platform of your choice:

-   [Reddit](https://www.reddit.com/r/rust/comments/1fc0ai7/media_nextgen_builder_macro_bon_22_release_derive/)
-   [X (Twitter)](https://x.com/veetaha/status/1832804375154065432)

:::

[`bon`]: https://github.com/elastio/bon
[@recatek]: https://github.com/recatek
