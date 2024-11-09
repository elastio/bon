<a href="https://bon-rs.com/guide/overview">
    <img
        src="https://bon-rs.com/bon-home.png"
        alt="bon home"
    />
</a>

<div align="center">
    <a href="https://github.com/elastio/bon"><img
        alt="github"
        src="https://img.shields.io/badge/github-elastio/bon-228b22?style=for-the-badge&labelColor=555555&logo=github"
        height="25"
    /></a>
    <a href="https://crates.io/crates/bon"><img
        alt="crates.io"
        src="https://img.shields.io/crates/v/bon.svg?style=for-the-badge&color=e37602&logo=rust"
        height="25"
    /></a>
    <a href="https://docs.rs/bon/latest/bon/"><img
        alt="docs.rs"
        src="https://img.shields.io/badge/docs.rs-bon-3b74d1?style=for-the-badge&labelColor=555555&logo=docs.rs"
        height="25"
    /></a>
        <a href="https://docs.rs/bon/latest/bon/"><img
        alt="docs.rs"
        src="https://img.shields.io/badge/MSRV-1.59.0-b83fbf?style=for-the-badge&labelColor=555555&logo=docs.rs"
        height="25"
    /></a>
</div>

<div align="center">
    <table>
        <tbody>
            <tr>
                <td><a href="https://bon-rs.com/guide/overview">📖 Guide Book</a></td>
                <td>Narrative introduction</td>
            </tr>
            <tr>
                <td><a href="https://bon-rs.com/reference/builder">🔍 API Reference</a></td>
                <td>Attributes API index</td>
            </tr>
        </tbody>
    </table>
</div>

<!-- #region overview -->

# Announcement

