# Positional Members

You can let the caller pass some values as positional parameters to the starting function, that creates the builder or to the finishing function, that consumes it.

## Starting Function

Use `#[builder(start_fn)]` to move some members to the parameters of the starting function.

::: code-group

```rust [Struct]
#[derive(bon::Builder)]
// Top-level attribute to give a custom name for the starting function // [!code highlight]
#[builder(start_fn = with_coordinates)]                                // [!code highlight]
struct Treasure {
    // Member-level attributes to move members   // [!code highlight]
    // to the parameters of `with_coordinates()` // [!code highlight]
    #[builder(start_fn)]                         // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9) // [!code highlight]
    .label("oats".to_owned())
    .build();

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
```

```rust [Function]
// The starting function's name is the name of the underlying function itself,
// that's why we don't really need `#[builder(start_fn = ...)] rename here
// unlike in #[derive(Builder)] syntax case.
#[bon::builder]
fn display_treasure(
    // Member-level attributes to move members   // [!code highlight]
    // to the parameters of `display_treasure()` // [!code highlight]
    #[builder(start_fn)]                         // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
) -> String {
    format!("{x}, {y}, {label:?}")
}

let treasure = display_treasure(2, 9) // [!code highlight]
    .label("oats".to_owned())
    .call();

assert_eq!(treasure, r#"2, 9, Some("oats")"#)
```

```rust [Method]
struct Example;

#[bon::bon]
impl Example {
    // The starting function's name is the name of the underlying function itself,
    // that's why we don't really need `#[builder(start_fn = ...)] rename here
    // unlike in #[derive(Builder)] syntax case.
    #[builder]
    fn display_treasure(
        // Member-level attributes to move members   // [!code highlight]
        // to the parameters of `display_treasure()` // [!code highlight]
        #[builder(start_fn)]                         // [!code highlight]
        x: u32,

        #[builder(start_fn)] // [!code highlight]
        y: u32,

        label: Option<String>,
    ) -> String {
        format!("{x}, {y}, {label:?}")
    }
}

let treasure = Example::display_treasure(2, 9) // [!code highlight]
    .label("oats".to_owned())
    .call();

assert_eq!(treasure, r#"2, 9, Some("oats")"#)
```

:::

::: tip

There are two versions of the `#[builder(start_fn)]` used here: [top-level](../../reference/builder/top-level/start_fn) and [member-level](../../reference/builder/member/start_fn).
They have different meanings.

:::

## Finishing Function

Use `#[builder(finish_fn)]` to move some members to the parameters of the finishing function.

::: code-group

```rust [Struct]
#[derive(bon::Builder)]
// Top-level attribute to give a custom name for the finishing function // [!code highlight]
#[builder(finish_fn = located_at)]                                      // [!code highlight]
struct Treasure {
    // Member-level attributes to move members // [!code highlight]
    // to the parameters of `located_at()`     // [!code highlight]
    #[builder(finish_fn)]                      // [!code highlight]
    x: u32,

    #[builder(finish_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::builder()
    .label("oats".to_owned())
    .located_at(2, 9);  // [!code highlight]

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
```

```rust [Function]
// Top-level attribute to give a custom name for the finishing function // [!code highlight]
#[bon::builder(finish_fn = located_at)]                                 // [!code highlight]
fn treasure(
    // Member-level attributes to move members // [!code highlight]
    // to the parameters of `located_at()`     // [!code highlight]
    #[builder(finish_fn)]                      // [!code highlight]
    x: u32,

    #[builder(finish_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
) -> String {
    format!("{x}, {y}, {label:?}")
}

let treasure = treasure()
    .label("oats".to_owned())
    .located_at(2, 9);  // [!code highlight]

assert_eq!(treasure, r#"2, 9, Some("oats")"#);
```

```rust [Method]
struct Example;

#[bon::bon]
impl Example {
    // Top-level attribute to give a custom name for the finishing function // [!code highlight]
    #[builder(finish_fn = located_at)]                                      // [!code highlight]
    fn treasure(
        // Member-level attributes to move members // [!code highlight]
        // to the parameters of `located_at()`     // [!code highlight]
        #[builder(finish_fn)]                      // [!code highlight]
        x: u32,

        #[builder(finish_fn)] // [!code highlight]
        y: u32,

        label: Option<String>,
    ) -> String {
        format!("{x}, {y}, {label:?}")
    }
}

let treasure = Example::treasure()
    .label("oats".to_owned())
    .located_at(2, 9);  // [!code highlight]

assert_eq!(treasure, r#"2, 9, Some("oats")"#);
```

:::

::: tip

There are two versions of the `#[builder(finish_fn)]` used here: [top-level](../../reference/builder/top-level/finish_fn) and [member-level](../../reference/builder/member/finish_fn).
They have different meanings.

:::
