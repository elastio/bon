<a href="https://elastio.github.io/bon/guide/overview">
    <!--
    We use an absolute link to the image here because this README is hosted on crates.io,
    lib.rs and docs.rs where this image isn't available through the relative link.
    -->
    <img
        src="https://elastio.github.io/bon/bon-home.png"
        alt="bon home"
    />
</a>

<p align="center">
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
</p>

`bon` is a Rust crate for generating compile-time-checked builders for functions and structs.

Visit the [guide for a complete overview of the crate](https://elastio.github.io/bon/guide/overview).

## Quick examples

`bon` can turn a function with positional parameters into a function with "named" parameters via a builder. It's as easy as placing the `#[builder]` attribute on top of it.

```rust
use bon::builder;

#[builder]
fn greet(name: &str, age: u32) -> String {
    format!("Hello {name} with age {age}!")
}

let greeting = greet()
    .name("Bon")
    .age(24)
    .call();

assert_eq!(greeting, "Hello Bon with age 24!");
```

You can also use the `#[builder]` attribute with structs and associated methods:

```rust
use bon::{bon, builder};

#[builder]
struct User {
    id: u32,
    name: String,
}

#[bon]
impl User {
    #[builder]
    fn greet(&self, target: &str, level: Option<&str>) -> String {
        let level = level.unwrap_or("INFO");
        let name = &self.name;

        format!("[{level}] {name} says hello to {target}")
    }
}

let user = User::builder()
    .id(1)
    .name("Bon".to_owned())
    .build();

let greeting = user
    .greet()
    .target("the world")
    // `level` is optional, we can omit it here
    .call();

assert_eq!(user.id, 1);
assert_eq!(user.name, "Bon");
assert_eq!(greeting, "[INFO] Bon says hello to the world");
```

See [the guide](https://elastio.github.io/bon/guide/overview) for the rest.

---

If you like the idea of this crate and want to say "thank you" or "keep doing this" consider giving us a [star ⭐ on Github](https://github.com/elastio/bon). Any support and contribution are appreciated 🐱!

#### License

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
