---
outline: deep
---

# `with`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Overrides setters' signature and applies a custom conversion.

You can specify the signature and the conversion either with the closure syntax or with a [well-known function](#well-known-functions).

| Example                                                                            | Meaning                                      |
| ---------------------------------------------------------------------------------- | -------------------------------------------- |
| `#[builder(with = \|...\| body)]`                                                  | [Infallible closure](#infallible-closure)    |
| <code class="nobr">#[builder(with = \|...\| -> \*Result<\_[, E]> { body })]</code> | [Fallible closure](#fallible-closure)        |
| `#[builder(with = FromIterator::from_iter)]`                                       | [Well-known function](#well-known-functions) |

## Closure Syntax

If you specify a closure, its input parameters will become the input parameters of the setters.

### Infallible Closure

The simplest form of the custom closure is an infallible closure. It _must not_ have a return type annotation and it _must_ return the value of the _underlying_ member's type.

::: code-group

```rust [Struct]
use bon::Builder;

struct Point {
    x: u32,
    y: u32,
}

#[derive(Builder)]
struct Example {
    #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
    point: Point,
}

let value = Example::builder()
    .point(2, 3) // [!code highlight]
    .build();

assert_eq!(value.point.x, 2);
assert_eq!(value.point.y, 3);
```

```rust [Function]
use bon::builder;

struct Point {
    x: u32,
    y: u32,
}

#[builder]
fn example(
    #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
    point: Point,
) -> Point {
    point
}

let value = example()
    .point(2, 3) // [!code highlight]
    .call();

assert_eq!(value.x, 2);
assert_eq!(value.y, 3);
```

```rust [Method]
use bon::bon;

struct Point {
    x: u32,
    y: u32,
}

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
        point: Point,
    ) -> Point {
        point
    }
}

let value = Example::example()
    .point(2, 3) // [!code highlight]
    .call();

assert_eq!(value.x, 2);
assert_eq!(value.y, 3);
```

:::

### Fallible Closure

You can add a return type annotation to the closure to signify that it's fallible. The closure is expected to return a `Result` with the `Ok` variant of the member's underlying type. This will make the setter fallible.

<!-- #region fallible-closure-example -->

::: code-group

```rust [Struct]
use bon::Builder;
use std::num::ParseIntError;

#[derive(Builder)]
struct Example {
    #[builder(with = |string: &str| -> Result<_, ParseIntError> { // [!code highlight]
        string.parse()                                            // [!code highlight]
    })]                                                           // [!code highlight]
    x1: u32,                                                      // [!code highlight]
}

fn main() -> Result<(), ParseIntError> {
    Example::builder()
        .x1("99")? // <-- the setter returns a `Result` // [!code highlight]
        .build();

    Ok(())
}
```

```rust [Function]
use bon::builder;
use std::num::ParseIntError;

#[builder]
fn example(
    #[builder(with = |string: &str| -> Result<_, ParseIntError> { // [!code highlight]
        string.parse()                                            // [!code highlight]
    })]                                                           // [!code highlight]
    x1: u32,                                                      // [!code highlight]
) -> u32 {
    x1
}

fn main() -> Result<(), ParseIntError> {
    example()
        .x1("99")? // <-- the setter returns a `Result` // [!code highlight]
        .call();

    Ok(())
}
```

```rust [Method]
use bon::bon;
use std::num::ParseIntError;


struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(with = |string: &str| -> Result<_, ParseIntError> { // [!code highlight]
            string.parse()                                            // [!code highlight]
        })]                                                           // [!code highlight]
        x1: u32,                                                      // [!code highlight]
    ) -> u32 {
        x1
    }
}

fn main() -> Result<(), ParseIntError> {
    Example::example()
        .x1("99")? // <-- the setter returns a `Result` // [!code highlight]
        .call();

    Ok(())
}
```

:::

<!-- #endregion fallible-closure-example -->

The return type annotation must be of the form `*Result<_[, E]>`.

Here `*Result` means the type must have a `Result` suffix. `[, E]` means the error type annotation is optional.

Examples of valid return types:

- `Result<_, MyError>`
- `Result<_>`
- `my_crate::Result<_>`
- `ApiResult<_>`

The symbol `_` must be specified verbatim. You don't need to repeat the underlying type of the member there.

### Generics

You can reference generic parameters defined on the underlying `struct`, `fn` or the surrounding `impl` block.
You can also use `impl Trait` for parameters in the closure, even though you can't in regular Rust:

```rust ignore
#[builder(with = |value: impl Trait| /**/)]
```

You can't declare new generic parameters. If `impl Trait` isn't enough for you, consider defining a [custom method](../../../guide/typestate-api/custom-methods) on the builder.

### Optional members

For members of type `Option<T>` without [`#[builder(required)]`](./required), the closure needs to return a value of type `T` or `Result<T>`.

The `maybe_` setter's input type depends on the number of input parameters in the closure:

- If there is a single input parameter of type `T`, then the `maybe_` setter accepts `Option<T> `
- If there are two or more input parameters of types `T1, T2, ...`, then the maybe setter accepts `Option<(T1, T2, ...)>`

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(with = |value: u32| value * 2)]
    x1: Option<u32>,

    #[builder(with = |a: u32, b: u32| a * b)]
    x2: Option<u32>,
}

let value = Example::builder()
    .maybe_x1(Some(2))      // [!code highlight]
    .maybe_x2(Some((2, 3))) // [!code highlight]
    .build();

assert_eq!(value.x1, Some(4));
assert_eq!(value.x2, Some(6));
```

## Well-Known Functions

There are several well-known functions that you can specify instead of the closure to shorten your code.
All of them have an equivalent closure syntax, so they are just pure syntax sugar.

### `FromIterator::from_iter`

**Equivalent closure syntax:**

```rust ignore
#[builder(with = |iter: impl IntoIterator<...>| FromIterator::from_iter(iter))]
#[builder(with = |iter: impl IntoIterator<...>| iter.into_iter().collect())]
```

Makes the setter accept `impl IntoIterator<Item = T>` or `impl IntoIterator<Item = (K, V)>`.

This attribute can be used with custom and 3-rd party collections such as [`indexmap::IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html), not only with the `std::collections`. The only requirement is for the collection type to implement the `FromIterator` trait and match the following naming patterns:

