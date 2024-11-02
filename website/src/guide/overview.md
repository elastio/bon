<script setup>
import { data as version } from '/../data/version.data'
import VPSocialLink from "vitepress/dist/client/theme-default/components/VPSocialLink.vue";
let [_, versionWildcard] = version.match(/^(\d+\.\d+)\.\d+$/);
if (versionWildcard == null) {
    versionWildcard = version;
}

</script>

# Overview

`bon` is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you are new to the concept of builders or named function arguments, and you don't know what problems they may solve for you, then check out the [motivational blog post](../blog/how-to-do-named-function-arguments-in-rust).

## Installation

Add this to your `Cargo.toml` to use this crate:

```toml-vue
[dependencies]
bon = "{{ versionWildcard }}"
```

You can opt out of `std` and `alloc` cargo features with `default-features = false` for `no_std` environments.


## Builder for a function

`bon` can turn a function with positional parameters into a function with "named" parameters via a builder. It's as easy as placing the `#[builder]` macro on top of it.

**Example:**

```rust
use bon::builder;

#[builder] // [!code highlight]
fn greet(name: &str, age: u32) -> String {
    format!("Hello {name} with age {age}!")
}

let greeting = greet()
    .name("Bon")
    .age(24)
    .call();

assert_eq!(greeting, "Hello Bon with age 24!");
```

::: tip

Many things are customizable with additional attributes. [`#[builder]` macro reference](../reference/builder) describes all of them.

:::

Any syntax for functions is supported including `async`, fallible, generic functions, `impl Trait`, etc. If you find an edge case where `bon` doesn't work, please [create an issue on GitHub](https://github.com/elastio/bon/issues).

## Builder for an associated method

You can also generate a builder for associated methods. For this to work you need to add a `#[bon]` macro on top of the `impl` block additionally.

**Example:**

```rust
use bon::bon;

struct Counter {
    val: u32,
}

#[bon] // <- this macro is required on the impl block // [!code highlight]
impl Counter {
    #[builder] // [!code highlight]
    fn new(initial: Option<u32>) -> Self {
        Self {
            val: initial.unwrap_or_default(),
        }
    }

    #[builder] // [!code highlight]
    fn increment(&mut self, diff: u32) {
        self.val += diff;
    }
}

let mut counter = Counter::builder()
    .initial(3)
    .build();

counter
    .increment()
    .diff(3)
    .call();

assert_eq!(counter.val, 6);
```

::: details Why is that `#[bon]` macro on top of the `impl` block required? 🤔 (feel free to skip)

There are a couple of technical reasons.

First, it's the lack of surrounding context given to a proc macro in Rust. A proc macro sees only the syntax it is placed on top of. For example, the `#[builder]` macro inside of the `impl` block can't see the `impl Counter` part of the impl block above it. However, it needs that information to tell the actual type of `Self`.

Second, the `#[builder]` proc macro generates new items such as the builder struct type definition, which it needs to output **adjacently** to the `impl` block itself. However, proc macros in Rust can only modify the part of the syntax they are placed on and generate new items on the same level of nesting. The `#[builder]` macro inside of the `impl` block can't just break out of it.

:::

::: details Why does it compile without an import of `bon::builder`? 🤔 (feel free to skip)

This is because there is no separate `#[builder]` proc macro running in this case. Only the `#[bon]` macro handles code generation, it's an active attribute, while `#[builder]` is a dumb inert data attribute (see [the Rust Reference](https://doc.rust-lang.org/reference/attributes.html#active-and-inert-attributes) for details about active and inert attributes).

It wouldn't harm if `bon::builder` was imported. It won't shadow the inert `#[builder]` attribute, but the compiler will report that the import of that macro is unused.

:::


To follow the usual Rust builder naming conventions `bon` treats the method named `new` inside of the impl block specially. It generates functions with a bit of different names.

If `#[builder]` is placed on the method called `new`, then the generated functions are called:

| Starting function         | Finishing function
|---------------------------|---------------------
| `builder() -> {T}Builder` | `build(self) -> T`

For any other methods not called `new` and for any free function the naming is a bit different:

| Starting function                              | Finishing function
|------------------------------------------------|---------------------
| `{fn_name}() -> {T?}{PascalCaseFnName}Builder` | `call(self) -> T`

## Builder for a struct

`bon` supports the classic pattern of annotating a struct to generate a builder with the `#[derive(Builder)]` syntax.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
struct User {
    id: u32,
    name: String,
}

let user = User::builder()
    .id(1)
    .name("Bon".to_owned())
    .build();

assert_eq!(user.id, 1);
assert_eq!(user.name, "Bon");
```

::: tip

`#[derive(Builder)]` on a struct generates builder API that is fully compatible with placing `#[builder]` attribute on the `new()` method with a signature similar to the struct's fields.

See [compatibility](./compatibility#switching-between-derive-builder-and-builder-on-the-new-method) page for details.
:::

In general, both `#[derive(Builder)]` on structs and `#[builder]` on functions/methods have almost the same API. We'll use both of them throughout the documentation to provide examples. If the example shows only the usage of one syntax (e.g. `#[builder]`), it's very likely that the other syntax (e.g. `#[derive(Builder)]`) works similarly unless explicitly stated otherwise.

## What's next?

::: tip

If you like the idea of this crate and want to say "thank you" or "keep up doing this" consider giving us a [star ⭐ on Github](https://github.com/elastio/bon). Any support and contribution are appreciated 🐱!

:::

This is just part of what's available in `bon`. You may consider reading the `Basics` section to harness the full power of `bon`.

You can also consult the [API reference index](../reference/builder) that describes all available configuration attributes. This guide will cover most of them but not all. Check the short descriptions of available attributes to see if something might be of immediate interest to you.

However, feel free to skip the docs and just use the `#[builder]` and `#[derive(Builder)]` in your code. They are designed to be intuitive, so they'll probably do the thing you want them to do already.

If you can't figure something out, consult the docs and maybe use that search `🔍 Search` thing at the top to navigate. You may also create an issue or a discussion on the [Github repository](https://github.com/elastio/bon) for help or write us a message on [Discord](https://bon-rs.com/discord) (see below).

Click the "Next page" button at the bottom to proceed with the guide.

## Socials

<table>
<tbody>
    <tr>
        <td>
            <div style="display: flex; align-items: center">
                <VPSocialLink
                    icon="discord"
                    link="https://bon-rs.com/discord"
                />
                <a href="https://bon-rs.com/discord">Discord</a>
            </div>
        </td>
        <td>Here you can leave feedback, ask questions, report bugs, or just write "thank you".</td>
    </tr>
    <tr>
        <td>
            <div style="display: flex; align-items: center">
                <VPSocialLink
                    icon="x"
                    link="https://x.com/veetaha"
                />
                <a href="https://x.com/veetaha" class="nobr">X (Twitter)</a>
            </div>
        </td>
        <td>Profile of the maintainer. There are only posts about <code>bon</code> and Rust in general here.</td>
    </tr>
</tbody>
</table>


## Acknowledgments

This project was heavily inspired by such awesome crates as [`buildstructor`](https://docs.rs/buildstructor), [`typed-builder`](https://docs.rs/typed-builder) and [`derive_builder`](https://docs.rs/derive_builder). This crate was designed with many lessons learned from them.

See [alternatives](./alternatives) for comparison.
