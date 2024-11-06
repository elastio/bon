---
outline: deep
---

# `Into` Conversions In-Depth

## Preface

This is the continuation of [Into Conversions](../basics/into-conversions) from the `Basics` section. This page describes important caveats of using `impl Into` that you should know before enabling them.

Make sure you are familiar with the standard [`From`](https://doc.rust-lang.org/stable/std/convert/trait.From.html) and [`Into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) traits before you proceed. Reading their docs is pretty much enough. For example, you should know that every type that implements `From<T>` automatically implements `Into<T>`. Also, you should know that you can pass a value of type `T` at no cost directly to a function that accepts `impl Into<T>` thanks to [this blanket](https://github.com/rust-lang/rust/blob/1a94d839be8b248b972b9e022cb940d56de72fa1/library/core/src/convert/mod.rs#L763-L771) impl in `std`.

::: warning

This is generally a controversial topic 🐱. Some people like to be more explicit, but others prefer the shorter notation. This also depends on the kind of code you are writing.

If you prefer being explicit in code, feel free not to use `Into` conversions at all. They are fully opt-in. This article isn't prescriptive. The syntax savings are arguably small, so use your best judgement, and refer to this page if you can't decide.

:::

We'll cover the following:

-   [Use `Into` conversions](#use-into-conversions)
-   [Avoid `Into` conversions](#avoid-into-conversions)

## Use `Into` conversions

The main advantage of `impl Into` in setters is that it reduces the boilerplate for the caller. The code becomes shorter and cleaner, although not without [the drawbacks](#avoid-into-conversions).

`Into` conversions usually make sense only if _all of the following_ are true (AND):

::: tip The Rules of `Into`

1. The code where the builder is supposed to be _used_ is not performance-sensitive.
2. The builder is going to be used with literal values a lot or require wrapping the values.

:::

### Shorter syntax for literals

Here is an example that shows the _non-exhaustive_ list of standard types where it's usually fine to enable `Into` conversions.

::: tip

Switch between the UI tabs in the code snippets below to see how the code looks like with `Into Conversions` enabled and with the `Default` syntax.

:::

::: code-group

```rust [Into Conversions]
use bon::Builder;

#[derive(Builder)]
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
use bon::Builder;

#[derive(Builder)]
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

### Performance-sensitive code

If allocations can pose a bottleneck for your application and you need to see every place in code where an allocation is performed, you should avoid using `impl Into` overall. It can lead to implicitly moving data to the heap or cloning it.

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
    // Whoops, we passed a `&String`.              // [!code error]
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

The reason for this error is that `rustc` can't infer the type for the numeric literal `10` because it could be one of the following types: `u8`, `u16`, `u32`, which all implement `Into<u32>`. There isn't a suffix like `10_u16` in this code to tell the compiler the type of the numeric literal `10`. When the compiler can't infer the type of a numeric literal it falls back to assigning the type `i32` for an integer literal and `f64` for a floating point literal. In this case `i32` is inferred, which isn't convertible to `u32`.

Requiring an explicit type suffix in numeric literals would be the opposite of good ergonomics that `impl Into` is trying to achieve in the first place.

### Weakened generics inference

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

Notice how we didn't add a type annotation for the variable `ip_addr`. The compiler can deduce (infer) the type of `ip_addr` because it sees that the variable is passed to the `ip_addr()` setter method that expects a parameter of type `IpAddr`. It's a really simple exercise for the compiler in this case because all the context to do it is there.

However, if you use an `Into` conversion, not even Sherlock Holmes can answer the question "What type did you intend to parse?":

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

In this case, there is a compile error:

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
fn ip_addr(self, value: impl Into<IpAddr>) -> ConnectBuilder<SetIpAddr<S>> { /* */ }
```

This signature implies that the `value` parameter can be of any type that implements `Into<IpAddr>`. There are several types that implement such a trait. Among them: [`Ipv4Addr`](https://doc.rust-lang.org/stable/std/net/struct.Ipv4Addr.html#impl-From%3CIpv4Addr%3E-for-IpAddr) and [`Ipv6Addr`](https://doc.rust-lang.org/stable/std/net/struct.Ipv6Addr.html#impl-From%3CIpv6Addr%3E-for-IpAddr), and, obviously, `IpAddr` itself (thanks to [this blanket impl](https://github.com/rust-lang/rust/blob/1a94d839be8b248b972b9e022cb940d56de72fa1/library/core/src/convert/mod.rs#L763-L771)).

This means the setter for `ip_addr` can no longer hint the compiler a single type that it accepts. Thus the compiler can't decide which type to assign to the `ip_addr` variable in the original code, because _there can be many that make sense_. I.e. the code will compile if any of the `Ipv4Addr` or `Ipv6Addr` or `IpAddr` type annotations are added to the `ip_addr` variable, but the compiler has no right to decide which of them to use on your behalf.

This is the drawback of using not only `impl Into`, but any generics at all.

### `None` literals inference

`impl Into` breaks type inference for `None` literals. For example, this code doesn't use `Into` conversions and compiles fine:

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    member: Option<String>
}

Example::builder()
    // Suppose we want to be explicit about omitting the `member`,
    // so we intentionally invoke the `maybe_` setter and pass `None` to it
    .maybe_member(None)
    .build();
```

Now, let's enable an `Into` conversion for the `member`:

```rust compile_fail
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(into)] // [!code ++]
    member: Option<String>
}

Example::builder()
    .maybe_member(None)
    .build();
```

When we compile this code we get the following error:

```log
.maybe_member(None)                                            // [!code error]
 ------------ ^^^^ cannot infer type of the type parameter `T` // [!code error]
                   declared on the enum `Option`               // [!code error]
```

The problem here is that the compiler doesn't know the complete type of the `None` literal. It definitely knows that it's a value of type `Option<_>`, but it doesn't know what type to use in place of the `_`. There could be many potential candidates for the `_` inside of the `Option<_>`. This is because the signature of the `maybe_member()` setter changed:

```rust ignore
fn maybe_member(self, value: Option<String>) -> ExampleBuilder<SetMember<S>>            // [!code --]
fn maybe_member(self, value: Option<impl Into<String>>) -> ExampleBuilder<SetMember<S>> // [!code ++]
```

Before we enabled `Into` conversions the signature provided a hint for the compiler because the setter expected a single concrete type `Option<String>`, so it was obvious that the `None` literal was of type `Option<String>`.

However, after we enabled `Into` conversions, the signature no longer provides a single concrete type. It says that it accepts an `Option` of any type that implements `Into<String>`.

It means that the `None` literal could be of types `Option<&str>` or `Option<String>`, for example, so the compiler can't decide which one you meant. And this matters, because `Option<&str>` and `Option<String>` are totally different types. Simplified, `Option<&str>` is 16 bytes in size and `Option<String>` is 24 bytes, even when they are `None`.

To work around this problem the caller would need to explicitly specify the generic parameter for the `Option` type when passing the `None` literal:

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(into)] // [!code ++]
    member: Option<String>
}

Example::builder()
    .maybe_member(None::<String>) // [!code focus]
    .build();
```

### Code complexity

This is quite subjective, but `impl Into<T>` is a bit harder to read than just `T`. It makes the signature of the setter slightly bigger and requires you to understand what the `impl Trait` does, and what its implications are.

If you want to keep your code simpler and more accessible (especially for beginner rustaceans), just avoid the `Into` conversions.
