# `skip`

**Applies to:** <Badge type="warning" text="struct fields"/>

Skips generating setters for the member. This hides the member from the generated builder API, so the caller can't set its value.

The value for the member will be computed inside of the finishing function.

| Form                            | How value for the member is computed |
| ------------------------------- | ------------------------------------ |
| `#[builder(skip)]`              | `Default::default()`                 |
| `#[builder(skip = expression)]` | `expression`                         |

## Example

```rust
use bon::Builder;

#[derive(Builder)]
struct User {
    #[builder(skip)] // [!code highlight]
    level: u32,

    // Any complex expression is accepted // [!code highlight]
    #[builder(skip = "anon".to_owned())]  // [!code highlight]
    name: String,
}

let user = User::builder()
    // There are no `level`, and `name` setters generated // [!code highlight]
    .build();

assert_eq!(user.level, 0);
assert_eq!(user.name, "anon");
```

## Evaluation context

You can reference other members as variables in the `skip` expression. All members are initialized in the order of their declaration, and thus only members that are declared earlier (higher) in the code are available for the `skip` expression.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,

    // Note that here we don't have access to `x3`
    // because it's declared (and thus initialized) later
    #[builder(skip = 2 * x1)]
    x2: u32,

    #[builder(skip = x2 + x1)]
    x3: u32,
}

let example = Example::builder()
    .x1(3)
    .build();

assert_eq!(example.x1, 3);
assert_eq!(example.x2, 6);
assert_eq!(example.x3, 9);
```

## Unsupported function syntax

This attribute is not supported with function or method syntax because it's simply unnecessary there and can easier be expressed with local variables.
