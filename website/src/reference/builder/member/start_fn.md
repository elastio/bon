# `start_fn`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member a positional argument on the starting function that creates the builder.

::: tip

Don't confuse this with the [top-level](../top-level/start_fn) `#[builder(start_fn)]` attribute.

:::

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(start_fn)] // [!code highlight]
    foo: u32,

    #[builder(start_fn)] // [!code highlight]
    bar: u32,

    baz: u32,
}

let value = Example::builder(1, 2) // [!code highlight]
    .baz(3)
    .build();

assert_eq!(value.foo, 1);
assert_eq!(value.bar, 2);
assert_eq!(value.baz, 3);
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(start_fn)] // [!code highlight]
    foo: u32,

    #[builder(start_fn)] // [!code highlight]
    bar: u32,

    baz: u32,
) -> (u32, u32, u32) {
    (foo, bar, baz)
}

let value = example(1, 2) // [!code highlight]
    .baz(3)
    .call();

assert_eq!(value.0, 1);
assert_eq!(value.1, 2);
assert_eq!(value.2, 3);
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(start_fn)] // [!code highlight]
        foo: u32,

        #[builder(start_fn)] // [!code highlight]
        bar: u32,

        baz: u32,
    ) -> (u32, u32, u32) {
        (foo, bar, baz)
    }
}

let value = Example::example(1, 2) // [!code highlight]
    .baz(3)
    .call();

assert_eq!(value.0, 1);
assert_eq!(value.1, 2);
assert_eq!(value.2, 3);
```

:::

You can rename the starting function from default `builder()` for structs to something more readable via the top-level [`#[builder(start_fn = ...)]`](../top-level/start-fn) attribute.

## Ordering

The ordering of members annotated with `#[builder(start_fn)]` matters! They will appear in the same order relative to each other in the starting function signature. They must also be declared at the top of the members' list.

This ensures a consistent initialization order, and it makes these members available for expressions in `#[builder(default/skip = ...)]` for regular members that follow them.

## `Into` Conversions

You can combine this attribute with [`#[builder(into)]`](./into) or [`#[builder(on(..., into))]`](../top-level/on) to add an `Into` conversion for the parameter.

Importantly, `Into` conversions for such members work slightly differently from the regular (named) members in regard to the `Option<T>` type. There is no special handling of `Option<T>` type for the members annotated with `#[builder(start_fn)]`. Thus, they are matched by the type pattern of `on(..., into)` as any other type.

::: tip

In general, it's not recommended to annotate members of `Option<T>` type with `#[builder(start_fn)]` because you can't omit setting them using the positional function call syntax.

:::
