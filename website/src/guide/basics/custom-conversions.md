# Custom Conversions

[`#[builder(into)]`](../../reference/builder/member/into) is great and it works in many cases. However, what if you need to do a conversion that isn't a simple `Into`? What if you want your setter to accept several parameters? What if your setter should be fallible? The answer to all these questions is the bigger brother [`#[builder(with)]`](../../reference/builder/member/with).

You can pass a custom closure to `#[builder(with)]`. It will define the signature of the setter and perform a conversion.

::: code-group

```rust [Struct]
struct Point {
    x: u32,
    y: u32,
}

#[derive(bon::Builder)]
struct Example {
    #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
    point: Point,
}

let value = Example::builder()
    .point(2, 3) // [!code highlight]
    .build();

assert_eq!(value.point.x, 2);
assert_eq!(value.point.y, 3);
```

```rust [Function]
struct Point {
    x: u32,
    y: u32,
}

#[bon::builder]
fn example(
    #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
    point: Point,
) -> Point {
    point
}

let value = example()
    .point(2, 3) // [!code highlight]
    .call();

assert_eq!(value.x, 2);
assert_eq!(value.y, 3);
```

```rust [Method]
struct Point {
    x: u32,
    y: u32,
}

struct Example;

#[bon::bon]
impl Example {
    #[builder]
    fn example(
        #[builder(with = |x: u32, y: u32| Point { x, y })] // [!code highlight]
        point: Point,
    ) -> Point {
        point
    }
}

let value = Example::example()
    .point(2, 3) // [!code highlight]
    .call();

assert_eq!(value.x, 2);
assert_eq!(value.y, 3);
```

:::

You can make the setter fallible by passing a [fallible closure](../../reference/builder/member/with#fallible-closure).

You can pass one of [well-known functions](../../reference/builder/member/with#well-known-functions) instead of a closure to `#[builder(with)]`. If any of them fit your use, this will save you some characters to type.

If your setter needs more complex logic that isn't expressible with `#[builder(with)]` (e.g. mark the setter `unsafe`, set several members at once), then [Custom Methods](../typestate-api/custom-methods) can cover that.
