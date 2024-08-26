---
outline: deep
---


# `Into` Conversions In-Depth

## Preface

This is the continuation of the ["Into conversions" section](../overview#into-conversions) from the general overview page. This page describes the important caveats of using `impl Into` that you should know before enabling them.

Make sure you are familiar with the standard [`From`](https://doc.rust-lang.org/stable/std/convert/trait.From.html) and [`Into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) traits before you proceed. Reading their docs is pretty much enough. For example, you should know that you can pass a value of type `T` directly with zero overhead to a function that accepts `impl Into<T>` thanks to [this blanket](https://github.com/rust-lang/rust/blob/1a94d839be8b248b972b9e022cb940d56de72fa1/library/core/src/convert/mod.rs#L763-L771) impl in `std`.

::: warning

This is genenerally a controversial topic 🐱. Some people like to be more explicit, but others prefer the shorter notation. This also depends on the kind of code you are writing.

If you prefer being explicit in code, feel free not to use `Into` conversions at all. They are fully opt-in. This article isn't prescriptive. The syntax savings are arguably small, so use your best judgement, and refer to this article if you can't decide.

:::

We'll cover the following:

- [Use `Into` conversions](#reasons-to-use-into-conversions)
- [Avoid `Into` conversions](#reasons-to-avoid-into-conversions)

## Use `Into` conversions

The main advantage of `impl Into` in setters is that it reduces the boilerplate for the caller. The code becomes shorter and cleaner, although not without [the drawbacks](#reasons-to-avoid-into-conversions).

`Into` conversions usually make sense only if *all of the following* are true (AND):

::: tip The Rules of `Into`

1. The code where the builder is supposed to be *used* is not performance sensitive.
2. The builder is going to be used with literal values a lot or require wrapping the values.

:::


### Shorter syntax for literals

Here is an example that shows the *non-exhaustive* list of standard types where it's usually fine to enable into conversions.

::: tip

Use the tabs in the code snippets below to see how the code looks like with `Into Conversions` enabled and with the `Default` syntax.

:::

::: code-group
```rust [Into Conversions]
use bon::builder;

#[builder]
struct Example {
    #[builder(into)] // [!code highlight]
    string: String,

    #[builder(into)]                                                            // [!code highlight]
    path_buf: std::path::PathBuf,

    #[builder(into)] // [!code highlight]
    ip_addr: std::net::IpAddr,
}

Example::builder()
    // We can pass `&str` literal                                        // [!code highlight]
    .string("string literal")                                            // [!code highlight]
    // We can pass `&str` literal or a String                            // [!code highlight]
    .path_buf("string/literal")                                          // [!code highlight]
    // We can pass an array of IP components or `Ipv4Addr` or `Ipv6Addr` // [!code highlight]
    .ip_addr([127, 0, 0, 1])                                             // [!code highlight]
    .build();
```

```rust [Default]
use bon::builder;

#[builder]
struct Example {
    // No attributes
    string: String,

    // No attributes
    path_buf: std::path::PathBuf,

    // No attributes
    ip_addr: std::net::IpAddr,
}

Example::builder()
    // We have to convert `&str -> String` manually
    .string("string literal".to_owned())
    // We have to convert `&str -> PathBuf` manually
    .path_buf("string/literal".into())
    // We have to convert `[u8; 4] -> IpAddr` manually
    .ip_addr([127, 0, 0, 1].into())
    .build();
```
:::

### Automatic enum wrapping

If you are working with enums a lot, you may implement the `From<EnumVariant>` for your enum and avoid wrapping your enum variants when passing them to the builder. Pay attention to the difference in the focused code below.

::: code-group

```rust [Into Conversions]
use bon::builder;

#[builder]
fn evaluate(#[builder(into)] expr: Expr) { /* */ }  // [!code focus]

evaluate()
    .expr(BinaryExpr { // [!code focus]
        /* */          // [!code focus]
    })                 // [!code focus]
    .call();

enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr)
}

struct BinaryExpr { /* */ }
struct UnaryExpr { /* */ }

impl From<BinaryExpr> for Expr {
    fn from(expr: BinaryExpr) -> Self {
        Self::Binary(expr)
    }
}

impl From<UnaryExpr> for Expr {
    fn from(expr: UnaryExpr) -> Self {
        Self::Unary(expr)
    }
}
```

```rust [Default]
use bon::builder;

#[builder]
fn evaluate(expr: Expr) { /* */ }  // [!code focus]

evaluate()
    .expr(Expr::Binary(BinaryExpr {  // [!code focus]
        /* */                        // [!code focus]
    }))                              // [!code focus]
    .call();

enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr)
}

struct BinaryExpr { /* */ }
struct UnaryExpr { /* */ }

impl From<BinaryExpr> for Expr {
    fn from(expr: BinaryExpr) -> Self {
        Self::Binary(expr)
    }
}

impl From<UnaryExpr> for Expr {
    fn from(expr: UnaryExpr) -> Self {
        Self::Unary(expr)
    }
}
```

:::

As you can see, the difference isn't significant in this case. It makes more sense when you have deeply nested enums.

## Avoid `Into` conversions

### Performance sensitive code

If allocations can pose a bottleneck for you application and you need to see every place in code where an allocation is performed, you should avoid using `impl Into` overall. It can lead to implicitly moving data to heap or cloning it.

**Example:**

```rust
use bon::builder;

#[builder]
fn process_heavy_json(#[builder(into)] data: String) { /* */ }

let json = String::from(
    r#"{
        "key": "Pretend this is a huge JSON string with hundreds of MB in size"
    }"#
);

process_heavy_json()
    // Whooops, we passed a `&String`.             // [!code error]
    // The builder will clone the data internally. // [!code error]
    .data(&json)                                   // [!code error]
    .call();
```

The problem here is that we unintentionally passed a `String` by reference instead of moving the ownership of the `String` to `process_heavy_json()`. This code implicitly uses [this `From` impl](https://github.com/rust-lang/rust/blob/1a94d839be8b248b972b9e022cb940d56de72fa1/library/alloc/src/string.rs#L2774-L2784) from the standard library.

### Primitive numeric literals

`impl Into` breaks type inference for numeric literal values. For example, the following code doesn't compile.

```rust compile_fail
fn half(x: impl Into<u32>) -> u32 {
    x.into() / 2
}

half(10); // [!code error]
```

The compile error is the following ([Rust playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6b1b38e0de6f7747dc1ea3975fcffc06)):
```log
half(10);
---- ^^ the trait `std::convert::From<i32>` is not implemented for `u32`,
|       which is required by `{integer}: std::convert::Into<u32>`
|
required by a bound introduced by this call
```

The reason for this error is that `rustc` can't infer the type for the number literal `10`, because it could be one of the following types: `u8`, `u16`, `u32`, which all implement `Into<u32>`. There isn't a suffix like `10_u16` in this code to tell the compiler the type of the number literal `10`. When compiler can't infer the type of a numeric literal if falls back to assigning the type `i32` for an integer literal and `f64` for a floating point literal. In this case `i32` is inferred, which isn't convertible to `u32`.

Requiring an explicit type suffix in numeric literals would be the opposite of good ergonomics that `impl Into` is trying to achieve in the first place.

::: info

Type inference for primitive numeric types falls into the category of the [section below](#weakened-type-inference), but it's separated from it because this kind of type inference is generally much less obvious.

:::

### Weakened type inference

If you have a function that returns a generic type, then the compiler needs to infer that generic type from usage unless it's specified explicitly. A classic example of such a function is [`str::parse()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) or [`serde_json::from_str()`](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html).

**Example:**

```rust
use bon::builder;
use std::net::IpAddr;

#[builder]
fn connect(ip_addr: IpAddr) { /* */ }

let ip_addr = "127.0.0.1".parse().unwrap();

connect()
    .ip_addr(ip_addr)
    .call();
```
Notice how we didn't add a type annotation for the variable `ip_addr`. The compiler can deduce (infer) the type of `ip_addr` because it sees that the variable is passed to the `connect()` function that expects
the type `IpAddr`. It's a really simple exercise for the compiler in this case because all the context to solve it is there.

However, if use an `Into` conversion, not even Sherlock Holmes can answer the question "What type did you intend to parse?":

```rust ignore
use bon::builder;
use std::net::IpAddr;

#[builder]
fn connect(ip_addr: IpAddr) { /* */ }                  // [!code --]
fn connect(#[builder(into)] ip_addr: IpAddr) { /* */ } // [!code ++]

let ip_addr = "127.0.0.1".parse().unwrap();

connect()
    .ip_addr(ip_addr)
    .call();
```

In this case there is a compile error:

```log
error[E0284]: type annotations needed                                    // [!code error]
  |                                                                      // [!code error]
9 |     let ip_addr = "127.0.0.1".parse().unwrap();                      // [!code error]
  |         ^^^^^^^               ----- type must be known at this point // [!code error]
  |                                                                      // [!code error]
  = note: cannot satisfy `<_ as std::str::FromStr>::Err == _`            // [!code error]
help: consider giving `ip_addr` an explicit type                         // [!code error]
  |                                                                      // [!code error]
9 |     let ip_addr: /* Type */ = "127.0.0.1".parse().unwrap();          // [!code error]
  |                ++++++++++++                                          // [!code error]
```

This is because now the `ip_addr` setter looks like this:

```rust ignore
fn ip_addr(self, value: impl Into<IpAddr>) -> NextBuilderState { /* */ }
```

This signature implies that `value` parameter can be of any type that implements `Into<IpAddr>`. There are several types that implement such a trait. Among them: [`Ipv4Addr`](https://doc.rust-lang.org/stable/std/net/struct.Ipv4Addr.html#impl-From%3CIpv4Addr%3E-for-IpAddr) and [`Ipv6Addr`](https://doc.rust-lang.org/stable/std/net/struct.Ipv6Addr.html#impl-From%3CIpv6Addr%3E-for-IpAddr), and, obviously, `IpAddr` itself (thanks to [this blanket impl](https://github.com/rust-lang/rust/blob/1a94d839be8b248b972b9e022cb940d56de72fa1/library/core/src/convert/mod.rs#L763-L771)).

This means the setter for `ip_addr` can no longer hint the compiler a single type that it accepts. Thus the compiler can't decide which type to assign to the `ip_addr` variable in the original code, because *there can be many that make sense*. I.e. the code will compile if any of the `Ipv4Addr` or `Ipv6Addr` or `IpAddr` type annotations are added to the `ip_addr` variable, but the compiler has no right to decide which of them to use on your behalf.

This is the drawback of using not only `impl Into`, but any generics at all.

### Code complexity

This quite subjective, but `impl Into<T>` is a bit harder to read than just `T`. It makes the signature of the setter slightly bigger and requires you to understand what the `impl Trait` does, and what its implications are.

If you want to keep your code simpler and more accessible (especially for beginner rustaceans), just avoid the `Into` conversions.

*[Member]: Struct field or a function argument
*[member]: Struct field or a function argument
*[members]: Struct fields or function arguments
