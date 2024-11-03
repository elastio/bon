# `with`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Overrides setters' signature and applies a custom conversion.

You can specify the signature and the conversion either with the closure syntax or with a [well-known function](#well-known-functions)

| Form                                                                              | Meaning
|-----------------------------------------------------------------------------------|----------------------------
| `#[builder(with = \|...\| body)]`                                                 | Custom *infallible* closure
| <code class="nobr">#[builder(with = \|...\| -> \*Result<_[, E]> { body })]</code> | Custom *fallible* closure
| `#[builder(with = well_known_function)]`                                          | One of the [well-known functions](#well-known-functions)

## Closure syntax

The simplest form of the custom closure is an *infallible* closure. It *must not* have a return type annotation and it *must* return the value of the *underlying* member's type. If the member is of type `Option<T>` without [`#[builder(transparent)]`](./transparent), then the *underlying* member's type is `T`.

```rust
use bon::Builder;

struct Point {
    x: u32,
    y: u32,
}

#[derive(Builder)]
struct Example {
    #[builder(with = |x: u32, y: u32| Point { x, y })]
    point: Point,
}

let value = Example::builder()
    .point(2, 3)
    .build();

assert_eq!(value.point.x, 2);
assert_eq!(value.point.y, 3);
```


If the closure accepts a single parameter `T`, then the `maybe_` setter accepts `Option<T>`. Tuple is unnecessary in this case.

You can use `impl Trait` for parameters in the closure, even though you can't in the usual Rust code.

<!-- There are several well-known functions that you can use instead of the closure syntax to shorten your code e.g. `#[builder(with = Some)]` and `#[builder(with = FromIterator::from_iter)]` (more details [here](../../reference/builder/member/with#well-known-functions)). -->

## Optional members

<!-- ```rust
// The `maybe_` setter accepts `Option<(u32, u32)>`
Example::builder().maybe_x2(Some((4, 2)));
``` -->

## Fallible setters

You can add a return type annotation to the closure to signify that it's fallible. In this case a fallible setter will be generated.

```rust
use bon::Builder;
use std::num::ParseIntError;

#[derive(Builder)]
struct Example {
    #[builder(with = |string: &str| -> Result<_, ParseIntError> { // [!code focus]
        string.parse()                                            // [!code focus]
    })]                                                           // [!code focus]
    x1: u32,                                                      // [!code focus]
}

fn main() -> Result<(), ParseIntError> {
    Example::builder()
        .x1("99")? // <-- the setter returns a `Result` // [!code focus]
        .build();

    Ok(())
}
```


## Well-Known Functions
