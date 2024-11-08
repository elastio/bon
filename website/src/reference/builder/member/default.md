# `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Makes the member optional and assigns a default value to it. The default value is lazily computed inside of the finishing function based on the form of this attribute.

## Syntax

| Form                               | How default value is computed |
| ---------------------------------- | ----------------------------- |
| `#[builder(default)]`              | `Default::default()`          |
| `#[builder(default = expression)]` | `expression`                  |

If combined with [`#[builder(into)]`](./into), the default expression is additionally converted via [`Into::into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html).

## Setters

Two setter methods are generated for the member with `#[builder(default)]` just like for [members of type `Option<T>`](../../../guide/basics/optional-members#setters-pair):

| Name             | Input       | Description                   | Configuration attribute  |
| ---------------- | ----------- | ----------------------------- | ------------------------ |
| `{member}`       | `T`         | Accepts a non-`None` value.   | [`some_fn`](./setters)   |
| `maybe_{member}` | `Option<T>` | Accepts an `Option` directly. | [`option_fn`](./setters) |

If `None` is passed to the `maybe_{member}` setter, then the default value is used.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](../../../guide/basics/compatibility#switching-between-option-t-and-builder-default).

:::

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(default)] // [!code highlight]
    x1: u32,

    #[builder(default = "anon".to_owned())] // [!code highlight]
    x2: String,

    // No need for `.to_owned()`. Into is applied to the expression
    #[builder(default = "bon", into)] // [!code highlight]
    x3: String,
}

let value = Example::builder().build();

assert_eq!(value.x1, 0);
assert_eq!(value.x2, "anon");
assert_eq!(value.x3, "bon");

let value = Example::builder()
    .x1(99)
    .maybe_x2(None) // None means the default will be used
    .maybe_x3(Some("lyra"))
    .build();

assert_eq!(value.x1, 99);
assert_eq!(value.x2, "anon");
assert_eq!(value.x3, "lyra");
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(default)] // [!code highlight]
    x1: u32,

    #[builder(default = "anon".to_owned())] // [!code highlight]
    x2: String,

    // No need for `.to_owned()`. Into is applied to the expression
    #[builder(default = "bon", into)] // [!code highlight]
    x3: String,
) -> (u32, String, String) {
    (x1, x2, x3)
}

let value = example().call();

assert_eq!(value.0, 0);
assert_eq!(value.1, "anon");
assert_eq!(value.2, "bon");

let value = example()
    .x1(99)
    .maybe_x2(None) // None means the default will be used
    .maybe_x3(Some("lyra"))
    .call();

assert_eq!(value.0, 99);
assert_eq!(value.1, "anon");
assert_eq!(value.2, "lyra");
```

```rust [Method]
use bon::bon;

struct Example {
    x1: u32,
    x2: String,
    x3: String,
}

#[bon]
impl Example {
    #[builder]
    fn new(
        #[builder(default)] // [!code highlight]
        x1: u32,

        #[builder(default = "anon".to_owned())] // [!code highlight]
        x2: String,

        // No need for `.to_owned()`. Into is applied to the expression
        #[builder(default = "bon", into)] // [!code highlight]
        x3: String,
    ) -> Self {
        Self { x1, x2, x3 }
    }
}

let value = Example::builder().build();

assert_eq!(value.x1, 0);
assert_eq!(value.x2, "anon");
assert_eq!(value.x3, "bon");

let value = Example::builder()
    .x1(99)
    .maybe_x2(None) // None means the default will be used
    .maybe_x3(Some("lyra"))
    .build();

assert_eq!(value.x1, 99);
assert_eq!(value.x2, "anon");
assert_eq!(value.x3, "lyra");
```

:::

## Evaluation context

You can use the values of other members by referencing their names in the `default` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `default` expression.

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,

    // Note that here we don't have access to `x3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * x1)]
    x2: u32,

    #[builder(default = x2 + x1)]
    x3: u32,
}

let value = Example::builder()
    .x1(3)
    .build();

assert_eq!(value.x1, 3);
assert_eq!(value.x2, 6);
assert_eq!(value.x3, 9);
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    x1: u32,

    // Note that here we don't have access to `x3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * x1)]
    x2: u32,

    #[builder(default = x2 + x1)]
    x3: u32,
) -> (u32, u32, u32) {
    (x1, x2, x3)
}

let value = example()
    .x1(3)
    .call();

assert_eq!(value, (3, 6, 9));
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        x1: u32,

        // Note that here we don't have access to `x3`
        // because it's declared (and thus initialized) later
        #[builder(default = 2 * x1)]
        x2: u32,

        #[builder(default = x2 + x1)]
        x3: u32,
    ) -> (u32, u32, u32) {
        (x1, x2, x3)
    }
}

let value = Example::example()
    .x1(3)
    .call();

assert_eq!(value, (3, 6, 9));
```

:::

### Caveats

The `self` parameter in associated method syntax is not available to the `default` expression. If you need the `self` context for your defaulting logic, then set your member's type to `Option<T>` and handle the defaulting in the function's body manually.

## Compile errors

This attribute is incompatible with members of `Option` type, since `Option` already implies the default value of `None`. However, it can be used together with [`#[builder(required)]`](./required).