Release `3.0.0-rc` (release-candidate) was published 🎉. The ultimate stable `3.0.0` release is scheduled for 13-th of November. You are encouraged to use the `3.0.0-rc` version in the meantime and post your feedback in [#156](https://github.com/elastio/bon/issues/156). The release blog post will be ready on the scheduled release date, until then see the [changelog](https://bon-rs.com/changelog) for details.

# Overview

`bon` is a Rust crate for generating compile-time-checked builders for structs and functions. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you wonder "Why would I use builders?", see the [motivational blog post](https://bon-rs.com/blog/how-to-do-named-function-arguments-in-rust).

## Function Builder

You can turn a function with positional parameters into a function with named parameters with `#[builder]`.

```rust
use bon::builder;

#[builder]
fn greet(name: &str, level: Option<u32>) -> String {
    let level = level.unwrap_or(0);

    format!("Hello {name}! Your level is {level}")
}

let greeting = greet()
    .name("Bon")
    .level(24) // <- setting `level` is optional, we could omit it
    .call();

assert_eq!(greeting, "Hello Bon! Your level is 24");
```

Any syntax for functions is supported including `async`, fallible, generic functions, `impl Trait`, etc.

Many things are customizable with additional attributes described in the [API reference](https://bon-rs.com/reference/builder), but let's see what else `bon` has to offer.

## Struct Builder

Use `#[derive(Builder)]` to generate a builder for a struct.

```rust
use bon::Builder;

#[derive(Builder)]
struct User {
    name: String,
    is_admin: bool,
    level: Option<u32>,
}

let user = User::builder()
    .name("Bon".to_owned())
    // `level` is optional, we could omit it here
    .level(24)
    // call setters in any order
    .is_admin(true)
    .build();

assert_eq!(user.name, "Bon");
assert_eq!(user.level, Some(24));
assert!(user.is_admin);
```

## Method Builder

Associated methods require `#[bon]` on top of the impl block additionally.

### Method `new`

The method named `new` generates `builder()/build()` methods.

```rust
use bon::bon;

struct User {
    id: u32,
    name: String,
}

#[bon]
impl User {
    #[builder]
    fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

let user = User::builder()
    .id(1)
    .name("Bon".to_owned())
    .build();

assert_eq!(user.id, 1);
assert_eq!(user.name, "Bon");
```

`#[derive(Builder)]` on a struct generates builder API that is fully compatible with placing `#[builder]` on the `new()` method with a signature similar to the struct's fields (more details on the [Compatibility](https://bon-rs.com/guide/basics/compatibility#switching-between-derive-builder-and-builder-on-the-new-method) page).

### Other Methods

All other methods generate `{method_name}()/call()` methods.

```rust
use bon::bon;

struct Greeter {
    name: String,
}

#[bon]
impl Greeter {
    #[builder]
    fn greet(&self, target: &str, prefix: Option<&str>) -> String {
        let prefix = prefix.unwrap_or("INFO");
        let name = &self.name;

        format!("[{prefix}] {name} says hello to {target}")
    }
}

let greeter = Greeter { name: "Bon".to_owned() };

let greeting = greeter
    .greet()
    .target("the world")
    // `prefix` is optional, omitting it is fine
    .call();

assert_eq!(greeting, "[INFO] Bon says hello to the world");
```

Methods with or without `self` are both supported.

## No Panics Possible

Builders generated by `bon`'s macros use the typestate pattern to ensure all required parameters are filled, and the same setters aren't called repeatedly to prevent unintentional overwrites. If something is wrong, a compile error will be created.

| ⭐ Don't forget to give our repo a [star on Github ⭐](https://github.com/elastio/bon)! |
| --------------------------------------------------------------------------------------- |

## What's Next?

What you've seen above is the first page of the 📖 Guide Book. If you want to learn more, jump to the [Basics](https://bon-rs.com/guide/basics) section. And remember: knowledge is power 🐱!

Feel free to jump to code and use the `#[builder]` and `#[derive(Builder)]` once you've seen enough docs to get started.

The [🔍 API Reference](https://bon-rs.com/reference/builder) will help you navigate the attributes once you feel comfortable with the basics of `bon`. Both `#[derive(Builder)]` on structs and `#[builder]` on functions/methods have almost identical attributes API, so the documentation for them is common.

## Installation

Add `bon` to your `Cargo.toml`.

```toml
[dependencies]
bon = "2.3"
```

You can opt out of `std` and `alloc` cargo features with `default-features = false` for `no_std` environments.

## Acknowledgments

This project was heavily inspired by such awesome crates as [`buildstructor`](https://docs.rs/buildstructor), [`typed-builder`](https://docs.rs/typed-builder) and [`derive_builder`](https://docs.rs/derive_builder). This crate was designed with many lessons learned from them.

See [alternatives](https://bon-rs.com/guide/alternatives) for comparison.

## Who's Using `bon`?

Some notable users:

-   [`crates.io` backend](https://github.com/rust-lang/crates.io)
-   [`ractor`](https://github.com/slawlor/ractor)
-   [`comrak`](https://github.com/kivikakk/comrak)
-   [`soldeer`](https://github.com/mario-eth/soldeer) (package manager endorsed by [`foundry`](https://github.com/foundry-rs/foundry))
-   [`tachyonfx`](https://github.com/junkdog/tachyonfx)

## Getting Help

If you can't figure something out, consult the docs and maybe use the `🔍 Search` bar on our [docs website](https://bon-rs.com). You may also create an issue or a discussion on the [Github repository](https://github.com/elastio/bon) for help or write us a message on [Discord](https://bon-rs.com/discord).

## Socials

<table>
<tbody>
    <tr>
        <td><a href="https://bon-rs.com/discord">Discord</a></td>
        <td>Here you can leave feedback, ask questions, report bugs, or just write "thank you".</td>
    </tr>
    <tr>
        <td><a href="https://x.com/veetaha" class="nobr">X (Twitter)</a></td>
        <td>Profile of the maintainer. There are only posts about <code>bon</code> and Rust in general here.</td>
    </tr>
</tbody>
</table>

## License

<sup>
Licensed under either of <a href="https://github.com/elastio/bon/blob/master/LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="https://github.com/elastio/bon/blob/master/LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

<!-- #endregion overview -->
