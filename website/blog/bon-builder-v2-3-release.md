---
title: Next-gen builder macro Bon 2.3 release 🎉. Positional arguments in starting and finishing functions 🚀
date: 2024-09-14
author: Veetaha
outline: deep
---

[`bon`] is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you don't know about [`bon`], then see the [motivational blog post](./how-to-do-named-function-arguments-in-rust) and [the crate overview](../guide/overview).

## Meme of this release 🐱

<img
    src="https://github.com/user-attachments/assets/f9657e2b-1e64-4023-b239-3acc0cead350"
    data-zoomable
    style="border-radius: 15px"
/>


## New features

### Positional arguments in starting and finishing functions

While having the ability to use separate setters for the members gives you a ton of flexibility and extensibility described on the ["Compatibility"](../guide/compatibility) page, sometimes you don't need all of that.

Maybe you'd like to pick out some specific members and let the user pass their values as positional parameters to the starting function that creates the builder or to the finishing function that consumes it. This reduces the syntax a bit at the cost of some extensibility loss ⚖️, but it may be worth it!

#### Starting function

As an example, suppose we have a `Treasure` struct with `x` and `y` coordinates and a `label` that describes the payload of the treasure. Since all treasures are located somewhere, they all have coordinates, and it would be cool to specify them in a single starting function call.

To do that we can use the `#[builder(start_fn)]` attribute. There are two contexts where we can place it, and they both have a different meaning:

- [Top-level `#[builder(start_fn = ...)]`](../reference/builder#start-fn) - configures the name of the starting function and optionally its visibility
- [Member-level `#[builder(start_fn)]`](../reference/builder#start-fn-1) - configures the member to be a positional parameter on the starting function

We'll want to use both of these attributes in our example to give a better name for the starting function that describes its inputs and configure `x` and `y` as positional parameters on the starting function as well.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
// Top-level attribute to give a better name for the starting function // [!code highlight]
#[builder(start_fn = with_coordinates)]                                // [!code highlight]
struct Treasure {
    // Member-level attribute to mark the member as // [!code highlight]
    // a parameter of `with_coordinates()`          // [!code highlight]
    #[builder(start_fn)]                            // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9) // [!code highlight]
    .label("oats".to_owned())
    .build();

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
```

Here, the generated `with_coordinates` method has the following signature:

```rust ignore
impl Treasure {
    fn with_coordinates(x: u32, y: u32) -> TreasureBuilder { /**/ }
}
```

#### Finishing function

Now let's say we need to know the person who claimed the `Treasure`. While describing the treasure using the current builder syntax we'd like the person who claimed it to specify their first name and last name at the end of the building process.

We can use a similar combination of the [top-level `#[builder(finish_fn = ...)]`](../reference/builder#finish-fn) and the [member-level `#[builder(finish_fn)]`](../reference/builder#finish-fn-1) attributes to do that.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(start_fn = with_coordinates)]
#[builder(finish_fn = claim)]  // [!code highlight]
struct Treasure {
    #[builder(start_fn)]
    x: u32,

    #[builder(start_fn)]
    y: u32,

    #[builder(finish_fn)]          // [!code highlight]
    claimed_by_first_name: String, // [!code highlight]

    #[builder(finish_fn)]          // [!code highlight]
    claimed_by_last_name: String,  // [!code highlight]

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9)
    .label("oats".to_owned())
    .claim("Lyra".to_owned(), "Heartstrings".to_owned()); // [!code highlight]

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
assert_eq!(treasure.claimed_by_first_name, "Lyra");        // [!code highlight]
assert_eq!(treasure.claimed_by_last_name, "Heartstrings"); // [!code highlight]
```

You may also combine these attributes with [`#[builder(into)]`](../reference/builder#into) or [`#[builder(on(..., into))]`](../reference/builder#into) to reduce the number of `to_owned()` calls a bit. See this described in detail on the new ["Positional members"](../guide/positional-members#into-conversions) page in the guide.

### Guaranteed MSRV is 1.59.0 now

On the previous week's update (2.2 release) [a promise was made](./bon-builder-v2-2-release#guaranteed-msrv) to reduce the MSRV (minimum supported Rust version) from the initial 1.70.0 even further, and this has been done 🎉!

This is the lowest possible MSRV we can guarantee for now. The choice of this version was made based on our design requirements for const generics supports described in [the comment here](https://github.com/elastio/bon/blob/3217b4b0349f03f0b2a5853310f420c5b8b005a7/bon/Cargo.toml#L21-L28).


## Deprecation warnings

As was [promised](./bon-builder-v2-2-release#derive-builder-syntax-for-structs) in the previous release we are enabling deprecation warnings for the usage of the bare `#[bon::builder]` attribute on structs in favour of the new `#[derive(bon::Builder)]` syntax.

The `#[builder]` syntax is still supported on functions and associated methods, and it's the only way to generate builders for them.

The reasons for this deprecation as well as the instruction to update your code are described in the [2.2. release blog post](./bon-builder-v2-2-release#derive-builder-syntax-for-structs).


::: warning

This isn't a breaking change, and the code that uses `#[bon::builder]` on a struct will still compile albeit with a compiler warning. Once `bon` reaches a `3.0` release we'll remove support for `#[bon::builder]` on structs entirely. However, there are no particular reasons and plans for a new major release of `bon` yet.

:::

## Summary

Huge thank you for 925 stars ⭐ [on Github](https://github.com/elastio/bon)! Consider giving [`bon`] a star if you haven't already. Your support and feedback are a big motivation and together we can build a better builder 🐱!

Bon's goal is to empower everyone to build beautiful APIs with great flexibility and extensibility. If you have any feedback or ideas for improvement consider joining [our Discord server](https://discord.gg/QcBYSamw4c) to discuss them, or just [open an issue on Github](https://github.com/elastio/bon/issues).


<!--
::: tip

You can leave comments for this post on the platform of your choice:
- [Reddit](https://www.reddit.com/r/rust/comments/1fc0ai7/media_nextgen_builder_macro_bon_22_release_derive/)
- [X (Twitter)](https://x.com/veetaha/status/1832804375154065432)

:::
-->

[`bon`]: https://github.com/elastio/bon

*[Member]: Struct field or a function argument
*[member]: Struct field or a function argument
*[members]: Struct fields or function arguments
