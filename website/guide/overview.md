<script setup>
import { data as version } from '/data/version.data'
import VPSocialLink from "vitepress/dist/client/theme-default/components/VPSocialLink.vue";
const [_, versionWildcard] = version.match(/(\d+.\d+).\d+/);
</script>

# Overview

`bon` is a Rust crate for generating compile-time-checked builders for functions and structs. It also provides idiomatic partial application with optional and named parameters for functions and methods.

If you are new to the concept of builders or named function arguments, and you don't know what problems they may solve for you, then check out the motivational [blog post](../blog/how-to-do-named-function-arguments-in-rust).

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

| Start function            | Finish function
|---------------------------|---------------------
| `builder() -> {T}Builder` | `build(self) -> T`

For any other methods not called `new` and for any free function the naming is a bit different:

| Start function                                | Finish function
|-----------------------------------------------|---------------------
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

## No panics possible

The builders generated by `#[builder]` and `#[derive(Builder)]` use the typestate pattern to make sure all required parameters are filled, and the same setters aren't called repeatedly to prevent unintentional overwrites and typos. If something is wrong, a compile error will be created. There are no potential panics and `unwrap()` calls inside of the builder.

## `Option<T>` values are optional

If your function argument or struct field (or member for short) is of type `Option<T>`, then the generated builder will not enforce setting a value for this member, defaulting to `None`.

It also generates two setters: one accepts `T` and the other accepts `Option<T>`. The first avoids wrapping values with `Some()` on the call site. The second allows passing the `Option<T>` value directly.

```rust
use bon::Builder;

#[derive(Builder)]
struct Projection {
    x: Option<u32>,
    y: Option<u32>,

    // Use an annotation for members of non-`Option` type
    #[builder(default)]
    z: u32,
}

// Both `x` and `y` will be set to `None`, `z` will be set to `0`
Projection::builder().build();

Projection::builder()
    // Pass the value without wrapping it with `Some()`
    .x(10)
    // Or use a `maybe_`-prefixed setter that accepts `Option`
    .maybe_y(Some(20))
    // The APIs generated for `#[builder(default)]` and `Option<T>` are equivalent.
    // `z` will be set to `0` when `build()` is called.
    .maybe_z(None)
    .build();
```

See [optional members](./optional-members) page for details.

## `Into` conversions

If you have members of type `String`, or `PathBuf`, and you need to set them to a hard-coded string literal, then you have to write `.to_owned()` or `.to_string()` or `.into()`.

**Example:**

```rust
use bon::Builder;
use std::path::PathBuf;

#[derive(Builder)]       // [!code focus]
struct Project {         // [!code focus]
    name: String,        // [!code focus]
    description: String, // [!code focus]
    path: PathBuf,       // [!code focus]
}                        // [!code focus]

Project::builder()
    .name("Bon".to_owned())                      // [!code focus]
    .description("Awesome crate 🐱".to_string()) // [!code focus]
    .path("/path/to/bon".into())                 // [!code focus]
    .build();
```

However, you can ask `bon` to generate setters that accept `impl Into<T>` to remove the need for manual conversion.

This can be configured with [`#[builder(into)]`](../reference//builder#into) for a single member or with [`#[builder(on({type}, into))]`](../reference/builder#on) for many members at once.

```rust
use bon::Builder;
use std::path::PathBuf;

// All setters for members of type `String` will accept `impl Into<String>` // [!code highlight]
#[derive(Builder)]                                                          // [!code highlight]
#[builder(on(String, into))]                                                // [!code highlight]
struct Project {
    name: String,
    description: String,

    // The setter only for this member will accept `impl Into<PathBuf>`    // [!code highlight]
    #[builder(into)]                                                       // [!code highlight]
    path: PathBuf,
}

Project::builder()
    // &str is converted to `String` internally    // [!code highlight]
    .name("Bon")
    .description("Awesome crate 🐱")
    // `&str` is converted to `PathBuf` internally // [!code highlight]
    .path("/path/to/your/heart")
    .build();
```

See the ["Into Conversions In-Depth"](./patterns/into-conversions-in-depth) page for more details and important caveats (!).

## What's next?

::: tip

If you like the idea of this crate and want to say "thank you" or "keep up doing this" consider giving us a [star ⭐ on Github](https://github.com/elastio/bon). Any support and contribution are appreciated 🐱!

:::

This is just part of what's available in `bon`. You may consider reading the rest of the `Guide` section to harness the full power of `bon` and understand the decisions it makes. Just click on the "Next page" link at the bottom.

However, feel free to skip the docs and just use the `#[builder]` and `#[derive(Builder)]` in your code. They are designed to be intuitive, so they'll probably do the thing you want them to do already.

If you can't figure something out, consult the docs and maybe use that search `🔍 Search` thing at the top to navigate. You may also create an issue or a discussion in the [Github repository](https://github.com/elastio/bon) for help, or write us a message in [Discord](https://discord.gg/QcBYSamw4c) (see below).

## Socials

<table>
    <tr>
        <td>
            <div style="display: flex; align-items: center">
                <VPSocialLink
                    icon="discord"
                    link="https://discord.gg/8VJ8J3c"
                />
                <a href="https://discord.gg/QcBYSamw4c">Discord</a>
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
</table>


## Acknowledgments

This project was heavily inspired by such awesome crates as [`buildstructor`](https://docs.rs/buildstructor), [`typed-builder`](https://docs.rs/typed-builder) and [`derive_builder`](https://docs.rs/derive_builder). This crate was designed with many lessons learned from them.

See [alternatives](./alternatives) for comparison.

*[Member]: Struct field or a function argument
*[member]: Struct field or a function argument
*[members]: Struct fields or function arguments
