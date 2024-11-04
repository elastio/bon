# Limitations

Every tool has its constraints, and `bon` is not an exception. The limitations described below shouldn't generally occur in your day-to-day code, and if they do, there are ways to work around them. If you feel that some of the limitations are unacceptable, feel free to [open an issue] to ask for relaxing some of them.

## Intra-doc links to `Self` on setter methods

Documentation placed on the original function arguments or struct fields is copied verbatim to the documentation on the generated setter methods of the builder struct. The shortcoming of this approach is that references to `Self` break when moved into the `impl` block of the generated builder struct.

`bon` checks for the presence of ``[`Self`]`` and `[Self]` in the documentation on the function arguments and struct fields. If there are any, then a compile error suggesting to use full type name will be generated.

The following example doesn't compile with the reference to ``[`Self`]``. The fix is to replace that reference with the actual name of the struct ``[`Foo`]``.

```rust compile_fail
use bon::bon;

struct Foo;

#[bon]
impl Foo {
    #[builder]
    fn promote(
        /// Promotes [`Self`] to the level specified in this argument // [!code --]
        /// Promotes [`Foo`] to the level specified in this argument  // [!code ++]
        new_level: String
    ) {}
}
```

## Implicit generic lifetimes

Rust allows omitting generic lifetime parameters of a type in function parameters.

**Example:**

```rust compile_fail
use bon::builder;

struct User<'a> {
    name: &'a str
}

#[builder]
fn example(value: User) {} // [!code error]
```

In this example, the type `User` is referenced in the function argument without lifetime parameters. This breaks the logic of the `#[builder]` macro. It must know **all** the lifetime parameters involved in the function signature to properly generate lifetime parameters for the builder struct.

Unfortunately, macros in Rust don't have access to semantic information. All that macros see is just a sequence of syntax tokens. All that the `#[builder]` macro sees in the example above is just this:

```rust ignore
fn example(value: User) {}
```

This means `#[builder]` macro thinks as if there are no lifetime parameters in the `User` type, and thus it generates the wrong code that doesn't compile.

To fix this, we need to make it clear to the `#[builder]` macro that `User` expects some lifetime parameters. This can be done like this:

```rust compile_fail
#[builder]
fn example(value: User) {}     // [!code --]
fn example(value: User<'_>) {} // [!code ++]
```

If you want to make sure your code doesn't accidentally omit a generic lifetime parameter you may enable the `rustc` lint called [`elided_lifetimes_in_paths`](https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html). This lint is `allow` by default, you can enable it in your `Cargo.toml` like this:

```toml
[package.lints.rust]
elided_lifetimes_in_paths = "warn"
```

## `const` functions

It's possible to place `#[builder]` on top of a `const fn`, but the generated builder methods won't be marked `const`. Under [some conditions](into-conversions#types-that-qualify-for-an-automatic-into-conversion), the generated setter methods make use of the `Into::into` method, which isn't `const`. Except for that, the generated code should be `const`-compatible.

If you have a strong use case that requires full support for `const`, feel free to [open an issue]. We'll figure something out for sure 🐱.

## Conditional compilation

Conditionally-compiled members aren't supported yet. The blocker for this feature is a lack of support for attributes in `where` bounds in the language. See [rust-lang/rust/#115590](https://github.com/rust-lang/rust/issues/115590) for details.

[open an issue]: https://github.com/elastio/bon/issues
