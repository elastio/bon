# Limitations

Every tool has its constraints, and `bon` is not an exception. The limitations described below shouldn't generally occur in your day-to-day code, and if they do, there are ways to work around them. If you feel that some of the limitations are unacceptable, feel free to [open an issue] to ask for some of them to be relaxed.

## Intra-Doc Links to `Self` on Setter Methods

Documentation placed on the original function arguments or struct fields is copied verbatim to the documentation on the generated setter methods of the builder struct. The shortcoming of this approach is that references to `Self` break when moved into the `impl` block of the generated builder struct.

`bon` checks for the presence of ``[`Self`]`` and `[Self]` in the documentation on the function arguments and struct fields. If there are any, then a compile error suggesting to use the full type name will be generated.

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

## Implicit Generic Lifetimes

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

This means the `#[builder]` macro thinks as if there are no lifetime parameters in the `User` type, and thus it generates the code that doesn't compile.

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

## Destructuring Patterns in Function Parameters

When `#[builder]` is placed on a function (including associated methods), then the parameters of that function must be simple identifiers. Destructuring patterns aren't supported because it's not obvious what identifier to assign to the member in this case. This identifier will appear in `#[builder(default = ...)]` expressions, for example.

Instead, if you need to destructure your function parameter, just do that inside of the function's body.

**Example:**

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

## Formatting of Attributes on Function Arguments

At the time of this writing, `rustfmt` does a fairly bad job of formatting attributes placed on function arguments. Here is an example of `rustfmt`-formatted code that uses `#[bon::builder]`:

```rust
#[bon::builder]
fn example(
    #[builder(default = 1)] foo: u32,
    bar: u32,
    fizz: u32,
) {
}
```

The attribute on the function's parameter was formatted on the same line with the parameter itself, even though the signature of the function already takes up multiple lines. It is harder to read the signature this way because the names of function parameters aren't aligned.

As a workaround, you can place a dummy line comment right after the attribute to prevent `rustfmt` from placing the attribute on the same line with the function parameter:

**Example:**

```rust
#[bon::builder]
fn example(
    #[builder(default = 1)] // // [!code highlight]
    foo: u32,
    bar: u32,
    fizz: u32,
) {
}
```

Another workaround for this is to write a doc comment on top of the function argument:

```rust
#[bon::builder]
fn example(
    /// Doc comment fixes formatting // [!code highlight]
    #[builder(default = 1)]
    foo: u32,
    bar: u32,
    fizz: u32,
) {
}
```

Here is [the related issue](https://github.com/rust-lang/rustfmt/issues/6276) in `rustfmt` about this problem.

## `const` Evaluation

It's possible to place `#[builder]` on top of a `const fn`, but the generated builder methods won't be marked as `const`. Note that even if `bon` adds support for `const` builders, some features won't work for them. For example:

- `#[builder(default)]` can not work because `Default::default()` is not `const`. You'd have to write `#[builder(default = ...)]` instead to construct the default value manually
- `#[builder(into)]` can not work because `Into::into()` is not `const`

If you have a strong use case that requires full support for `const`, feel free to leave a 👍 on [this issue](https://github.com/elastio/bon/issues/169).

[open an issue]: https://github.com/elastio/bon/issues
