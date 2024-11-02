# `overwritable` :microscope:

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Allows calling setters for the same member repeatedly.

::: danger 🔬 **Experimental**

This attribute is available under the cargo feature `experimental-overwritable`. Breaking changes may occur between **minor** releases but not between patch releases.

The fate of this feature depends on your feedback in the tracking issue [#149](https://github.com/elastio/bon/issues/149). Please, let us know if you have a use case for this attribute!

:::

::: tip

This attribute is also configurable via the top-level [`#[builder(on(...))]`](../top-level/on)

:::

For example, this code wouldn't compile without this attribute:

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

Overwrites like in the example above are generally considered bugs. However, there are several cases when this attribute may be useful.

## Improving compile times

This attribute simplifies the generated code. For example, it completely removes type states for [optional members](../../../guide/optional-members).

If you'd like to improve your compile times, consider enabling overwrites with `#[builder(on(_, overwritable))]` and checking how much it helps. The difference is visible on a larger scale, especially for structs/functions with tens of optional members.

It is a trade-off between the level of compile-time checks and compilation performance, so choose wisely ⚖️!

## Dummy values in tests

You might want to use this to construct dummy values for tests (fixtures).

**Example:**

::: tip

The builder's type signature mentioned in this example is described in the ["Builder Extensions"](../../../guide/builder-extensions) guide.

:::

```rust
#[derive(bon::Builder)]
#[builder(on(_, overwritable))]
struct User {
    login: String,
    name: String,
    level: u32,
}

// See details about this module in the "Builder Extensions" guide
use user_builder::{SetLevel, SetLogin, SetName};

// Returns a base builder with dummy values for all fields
fn user() -> UserBuilder<SetLogin<SetName<SetLevel>>> {
    User::builder()
        .level(24)
        .name("Bon Bon".to_owned())
        .login("@bonbon".to_owned())
}

// Build a user with an empty name, and all other fields filled with dummy data
user()
    .name("".to_owned())
    .build();

// Builder an admin user with all other irrelevant fields filled with dummy data
user()
    .level(0)
    .login("@admin".to_owned())
    .build();
```