- `*Vec<T, ...>`
- `*Set<T, ...>`
- `*Map<K, V, ...>`
- `*Deque<T, ...>`
- `*Heap<T, ...>`
- `*List<T, ...>`

The `...` means there may be any number of other generic parameters (including zero), which could be used to pass a custom hasher or allocator.

This attribute can be specified using one of the two equivalent forms:

```attr
#[builder(with = FromIterator::from_iter)]
#[builder(with = <_>::from_iter)]
```

::: tip General Rust tip

`<_>::method()` is valid Rust syntax, although not everyone knows about it 😉.

For example, if you write `Default::default()` in your regular Rust code, then try writing `<_>::default()` instead, it'll compile. This notation asks the compiler to infer the _trait_ for the method. Importantly, this notation only works with trait methods. You can't use this syntax for inherent methods.

:::

::: code-group

```rust [Struct]
use bon::Builder;
use std::collections::BTreeMap;

#[derive(Builder)]
struct Example {
    #[builder(with = FromIterator::from_iter)] // [!code highlight]
    x1: Vec<u32>,

    #[builder(with = <_>::from_iter)] // [!code highlight]
    x2: BTreeMap<u32, u32>
}

Example::builder()
    // Accepts `impl IntoIterator<Item = u32>`
    .x1([1, 2, 3]) // [!code highlight]
    // Accepts `impl IntoIterator<Item = (u32, u32)>`
    .x2([       // [!code highlight]
        (1, 2), // [!code highlight]
        (3, 4), // [!code highlight]
    ])          // [!code highlight]
    .build();
```

```rust [Function]
use bon::builder;
use std::collections::BTreeMap;

#[builder]
fn example(
    #[builder(with = FromIterator::from_iter)] // [!code highlight]
    x1: Vec<u32>,

    #[builder(with = <_>::from_iter)] // [!code highlight]
    x2: BTreeMap<u32, u32>
) {}

example()
    // Accepts `impl IntoIterator<Item = u32>`
    .x1([1, 2, 3]) // [!code highlight]
    // Accepts `impl IntoIterator<Item = (u32, u32)>`
    .x2([       // [!code highlight]
        (1, 2), // [!code highlight]
        (3, 4), // [!code highlight]
    ])          // [!code highlight]
    .call();
```

```rust [Method]
use bon::bon;
use std::collections::BTreeMap;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(with = FromIterator::from_iter)] // [!code highlight]
        x1: Vec<u32>,

        #[builder(with = <_>::from_iter)] // [!code highlight]
        x2: BTreeMap<u32, u32>
    ) {}
}

Example::example()
    // Accepts `impl IntoIterator<Item = u32>`
    .x1([1, 2, 3]) // [!code highlight]
    // Accepts `impl IntoIterator<Item = (u32, u32)>`
    .x2([       // [!code highlight]
        (1, 2), // [!code highlight]
        (3, 4), // [!code highlight]
    ])          // [!code highlight]
    .call();
```

:::

### `Some`

**Equivalent closure syntax:**

```rust ignore
#[builder(with = |value: T| Some(value))]
```

Makes the setter accept the value of type `T` assuming the member is of type `Option<T>` with [`#[builder(required)]`](./required).

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(required, with = Some)]
    x1: Option<u32>,
}

Example::builder()
    .x1(1)
    .build();
```

::: tip History

This attribute was added as a way to make setters required for structs generated by [`prost`](https://docs.rs/prost/latest/prost/) from the `proto3` syntax. Protobuf v3 doesn't take apart required and optional fields, so `prost` generates `Option<T>` even for required fields ([original issue comment](https://github.com/elastio/bon/issues/35#issuecomment-2426991137)). However, you can use `#[builder(required, with = Some)]` to mark the fields required in the builder syntax.

:::
