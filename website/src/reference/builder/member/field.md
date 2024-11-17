# `field`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Defines a private field on the builder without setters. This field will be available in [Custom Methods](../../../guide/typestate-api/custom-methods).

The value for the member will be computed inside of the starting function.

| Form                             | How value for the member is computed |
| -------------------------------- | ------------------------------------ |
| `#[builder(field)]`              | `Default::default()`                 |
| `#[builder(field = expression)]` | `expression`                         |

## Difference with [`#[builder(skip)]`](./skip)

`#[builder(field)]` attribute is similar to [`#[builder(skip)]`](./skip), but the main difference is that `#[builder(field)]` computes the value in the starting function and stores it in a private field in the builder. This lets you access that field to manage additional custom state during the building process.

On the other hand, [`#[builder(skip)]`](./skip) computes the value in the finishing function and doesn't store it in the builder.

## Example

Let's define a `Vec` field with a custom method to push values into it during building.

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(field)] // [!code highlight]
    coefs: Vec<u32>,
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_builder::State> ExampleBuilder<S> {
    fn coef(mut self, value: u32) -> Self {
        self.coefs.push(value);
        self
    }
}

let example = Example::builder()
    .coef(2)
    .coef(99)
    .build();

assert_eq!(example.coefs, [2, 99]);
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(field)] // [!code highlight]
    coefs: Vec<u32>,
) -> Vec<u32> {
    coefs
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_builder::State> ExampleBuilder<S> {
    fn coef(mut self, value: u32) -> Self {
        self.coefs.push(value);
        self
    }
}

let example = example()
    .coef(2)
    .coef(99)
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
        coefs: Vec<u32>,
    ) -> Vec<u32> {
        coefs
    }
}

// Add a custom method that uses the field // [!code highlight]
impl<S: example_method_builder::State> ExampleMethodBuilder<S> {
    fn coef(mut self, value: u32) -> Self {
        self.coefs.push(value);
        self
    }
}

let example = Example::method()
    .coef(2)
    .coef(99)
    .call();

assert_eq!(example, [2, 99]);
```

:::

## Evaluation context

You can use values of other member marked with [`#[builder(start_fn)]`](./start_fn) or [`#[builder(field)]`](./field) by referencing their names in the `field` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `field` expression.

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
