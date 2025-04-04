# `required`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Disables `Option<T>` special handling, makes the member required.

::: tip

This attribute is also configurable via the top-level [`#[builder(on(...))]`](../top-level/on). Currently, it can only be used with the `_` type pattern and as the first `on(...)` clause.

:::

## Examples

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(required)]
    required: Option<u32>,

    optional: Option<u32>,
}

Example::builder()
    .required(Some(2))
    .optional(2)
    .build();
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(required)]
    required: Option<u32>,

    optional: Option<u32>,
) {}

example()
    .required(Some(2))
    .optional(2)
    .call();
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(required)]
        required: Option<u32>,

        optional: Option<u32>,
    ) {}
}

Example::example()
    .required(Some(2))
    .optional(2)
    .call();
```

:::

Notice the difference:

| Member name | Setters                                          | Comment                      |
| ----------- | ------------------------------------------------ | ---------------------------- |
| `required`  | `required(Option<u32>)`                          | Setter is required to call   |
| `optional`  | `optional(u32)`<br>`maybe_optional(Option<u32>)` | Setters are optional to call |
