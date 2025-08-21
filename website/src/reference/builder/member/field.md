# `field`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Defines a private field on the builder without setters. This field may be used in [Custom Methods](../../../guide/typestate-api/custom-methods). The value of this field will be moved into the resulting struct or function from which the builder was generated.

The initial value for the field will be computed inside of the starting function and stored in the builder.

| Form                             | How value for the member is computed |
| -------------------------------- | ------------------------------------ |
| `#[builder(field)]`              | `Default::default()`                 |
| `#[builder(field = expression)]` | `expression`                         |

::: tip

This attribute is similar to [`#[builder(skip)]`](./skip). The difference is that the latter computes the value in the finishing function and doesn't store it in the builder.

:::

## Example

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(field)] // [!code highlight]
    levels: Vec<u32>,
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_builder::State> ExampleBuilder<S> {
    fn level(mut self, value: u32) -> Self {
        // `self.levels` is accessible in the builder // [!code highlight]
        self.levels.push(value); // [!code highlight]
        self
    }
}

let example = Example::builder()
    .level(2)
    .level(99)
    .build();

assert_eq!(example.levels, [2, 99]);
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(field)] // [!code highlight]
    levels: Vec<u32>,
) -> Vec<u32> {
    levels
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_builder::State> ExampleBuilder<S> {
    fn level(mut self, value: u32) -> Self {
        // `self.levels` is accessible in the builder // [!code highlight]
        self.levels.push(value); // [!code highlight]
        self
    }
}

let example = example()
    .level(2)
    .level(99)
    .call();

assert_eq!(example, [2, 99]);
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn method(
        #[builder(field)] // [!code highlight]
        levels: Vec<u32>,
    ) -> Vec<u32> {
        levels
    }
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_method_builder::State> ExampleMethodBuilder<S> {
    fn level(mut self, value: u32) -> Self {
        // `self.levels` is accessible in the builder // [!code highlight]
        self.levels.push(value); // [!code highlight]
        self
    }
}

let example = Example::method()
    .level(2)
    .level(99)
    .call();

assert_eq!(example, [2, 99]);
```

:::

## Evaluation Context

You can reference other members marked with [`#[builder(start_fn)]`](./start_fn) or `#[builder(field)]` as variables in the `field` expression. All members are initialized in the order of their declaration, and thus only members that are declared earlier (higher) in the code are available for the `field` expression.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(start_fn)]
    x1: u32,

    // Note that here we don't have access to `x3`
    // because it's declared (and thus initialized) later
    #[builder(field = 2 * x1)]
    x2: u32,

    #[builder(field = x2 + x1)]
    x3: u32,
}

let example = Example::builder(3).build();

assert_eq!(example.x1, 3);
assert_eq!(example.x2, 6);
assert_eq!(example.x3, 9);
```
