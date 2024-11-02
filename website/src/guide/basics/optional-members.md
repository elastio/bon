---
outline: deep
---

# Optional Members

If your function argument or struct field (or member for short) is of type `Option<T>`, then the generated builder will not enforce setting a value for this member, defaulting to `None`.

It also generates two setters: one accepts `T` and the other accepts `Option<T>`. The first avoids wrapping values with `Some()` on the call site. The second allows passing the `Option<T>` value directly.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x: Option<u32>,
    y: Option<u32>,

    // Use an annotation for members of non-`Option` type
    #[builder(default)]
    z: u32,
}

// Both `x` and `y` will be set to `None`, `z` will be set to `0`
Example::builder().build();

Example::builder()
    // Pass the value without wrapping it with `Some()`
    .x(10)
    // Or use a `maybe_`-prefixed setter that accepts `Option<T>`
    .maybe_y(Some(20))
    // The APIs generated for `#[builder(default)]` and `Option<T>` are equivalent.
    // `z` will be set to `0` when `build()` is called.
    .maybe_z(None)
    .build();
```

See [optional members](./optional-members) page for details.

## `Option<T>`

Setters generated for members of `Option<T>` type are optional to call. If they aren't invoked, then `None` will be used as the default.

```rust
#[bon::builder]
fn example(level: Option<u32>) {}

// We can call it without specifying the `level`
example().call();
```

You can use [`#[builder(transparent)]`](../reference/builder/member/transparent) to opt-out from this.

### Setters pair

The builder provides a **pair** of setters for each optional member:

| Name             | Input       | Description                   | Configuration attribute
|------------------|-------------|-------------------------------|------------------
| `{member}`       | `T`         | Accepts a non-`None` value.   | [`some_fn`][setters]
| `maybe_{member}` | `Option<T>` | Accepts an `Option` directly. | [`option_fn`][setters]

[setters]: ../reference/builder/member/setters


::: details See how the setters look like in the generated code

```rust ignore
// [GENERATED CODE (simplified)]
impl<S: State> ExampleBuilder<S> {
    fn level(self, value: u32) -> ExampleBuilder<SetLevel<S>> {
        self.maybe_level(Some(value)) // Yes, it's this simple!
    }

    fn maybe_level(self, value: Option<u32>) -> ExampleBuilder<SetLevel<S>> { /* */ }
}
```

:::

Thanks to this design, changing the member from required to optional [preserves compatibility](./compatibility#making-a-required-member-optional).

### Examples

Pass a non-`None` value via the `{member}(T)` setter:

```rust ignore
example().level(42).call();
```

Pass an `Option` value directly via the `maybe_{member}(Option<T>)` setter:

```rust ignore
let value = if some_condition {
    Some(42)
} else {
    None
};

example().maybe_level(value).call();
```

## `#[builder(default)]`

To make a member of non-`Option` type optional you may use the attribute [`#[builder(default)]`](../reference/builder/member/default). This attribute uses the [`Default`](https://doc.rust-lang.org/stable/std/default/trait.Default.html) trait or the provided expression to assign the default value for the member.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](./compatibility#switching-between-option-t-and-builder-default).

:::

```rust
#[bon::builder]
fn example(
    // This uses the `Default` trait // [!code highlight]
    #[builder(default)]              // [!code highlight]
    a: u32,

    // This uses the given custom default value // [!code highlight]
    #[builder(default = 4)]                     // [!code highlight]
    b: u32,
) -> u32 {
    a + b
}

// Here, the default values will be used `a = 0` and `b = 4` // [!code highlight]
let result = example().call();

assert_eq!(result, 4);
```

The same [pair of optional setters](#setters-pair) is generated for members with default values.

```rust ignore
let result = example()
    // Pass a non-None value
    .a(3)
    // Pass an `Option` value directly. `None` means the default
    // value will be used (4 in this case)
    .maybe_b(None)
    .call();
```

## Conditional building

Now that you know how optional members work you can check out the ["Conditional building" design patterns](./conditional-building) or continue studying other features of `bon` by following the "Next page" link at the bottom.