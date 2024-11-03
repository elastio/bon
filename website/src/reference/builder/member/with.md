# `with`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

TODO: add docs (update the short descriptions on the parent page)


If the closure accepts a single parameter `T`, then the `maybe_` setter accepts `Option<T>`. Tuple is unnecessary in this case.

You can use `impl Trait` for parameters in the closure, even though you can't in the usual Rust code.

There are several well-known functions that you can use instead of the closure syntax to shorten your code e.g. `#[builder(with = Some)]` and `#[builder(with = FromIterator::from_iter)]` (more details [here](../../reference/builder/member/with#well-known-functions)).

## Optional members

<!-- ```rust
// The `maybe_` setter accepts `Option<(u32, u32)>`
Example::builder().maybe_x2(Some((4, 2)));
``` -->


## Well-Known Functions


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
