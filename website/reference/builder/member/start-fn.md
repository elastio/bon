# `start_fn`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member a positional argument on the starting function that creates the builder.

The ordering of members annotated with `#[builder(start_fn)]` matters! They will appear in the same order relative to each other in the starting function signature. They must also be declared at the top of the members' list.

This ensures a consistent initialization order, and it makes these members available for expressions in `#[builder(default/skip = ...)]` for regular members that follow them.

::: tip

Don't confuse this with the top-level [`#[builder(start_fn = ...)]`](#start-fn) attribute that can be used to configure the name and visibility of the starting function. You'll likely want to use it in combination with this member-level attribute to define a better name for the starting function.

:::

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
// Top-level attribute to give a better name for the starting function // [!code highlight]
#[builder(start_fn = with_coordinates)]                                // [!code highlight]
struct Treasure {
    // Member-level attribute to mark the member as // [!code highlight]
    // a parameter of `with_coordinates()`          // [!code highlight]
    #[builder(start_fn)]                            // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9) // [!code highlight]
    .label("knowledge".to_owned())
    .build();

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("knowledge"));
```

```rust [Free function]
use bon::builder;

#[builder]
fn mark_treasure_at(
    #[builder(start_fn)] // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
) {}

mark_treasure_at(2, 9)
    .label("knowledge".to_owned())
    .call();
```

```rust [Associated method]
use bon::bon;

struct Navigator {}

#[bon]
impl Navigator {
    #[builder]
    fn mark_treasure_at(
        &self,

        #[builder(start_fn)] // [!code highlight]
        x: u32,

        #[builder(start_fn)] // [!code highlight]
        y: u32,

        label: String,
    ) {}
}

let navigator = Navigator {};

navigator
    .mark_treasure_at(2, 9)
    .label("knowledge".to_owned())
    .call();
```

:::

You can also combine this attribute with [`#[builder(into)]`](./into) or [`#[builder(on(..., into))]`](../top-level/on) to add an into conversion for the parameter.

Importantly, `Into` conversions for such members work slightly differently from the regular (named) members in regard to the `Option` type. The `Option` type gives no additional meaning to the member annotated with `#[builder(start_fn)]`. Thus, it is matched by the type pattern of `on(..., into)` and wrapped with `impl Into<Option<T>>` as any other type.

::: tip

In general, it's not recommended to annotate optional members with `#[builder(start_fn)]` because you can't omit setting them using the positional function call syntax.

:::
