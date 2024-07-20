# `Into` conversions

## Problem statement

It's often annoying to have to do a type conversion manually when passing a value to a setter. For example, suppose you have a function that accepts a `String`. You want to pass a string slice such as `"Bon"` to that function. However, to do that you need to explicitly call `"Bon".to_owned()` or `"Bon".to_string()` to do a `&str -> String` conversion at the call site. This is inconvenient for APIs that are often invoked with hardcoded string values.

**Example:**

```rust
struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }
}

let user = User::new("Bon".to_owned());
```

That `.to_owned()` call is just boilerplate that we'd like to avoid. A common workaround for this problem is to let the function accept `impl Into<String>`.

```rust ignore
struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {  // [!code --]
        Self { name }               // [!code --]
    fn new(name: impl Into<String>) -> Self {  // [!code ++]
        Self { name: name.into() }             // [!code ++]
    }
}

let user = User::new("Bon".to_owned()); // [!code --]
let user = User::new("Bon");            // [!code ++]
```

This makes it possible for the caller to pass a `&str`. However, the signature of the function becomes a bit more complex and an `into()` conversion has to be invoked inside of the function implementation manually. So this approach just shifts the boilerplate from the caller to the callee.

## How `bon` solves this problem

The `#[builder]` macro automatically adds `impl Into` in the setter methods and invokes the `into()` conversion internally.

**Example:**

```rust
use bon::bon;

struct User {
    name: String,
}

#[bon] // [!code highlight]
impl User {
    #[builder] // [!code highlight]
    fn new(name: String) -> Self { // [!code highlight]
        Self { name }
    }
}

let user = User::builder()
    .name("Bon") // [!code highlight]
    .build();
```

::: details This also works when `#[builder]` is placed on top of a free function or a struct.

**Example:**

::: code-group

```rust [Struct]
use bon::builder;

#[builder] // [!code highlight]
struct User {
    name: String // [!code highlight]
}

let user = User::builder()
    .name("Bon") // [!code highlight]
    .build();
```

```rust [Function]
use bon::builder;

#[builder] // [!code highlight]
fn accept_string(
    name: String // [!code highlight]
) {}

let user = accept_string()
    .name("Bon") // [!code highlight]
    .call();
```

:::

We didn't need to add any more attributes for `bon` to figure out that the setter for `name` needs to accept `impl Into<String>`. We also didn't change the signature of `new()`, so it still accepts a `String`. This is because `bon` automatically performs an `into()` conversion inside of the `name()` setter method.


## Types that qualify for an automatic `Into` conversion

An automatic `Into` conversion in setter methods applies only to types that are represented by a simple path (e.g. `crate::foo::Bar`) or a simple identifier (e.g. `Bar`, `String`) with the exception of primitive types.

The following list describes the types that don't qualify for an automatic `Into` conversion with the explanation of the reason.

1. Primitive types
    Unsigned integers                    | Signed integers                       | Floats     | Other
    -------------------------------------|---------------------------------------|------------|--------------
    `u8` `u16` `u32` `u64` `u128` `usize`| `i8` `i16` `i32` `i64` `i128` `isize` | `f32` `f64`| `bool` `char`

    ::: details Primitive types aren't qualified for an automatic `Into` conversion for several reasons

    First, it's because `impl Into` breaks type inference for numeric literal values. For example, the following code doesn't compile.

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

    Requiring an explicit type suffix in numeric literals would be the opposite of what `bon` tries to achieve with its focus on great ergonomics.

    ---

    The second reason is that it's just conventional not to use `impl Into` for primitive types in Rust. There aren't many types that implement `Into<bool>` or `Into<char>`, for example. It also keeps simple things (primitive values) simple in the method signatures of the generated builder. Hints in IDE become easier to read.
    :::

2. `impl Trait` in function parameter types
    ::: details The reason is that it leads to unnecessarily nested generics that may block type inference.

    **Example:**
    ```rust
    use bon::builder;

    #[builder]
    fn greet(name: impl Into<String>) {
        let name = name.into();
        println!("Hello {name}")
    }

    greet().name("Bon").call();
    ```

    In this case the `name` parameter already uses an explicit `Into` conversion. There is no need for `bon` to add an `impl Into<impl Into<String>>` conversion on top of that because it would complicate type inference for `rustc`.

    The compiler would need to infer two generic types in this case for each of the `impl Into`, which it may not always do automatically and it would require providing type hints manually. That would break ergonomics promised by `bon`.
    :::

3. Generic types from the function signature, surrounding `impl` block or struct's declaration.
    ::: details The reason is similar to the previous item.

    **Example:**
    ```rust
    use bon::builder;

    #[builder]
    fn greet<T: Into<String>>(name: T) {
        let mut name = name.into();
        println!("Hello {name}")
    }

    greet().name("Bon").call();
    ```

    `bon` avoids a nested `impl Into<T>` where `T: Into<String>` to prevent type inference from stalling.
    :::

4. Tuples, arrays, references, function pointers and other type expressions.

    **Examples:** `&str`, `&mut String`, `[u8; 10]`, `(u32, u32)`, `&dyn Trait`.

    ::: details The reason is that analysis of complex type expressions is also complex.

    The goal of the automatic `Into` conversions is to spare the caller from converting the types at the call site if an `Into` conversion exists. There aren't many types that implement `Into` conversions to complex type expressions involving references, tuples, arrays, function pointers etc.

    Anyhow, there is likely a subset of simple type expressions for which `bon` may provide an automatic `Into` conversion. If you have a use case that needs such conversions to be automatic, you may [override the default behavior](#override-the-default-behavior) and consider to [open an issue].
    :::

## Override the default behavior

Suppose automatic `Into` conversion qualification rules don't satisfy your use case. For example, you want the setter method to accept an `Into<(u32, u32)>` then you can use an explicit `#[builder(into)]` to override the default behavior. See [this attribute's docs](../reference/builder#into) for details.




[open an issue]: https://github.com/elastio/bon/issues
