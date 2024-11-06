# Positional Members

You can let the caller pass some values as positional parameters to the starting function, that creates the builder or to the finishing function, that consumes it.

## Starting function

Use `#[builder(start_fn)]` to move some members to the parameters of the starting function.

```rust
use bon::Builder;

#[derive(Builder)]
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

::: tip

There are two versions of the `#[builder(start_fn)]` used here: [top-level](../../reference/builder/top-level/start_fn) and [member-level](../../reference/builder/member/start_fn).
They have different meanings.

:::

## Finishing function

Use `#[builder(finish_fn)]` to move some members to the parameters of the finishing function.

```rust
use bon::Builder;

#[derive(Builder)]
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

::: tip

There are two versions of the `#[builder(finish_fn)]` used here: [top-level](../../reference/builder/top-level/finish_fn) and [member-level](../../reference/builder/member/finish_fn).
They have different meanings.

:::
