---
title: Bon builder generator 2.0 release 🎉
date: 2024-08-26
author: Veetaha
outline: deep
---

`bon` is a Rust crate for generating compile-time-checked builders for functions and structs.

If you don't know about `bon`, then see the [motivational blog post](./how-to-do-named-function-arguments-in-rust) and [the crate overview](../guide/overview).

## New features

### Experimental completions with Rust Analyzer

The macro now generates a special code visible only to Rust Analyzer that adds hints for available attributes and their syntax. This way Rust Analyzer can generate completions and documentation pop-ups when you write your code.

**Example:**

<video controls src="/completions-demo.mp4" title="Title"></video>

Note that this is a highly experimental feature. It is also limited. It works only for `#[builder]` attribute at the top level of a `struct` or a free function. If you find any bugs caused by this feature, please [open an issue](https://github.com/elastio/bon/issues).

The way this is implemented is a big topic that deserves a separate blog post. However, if you are curious here is [the code reference](https://github.com/elastio/bon/blob/c5285102fd9f9fdec0b467d2f75eb8a18bf03c6e/bon-macros/src/util/ide.rs#L128-L138) for this implementation.

### `skip` and `default` expressions have access to other members in their scope

It's now possible to reference other members in `skip` and `default` expressions. Only the members declared higher in code are accessible to these expressions. Here is an example of how it works with `skip`:

```rust compile_fail
use bon::builder;

#[builder]
struct Example {
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(skip = 2 * member_1)]
    member_2: u32,

    #[builder(skip = member_2 + member_1)]
    member_3: u32,
}

let example = Example::builder()
    .member_1(3)
    .build();

assert_eq!(example.member_1, 3);
assert_eq!(example.member_2, 6);
assert_eq!(example.member_3, 9);
```

### Documentation updates

Now the documentation was split into the ["Guide"](../guide/overview) and ["Reference"](../reference/builder) sections. Each of them is versioned, and you can read the docs for the older major versions.

#### Patterns guides

We added 3 new pages with guides on how to use builders idiomatically or solve some common problems (e.g. validating inputs):

- [Conditional Building](../guide/patterns/conditional-building)
- [Fallible Builders](../guide/patterns/fallible-builders)
- [Into Conversions In-Depth](../guide/patterns/into-conversions-in-depth)

I recommend you to check out the ["Into Conversions In-Depth"](../guide/patterns/into-conversions-in-depth) especially because it's highly related to one of the breaking changes that we'll review below.

## Breaking changes

### Removed magical automatic `Into` conversions

This has been a topic of [controversy](https://github.com/elastio/bon/issues/15), but finally, we aligned on the decision to remove the magical automatic `Into` conversions.

The main reason for removing this is to make `bon` more obvious and intuitive. Rust's core pillar is "being explicit". By having automatic `Into` conversions `bon` v1 introduced magical implicit behaviour, that also could lead to some footguns. For a detailed explanation of the potential footguns, see the ["Into Conversions In-Depth"](../guide/patterns/into-conversions-in-depth) page.

Now, if you want to enable `Into` conversions for a set of members, you can use the new [`#[builder(on(type_pattern, into))]`](../reference/builder#on) attribute. It allows you to specify the type that you want to enable `Into` conversions for explicitly.

Let's review an example of a migration that you need to do when upgrading to `bon` v2.

Suppose you had this code previously:

```rust compile_fail
use bon::builder;

#[builder]
struct Example {
    arg1: String,
    arg2: Option<String>,
    arg3: u32,
}

Example::builder()
    .arg1("accepts an impl Into<String>")
    .arg2("accepts an impl Into<String>")
    // No `impl Into` for arg3
    .arg3(32)
    .build();
```

Now, to preserve the same behavior, you need to do this:

```rust compile_fail
use bon::builder;

#[builder(on(String, into))] // Only this line needs to change
struct Example {
    arg1: String,
    arg2: Option<String>,
    arg3: u32,
}

Example::builder()
    .arg1("accepts an impl Into<String>")
    .arg2("accepts an impl Into<String>")
    // No `impl Into` for arg3
    .arg3(32)
    .build();
```

If you need to enable `Into` conversions for multiple types, then you can specify multiple `on(...)` clauses. For example:

```rust ignore
#[builder(on(String, into), on(PathBuf, into))]
```

Alternatively, you can add `#[builder(into)]` on top of each field that requires an `Into` conversion.

**Example:**

```rust compile_fail
use bon::builder;

#[builder]
struct Example {
    #[builder(into)]
    arg1: String,

    #[builder(into)]
    arg2: Option<String>,

    arg3: u32,
}
```

---

Note that if you do this migration, then your builder API will still be the same. If you use `bon` in your public API to generate builders, then it won't be a breaking change if you update it to `2.0`. Just make sure to add `#[builder(into)]` or `#[builder(on(..., into))]` to preserve the same public setters API so that they still accept `impl Into<T>`.

### Removed support for `skip` in function arguments

With bon v1 you could write this:

```rust compile_fail
use bon::builder;

#[builder]
fn example(#[builder(skip = 42)] value: u32) {
    println!("Arg: {value}")
}

// You can't set the `value`, it's skipped and no setters are generated for it
example().call();
```

This code looks unnecessarily complicated. There is no need for a `skip` attribute in functions because it's just easier to use local variables instead:

```rust
use bon::builder;

#[builder]
fn example() {
    let value = 42;
    println!("Arg: {value}")
}

example().call();
```

### Removed support for destructuring patterns in function arguments

Previously, you could write the following code:

```rust compile_fail
use bon::builder;

#[builder]
fn example(
    #[builder(name = point)]
    (x, y): (u32, u32)
) {}

example()
    .point((1, 2))
    .call();
```

Now, destructuring patterns in the function parameter's position aren't supported because this code is unnecessarily complex, and can be made simpler by just doing the destructuring inside of the function's body:

```rust
use bon::builder;

#[builder]
fn example(point: (u32, u32)) {
    let (x, y) = point;
}

example()
    .point((1, 2))
    .call();
```

This also plays better with the feature where [`skip` and `default` expressions have access to other members in their scope](#skip-and-default-expressions-have-access-to-other-members-in-their-scope). This makes sure all members are named and accessible to expressions in `skip` and `default`.

# Summary

`bon` is only a month-old crate, and we've learned many things since its [initial release](https://www.reddit.com/r/rust/comments/1eeem92/how_to_do_named_function_arguments_in_rust/) from your feedback. There were various extensions made to its API, although, there also were breaking changes that eventually led to a `2.0` release. I'm doing this major release earlier rather than later while `bon`'s adoption is growing yet.

Also, huge thank you for 500 stars ⭐ [on Github](https://github.com/elastio/bon)! Consider giving `bon` a star if you haven't already. Your support is a big motivation, and together we can build a better builder 🐱!

::: tip

You can leave comments for this post on the platform of your choice:
- [Reddit](https://www.reddit.com/r/rust/comments/1f1uzkw/bon_builder_generator_20_release/)
- [X (Twitter)](https://x.com/veetaha/status/1828210142514491658)
- [Hacker News](https://news.ycombinator.com/item?id=41359892)

:::
