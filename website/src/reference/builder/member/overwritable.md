# `overwritable` 🔬

::: danger 🔬 **Experimental**

This attribute available under the cargo feature `"experimental-overwritable"`. There may be breaking changes to this attribute between **minor** releases, but not between patch releases.

The fate of this feature depends on your feedback in the tracking issue [#149](https://github.com/elastio/bon/issues/149). Please, let us know if you have a use case for this attribute!

:::

::: tip

This attribute is also configurable via the top-level [`#[builder(on(...))]`](../top-level/on)

:::

Makes it possible to call setters for the same member repeatedly.

For example this code wouldn't compile without this attribute:

```rust
#[derive(bon::Builder)]
struct Example {
    #[builder(overwritable)] // [!code highlight]
    x: u32,
}

Example::builder()
    .x(1)
    // Setting the value for `x` second time is allowed with `overwritable` // [!code highlight]
    .x(2) // [!code highlight]
    .build();
```

Overwrites like on the example above are generally considered to be bugs. However, there are several cases where this attribute may be useful.

## Dummy values in tests

You might want to use this to construct dummy values for tests (fixtures).

**Example:**

::: tip

The type signature of the builder is described in the "Typestate API" guide.

:::

```rust
#[derive(bon::Builder)]
#[builder(on(_, overwritable))]
struct User {
    login: String,
    name: String,
    level: u32,
}

// See details about this module in the "Typestate API" guide
use user_builder::{SetLevel, SetLogin, SetName};

// Returns a base builder with dummy values for all fields
fn user() -> UserBuilder<SetLevel<SetName<SetLogin>>> {
    User::builder()
        .level(24)
        .name("Bon Bon".to_owned())
        .login("@bonbon".to_owned())
}

// Build user with an empty name, and all other fields filled with dummy data
let user = user()
    .name("".to_owned())
    .build();

let admin = user()
    .level(0)
    .login("@admin".to_owned())
    .build();
```
