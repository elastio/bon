# `transparent`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Disables `Option<T>` special handling, makes the member required.

::: tip

This attribute is also configurable via the top-level [`#[builder(on(...))]`](../top-level/on). Currently, its can only be used with the `_` type pattern and as the first `on(...)` clause.

:::


## Examples

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    optional: Option<u32>,

    #[builder(transparent)]
    required: Option<u32>,
}
```
