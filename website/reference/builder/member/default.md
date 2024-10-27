# `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member optional and assigns a default value to it. There will be two setter methods generated for the member just like for [members of type `Option<T>`](../guide/optional-members). One setter accepts a value of type `T` (type of the member) and the other (with the `maybe_` prefix) accepts an `Option<T>`.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](../guide/compatibility#switching-between-option-t-and-builder-default).

:::

The default value will be lazily computed inside of the [finishing function](#finish-fn) (i.e. `build()` or `call()`). It is computed only if the setter for the member wasn't called or `None` was passed to the `maybe_{member}()` setter.

The default value is computed based on the form of this attribute:

| Form                               | How default value is computed |
| ---------------------------------- | ----------------------------- |
| `#[builder(default)]`              | `Default::default()`          |
| `#[builder(default = expression)]` | `expression`                  |

The result of the `expression` will be converted into the target type using [`Into::into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) if [`#[builder(into)]`](#into) is enabled for the setter.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct User {
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically             // [!code highlight]
    // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
    #[builder(into, default = "anon")]                                    // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
}

let user = User::builder().build();

assert_eq!(user.level, 0);
assert_eq!(user.name, "anon");
assert_eq!(user.permissions, ["read"]);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn greet_user(
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically             // [!code highlight]
    // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
    #[builder(into, default = "anon")]                                    // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
) -> String {
    format!("Hello {name}! Your level is {level}, permissions: {permissions:?}")
}

let greeting = greet_user().call();

assert_eq!(greeting, "Hello anon! Your level is 0, permissions: [\"read\"]");
```

```rust [Associated method argument]
use bon::bon;

struct User {
    level: u32,
    name: String,
    permissions: Vec<String>,
}

#[bon]
impl User {
    #[builder]
    fn new(
        #[builder(default)] // [!code highlight]
        level: u32,

        // The expression of type `&'static str` is automatically             // [!code highlight]
        // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
        #[builder(into, default = "anon")]                                    // [!code highlight]
        name: String,

        // Any complex expression is accepted   // [!code highlight]
        #[builder(default = bon::vec!["read"])] // [!code highlight]
        permissions: Vec<String>,
    ) -> Self {
        Self { level, name, permissions }
    }
}

let user = User::builder().build();

assert_eq!(user.name, "anon");
assert_eq!(user.level, 0);
assert_eq!(user.permissions, ["read"]);
```

:::

You can also use the values of other members by referencing their names in the `default` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `default` expression.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Example {
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * member_1)]
    member_2: u32,

    #[builder(default = member_2 + member_1)]
    member_3: u32,
}

let example = Example::builder()
    .member_1(3)
    .build();

assert_eq!(example.member_1, 3);
assert_eq!(example.member_2, 6);
assert_eq!(example.member_3, 9);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn example(
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * member_1)]
    member_2: u32,

    #[builder(default = member_2 + member_1)]
    member_3: u32,
) -> (u32, u32, u32) {
    (member_1, member_2, member_3)
}

let example = example()
    .member_1(3)
    .call();

assert_eq!(example, (3, 6, 9));
```

```rust [Associated method argument]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        member_1: u32,

        // Note that here we don't have access to `member_3`
        // because it's declared (and thus initialized) later
        #[builder(default = 2 * member_1)]
        member_2: u32,

        #[builder(default = member_2 + member_1)]
        member_3: u32,
    ) -> (u32, u32, u32) {
        (member_1, member_2, member_3)
    }
}

let example = Example::example()
    .member_1(3)
    .call();

assert_eq!(example, (3, 6, 9));
```

:::

## Caveats

The `self` parameter in associated methods is not available to the `default` expression. If you need the `self` context for your defaulting logic, then set your member's type to `Option<T>` and handle the defaulting in the function's body manually.

## Compile errors

This attribute is incompatible with members of `Option` type, since `Option` already implies the default value of `None`.
