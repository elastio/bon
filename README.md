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
      <a href="https://docs.rs/bon/latest/bon/"><img
        alt="docs.rs"
        src="https://img.shields.io/badge/MSRV-1.59.0-b83fbf?style=for-the-badge&labelColor=555555&logo=docs.rs"
        height="25"
    /></a>
</p>

`bon` is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

Visit the [guide for a complete overview of the crate](https://elastio.github.io/bon/guide/overview).

## Quick examples

### Builder for a free function

You can turn a function with positional parameters into a function with named parameters just by placing the `#[builder]` attribute on top of it.

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

### Builder for an associated method

For associated methods you also need to add the `#[bon]` macro on top of the impl block.

```rust
use bon::bon;

struct User {
    id: u32,
    name: String,
}

#[bon] // <- this attribute is required on impl blocks that contain `#[builder]`
impl User {
    #[builder]
    fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    #[builder]
    fn greet(&self, target: &str, level: Option<&str>) -> String {
        let level = level.unwrap_or("INFO");
        let name = &self.name;

        format!("[{level}] {name} says hello to {target}")
    }
}

// The method named `new` generates `builder()/build()` methods
let user = User::builder()
    .id(1)
    .name("Bon".to_owned())
    .build();

// All other methods generate `method_name()/call()` methods
let greeting = user
    .greet()
    .target("the world")
    // `level` is optional, we can omit it here
    .call();

assert_eq!(user.id, 1);
assert_eq!(user.name, "Bon");
assert_eq!(greeting, "[INFO] Bon says hello to the world");
```

### Builder for a struct

The `#[derive(Builder)]` macro generates a builder for a struct.

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
