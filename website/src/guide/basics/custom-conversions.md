# Custom Conversions

[`#[builder(into)]`](../../reference/builder/member/into) is great and it works in many cases. However, what if you need to do a conversion that isn't a simple `Into`? What if you want your setter to accept several parameters? What if your setter should be fallible? The answer to all these questions is the bigger brother [`#[builder(with)]`](../../reference/builder/member/with).

You can pass a custom closure to `#[builder(with)]`. It will be used to define the signature of the setter and perform a conversion.

```rust
use bon::Builder;

struct Point {
    x: u32,
    y: u32,
}

#[derive(Builder)]
struct Example {
    #[builder(with = |x: u32, y: u32| Point { x, y })]
    point: Point,
}

let value = Example::builder()
    .point(2, 3)
    .build();

assert_eq!(value.point.x, 2);
assert_eq!(value.point.y, 3);
```

You can also pass a [fallible closure](../../reference/builder/member/with#fallible-setters) and some [well-known functions](../../reference/builder/member/with#well-known-functions) to `#[builder(with)]`.

If your setter needs more complex logic that isn't expressible with `#[builder(with)]` (e.g. mark the setter `unsafe`, set several members at once), then [Custom Methods](./custom-methods) can cover that.
