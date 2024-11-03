# `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member optional and assigns a default value to it. The default value is lazily computed inside of the finishing function based on the form of this attribute.

## Syntax

| Form                                     | How default value is computed
| -----------------------------------------|-------------------------------
| `#[builder(default)]`                    | `Default::default()`
| `#[builder(default = expression)]`       | `expression`

If combined with [`#[builder(into)]`](./into), the default expression is additionally converted via [`Into::into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html).

## Setters

Two setter methods are generated for the member with `#[builder(default)]` just like for [members of type `Option<T>`](../../../guide/optional-members#setters-pair):

| Name             | Input       | Description                   | Configuration attribute
|------------------|-------------|-------------------------------|------------------
| `{member}`       | `T`         | Accepts a non-`None` value.   | [`some_fn`](./setters)
| `maybe_{member}` | `Option<T>` | Accepts an `Option` directly. | [`option_fn`](./setters)

If `None` is passed to the `maybe_{member}` setter, then the default value is used.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](../../../guide/compatibility#switching-between-option-t-and-builder-default).

:::


::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(default)] // [!code highlight]
    foo: u32,

    #[builder(default = "anon".to_owned())] // [!code highlight]
    bar: String,

    // No need for `.to_owned()`. Into is applied to the expression
    #[builder(default = "bon", into)] // [!code highlight]
    baz: String,
}

let value = Example::builder().build();

assert_eq!(value.foo, 0);
assert_eq!(value.bar, "anon");
assert_eq!(value.baz, "bon");

let value = Example::builder()
    .foo(99)
    .maybe_bar(None) // None means the default will be used
    .maybe_baz(Some("lyra"))
    .build();

assert_eq!(value.foo, 99);
assert_eq!(value.bar, "anon");
assert_eq!(value.baz, "lyra");
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    #[builder(default)] // [!code highlight]
    foo: u32,

    #[builder(default = "anon".to_owned())] // [!code highlight]
    bar: String,

    // No need for `.to_owned()`. Into is applied to the expression
    #[builder(default = "bon", into)] // [!code highlight]
    baz: String,
) -> (u32, String, String) {
    (foo, bar, baz)
}

let value = example().call();

assert_eq!(value.0, 0);
assert_eq!(value.1, "anon");
assert_eq!(value.2, "bon");

let value = example()
    .foo(99)
    .maybe_bar(None) // None means the default will be used
    .maybe_baz(Some("lyra"))
    .call();

assert_eq!(value.0, 99);
assert_eq!(value.1, "anon");
assert_eq!(value.2, "lyra");
```

```rust [Method]
use bon::bon;

struct Example {
    foo: u32,
    bar: String,
    baz: String,
}

#[bon]
impl Example {
    #[builder]
    fn new(
        #[builder(default)] // [!code highlight]
        foo: u32,

        #[builder(default = "anon".to_owned())] // [!code highlight]
        bar: String,

        // No need for `.to_owned()`. Into is applied to the expression
        #[builder(default = "bon", into)] // [!code highlight]
        baz: String,
    ) -> Self {
        Self { foo, bar, baz }
    }
}

let value = Example::builder().build();

assert_eq!(value.foo, 0);
assert_eq!(value.bar, "anon");
assert_eq!(value.baz, "bon");

let value = Example::builder()
    .foo(99)
    .maybe_bar(None) // None means the default will be used
    .maybe_baz(Some("lyra"))
    .build();

assert_eq!(value.foo, 99);
assert_eq!(value.bar, "anon");
assert_eq!(value.baz, "lyra");
```

:::

## Evaluation context

You can use the values of other members by referencing their names in the `default` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `default` expression.

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    foo: u32,

    // Note that here we don't have access to `baz`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * foo)]
    bar: u32,

    #[builder(default = bar + foo)]
    baz: u32,
}

let value = Example::builder()
    .foo(3)
    .build();

assert_eq!(value.foo, 3);
assert_eq!(value.bar, 6);
assert_eq!(value.baz, 9);
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    foo: u32,

    // Note that here we don't have access to `baz`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * foo)]
    bar: u32,

    #[builder(default = bar + foo)]
    baz: u32,
) -> (u32, u32, u32) {
    (foo, bar, baz)
}

let value = example()
    .foo(3)
    .call();

assert_eq!(value, (3, 6, 9));
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        foo: u32,

        // Note that here we don't have access to `baz`
        // because it's declared (and thus initialized) later
        #[builder(default = 2 * foo)]
        bar: u32,

        #[builder(default = bar + foo)]
        baz: u32,
    ) -> (u32, u32, u32) {
        (foo, bar, baz)
    }
}

let value = Example::example()
    .foo(3)
    .call();

assert_eq!(value, (3, 6, 9));
```

:::

### Caveats

The `self` parameter in associated method syntax is not available to the `default` expression. If you need the `self` context for your defaulting logic, then set your member's type to `Option<T>` and handle the defaulting in the function's body manually.

## Compile errors

This attribute is incompatible with members of `Option` type, since `Option` already implies the default value of `None`. However, it can be used together with [`#[builder(transparent)]`](./transparent).
