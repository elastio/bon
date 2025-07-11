---
outline: deep
---

# Optional Members

## `Option<T>`

If your function argument or struct field (or member for short) is of type `Option<T>`, then the generated builder will not enforce setting a value for this member, defaulting to `None`.

::: code-group

```rust [Struct]
#[derive(bon::Builder)]
struct Example {
    level: Option<u32>
}

// We can call it without specifying the `level`
Example::builder().build();
```

```rust [Function]
#[bon::builder]
fn example(level: Option<u32>) {}

// We can call it without specifying the `level`
example().call();
```

```rust [Method]
#[bon::builder]
fn example(level: Option<u32>) {}

// We can call it without specifying the `level`
example().call();
```

:::

You can use [`#[builder(required)]`](../../reference/builder/member/required) to opt-out from this.

### Setters Pair

The builder provides a **pair** of setters for each optional member:

| Name             | Input       | Description                   | Configuration attribute |
| ---------------- | ----------- | ----------------------------- | ----------------------- |
| `{member}`       | `T`         | Accepts a non-`None` value.   | [`some_fn`][setters]    |
| `maybe_{member}` | `Option<T>` | Accepts an `Option` directly. | [`option_fn`][setters]  |

[setters]: ../../reference/builder/member/setters

This is how setters look in the generated code for the example above (simplified):

```rust ignore
impl<S> ExampleBuilder<S> {
    fn level(self, value: u32) -> ExampleBuilder<SetLevel<S>> {
        self.maybe_level(Some(value)) // Yes, it's this simple!
    }

    fn maybe_level(self, value: Option<u32>) -> ExampleBuilder<SetLevel<S>> {
        /* */
    }
}
```

Thanks to this design, changing the member from required to optional [preserves compatibility](./compatibility#making-a-required-member-optional).

### Examples

Pass a non-`None` value via the `{member}(T)` setter:

::: code-group

```rust ignore [Struct]
Example::builder().level(42).build();
```

```rust ignore [Function]
example().level(42).call();
```

```rust ignore [Method]
Example::example().level(42).call();
```

:::

Pass an `Option` value directly via the `maybe_{member}(Option<T>)` setter:

::: code-group

```rust ignore [Struct]
let value = if some_condition {
    Some(42)
} else {
    None
};

Example::builder().maybe_level(value).build();
```

```rust ignore [Function]
let value = if some_condition {
    Some(42)
} else {
    None
};

example().maybe_level(value).call();
```

```rust ignore [Method]
let value = if some_condition {
    Some(42)
} else {
    None
};

Example::example().maybe_level(value).call();
```

:::

## `#[builder(default)]`

To make a member of non-`Option` type optional you may use [`#[builder(default)]`](../../reference/builder/member/default). This attribute uses the [`Default`](https://doc.rust-lang.org/stable/std/default/trait.Default.html) trait or the provided expression to assign the default value for the member.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](./compatibility#switching-between-option-t-and-builder-default).

:::

::: code-group

```rust [Struct]
#[derive(bon::Builder)]
struct Example {
    // This uses the `Default` trait // [!code highlight]
    #[builder(default)]              // [!code highlight]
    a: u32,

    // This uses the given custom default value // [!code highlight]
    #[builder(default = 4)]                     // [!code highlight]
    b: u32,
}

// Here, the default values will be used `a = 0` and `b = 4` // [!code highlight]
let result = Example::builder().build();

assert_eq!(result.a, 0);
assert_eq!(result.b, 4);
```

```rust [Function]
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

```rust [Method]
struct Example;

#[bon::bon]
impl Example {
    #[builder]
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
}

// Here, the default values will be used `a = 0` and `b = 4` // [!code highlight]
let result = Example::example().call();

assert_eq!(result, 4);
```

:::

The same [pair of optional setters](#setters-pair) is generated for members with default values.

::: code-group

```rust ignore [Struct]
let result = Example::builder()
    // Pass a non-None value
    .a(3)
    // Pass an `Option` value directly. `None` means the default
    // value will be used (4 in this case)
    .maybe_b(None)
    .build();
```

```rust ignore [Function]
let result = example()
    // Pass a non-None value
    .a(3)
    // Pass an `Option` value directly. `None` means the default
    // value will be used (4 in this case)
    .maybe_b(None)
    .call();
```

```rust ignore [Method]
let result = Example::example()
    // Pass a non-None value
    .a(3)
    // Pass an `Option` value directly. `None` means the default
    // value will be used (4 in this case)
    .maybe_b(None)
    .call();
```

:::

You can also reference other members in the default expression. See [`#[builder(default)]`](../../reference/builder/member/default#evaluation-context) reference for details.

## Conditional Building

Now that you know how optional members work you can check out the [Conditional Building](../patterns/conditional-building) design patterns or continue studying other features of `bon` by following the "Next page" link at the bottom.
