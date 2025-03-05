# Getters

You can generate a getter method for the member with the attribute [`#[builder(getter)]`](../../reference/builder/member/getter). The generated getter is available only when the value for the member was set (i.e. its type state implements the [`IsSet`](./custom-methods#isset-trait) trait).

## Custom Getters

You can define a custom getter method on the builder, that adds some custom logic on top of just getting a value. To do that, generate an "internal" getter with private visibility under a different name and define your own getter that uses it in an impl block.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(getter(name = get_x_internal, vis = ""))]
    x: u32
}

use example_builder::{IsSet, State};

impl<S: State> ExampleBuilder<S> {
    // Getter method that performs additional computations
    fn get_x(&self) -> u32
    where
        S::X: IsSet
    {
        *self.get_x_internal() * 2
    }
}

let builder = Example::builder().x(3);

assert_eq!(builder.get_x(), 6);
```

You can also create custom getters for builder's [native fields](./builder-fields#native-fields), which are module-private by default.
