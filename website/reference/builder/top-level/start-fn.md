# `start_fn`

**Applies to:** <Badge text="structs"/>

Overrides the name and visibility of the associated method that starts the building process, i.e. returns the builder for the struct.

The default name for this method is `builder`, and the default visibility is the same as the visibility of the struct itself.

This attribute can take several forms.

-   Simple: `#[builder(start_fn = identifier)]`. Overrides only the name of the "start" method.
-   Verbose: `#[builder(start_fn(name = identifier, vis = "visibility"))]`.
    Allows overriding both the name and the visibility of the "start" method.
    Each key is optional. The `vis` must be specified as a string literal e.g. `"pub(crate)"`, `"pub"` or `""` (empty string means private visibility).

**Example:**

::: code-group

```rust [Simple form]
use bon::Builder;

#[derive(Builder)]
#[builder(start_fn = init)] // [!code highlight]
struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

```rust [Verbose form]
use bon::Builder;

// `User::init()` method will have `pub(crate)` visibility // [!code highlight]
// Use `vis = ""` to make it fully private instead         // [!code highlight]
#[derive(Builder)]
#[builder(start_fn(name = init, vis = "pub(crate)"))]      // [!code highlight]
pub struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

:::
