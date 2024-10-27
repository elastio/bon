# `name`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Overrides the name of the member in the builder's setters and type state. This is most useful when with struct syntax (`#[derive(Builder)]`) where you'd like to use a different name for the field internally. For functions this attribute makes less sense since it's easy to just create a variable named differently `let new_name = param_name;`. However, this attribute is still supported on function arguments.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Player {
    #[builder(name = rank)] // [!code highlight]
    level: u32
}

Player::builder()
    .rank(10) // [!code highlight]
    .build();
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn player(
    #[builder(name = rank)] // [!code highlight]
    level: u32
) {}

player()
    .rank(10) // [!code highlight]
    .call();
```

```rust [Associated method argument]
use bon::bon;

struct Player {
    level: u32,
}

#[bon]
impl Player {
    #[builder]
    fn new(
        #[builder(name = rank)] // [!code highlight]
        level: u32
    ) -> Self {
        Self { level }
    }
}

Player::builder()
    .rank(10) // [!code highlight]
    .build();
```

:::
