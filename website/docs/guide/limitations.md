# Limitations

Every tool has its constraints, and `bon` is not an exception. The limitations described below shouldn't generally occur in your day-to-day code, and if they do, there are ways to work around them. If you feel that some of the limitations are unacceptable, feel free to [open an issue] so we consider relaxing some of them.

## Destructuring patterns

Function parameters must be simple identifiers that will be turned into setter methods. Destructuring in function parameters position complicates this logic and thus is rejected by the `#[builder]` macro.

For example, this generates a compile error:

```rust compile_fail
use bon::builder;

#[builder]
fn foo((x, y): (u32, u32)) { // [!code error]
    // ...
}
```

If you need to destructure your arguments, then do it separately inside of the function body.

```rust
use bon::builder;

#[builder]
fn foo(point: (u32, u32)) { // [!code highlight]
    let (x, y) = point;     // [!code highlight]
    // ...
}
```

This limitation may be relaxed in the future by adding a new argument-level attribute that lets developers override the name for the setter method.


## Intra-doc links to `Self` on setter methods

Documentation placed on the original function arguments or struct fields is copied verbatim to the documentation on the generated setter methods of the builder struct. The shortcoming of this approach is that references to `Self` break when moved into the `impl` block of the generated builder struct.

`bon` checks for presence of ``[`Self`]`` and `[Self]` in the documentation on the function arguments and struct fields. If there are any, then a compile error suggesting to use full type name will be generated.

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

## `const` functions

It's possible to place `#[builder]` on top of a `const fn`, but the generated builder methods won't be marked `const`. Under [some conditions](into-conversions#types-that-qualify-for-an-automatic-into-conversion), the generated setter methods make use of the `Into::into` method, which isn't `const`. Except for that, the generated code should be `const`-compatible.

If you have a strong use case that requires full support for `const`, feel free to [open an issue]. We'll figure something out for sure 🐱.

[open an issue]: https://github.com/elastio/bon/issues
