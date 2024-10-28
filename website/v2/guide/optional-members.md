# Optional Members

## `Option<T>`

Setters generated for members of `Option<T>` type are optional to call. If they aren't invoked, then `None` will be used as the default.

**Example:**

```rust
use bon::builder;

#[builder]
fn example(level: Option<u32>) {}

// We can call it without specifying the `level`
example().call();
```

The generated builder has two setters for each optional member. One setter accepts `T` and the other accepts `Option<T>`. The following setters will be generated in the example above (simplified):

```rust ignore
impl ExampleBuilder {
    // Accepts the underlying value. Wraps it in `Some()` internally
    fn level(value: u32) -> NextBuilderState { /* */ }

    // Accepts the `Option` directly.
    fn maybe_level(value: Option<u32>) -> NextBuilderState { /* */ }
}
```

::: tip

Thanks to this design, changing the member from required to optional [preserves compatibility](./compatibility#making-a-required-member-optional).

:::

---

If you need to pass a simple literal value, then the syntax is very short

```rust ignore
example().level(42).call();
```

If you already have an `Option` variable somewhere or you need to dynamically decide if the value should be `Some` or `None`, then you can use the `maybe_` variant of the setter.

```rust ignore
let value = if some_condition {
    Some(42)
} else {
    None
};

example().maybe_level(value).call();
```

## `#[builder(default)]`

To make a member of non-`Option` type optional you may use the attribute [`#[builder(default)]`](../reference/builder#default). This attribute uses the [`Default`](https://doc.rust-lang.org/stable/std/default/trait.Default.html) trait or the provided expression to assign the default value for the member.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](./compatibility#switching-between-option-t-and-builder-default).

:::

**Example:**

```rust
use bon::builder;

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

// Here, the default values will be used `a = 0` and `b = 4` // [!code highlight]
let result = example().call();
assert_eq!(result, 4);

// The same couple of setters `{member}(T)` and `maybe_{member}(Option<T>)` // [!code highlight]
// are generated just like it works with members of `Option<T>` type        // [!code highlight]
let result = example()
    .a(3)
    .b(5)
    .call();
assert_eq!(result, 8);

let result = example()
    .maybe_a(Some(3))
    .maybe_b(Some(5))
    .call();
assert_eq!(result, 8);
```

## Conditional building

Now that you know how optional members work you can check out the ["Conditional building" patterns](./patterns/conditional-building) or continue studying other features of `bon` by following the "Next page" link at the bottom.
