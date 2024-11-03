# `name`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="function arguments"/> <Badge type="warning" text="method arguments"/>

Overrides the name of the member used in the builder's API. This is most useful when with struct syntax (`#[derive(Builder)]`) where you'd like to use a different name for the field internally. For functions this attribute makes less sense since it's easy to just create a variable named differently `let new_name = param_name;`. However, this attribute is still supported on function arguments.

## Examples

::: code-group

```rust [Struct]
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

```rust [Function]
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

```rust [Method]
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
