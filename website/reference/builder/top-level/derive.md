### `derive`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

*⚠️ Do not confuse this with `derive(bon::Builder)`⚠️*

Generates additional derives on the builder struct itself. The syntax is similar to the regular `#[derive(...)]` attribute, but it must be wrapped in `#[builder(derive(...))]`. Expects one or more of the supported derives separated by commas.


The following derives are supported: `Clone`, `Debug`.

::: warning
The format of the `Debug` output of the builder is not stable and it may change between the patch versions of `bon`.
:::

**Example:**

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(derive(Clone, Debug))] // [!code highlight]
struct Example {
    name: String,
    is_admin: bool,
    level: Option<u32>,
}

let builder = Example::builder().name("Bon".to_owned());

// We can clone the builder    // [!code highlight]
let builder = builder.clone(); // [!code highlight]

// We can debug-format the builder          // [!code highlight]
let builder_debug = format!("{builder:?}"); // [!code highlight]

assert_eq!(
    builder_debug,
    // Only the fields that were set will be output
    r#"ExampleBuilder { name: "Bon" }"#
);

// Finish building
let example = builder.is_admin(true).build();
```

```rust [Free function]
use bon::builder;

#[builder(derive(Clone, Debug))] // [!code highlight]
fn example(
    name: String,
    is_admin: bool,
    level: Option<u32>,
) {}

let builder = example().name("Bon".to_owned());

// We can clone the builder    // [!code highlight]
let builder = builder.clone(); // [!code highlight]

// We can debug-format the builder          // [!code highlight]
let builder_debug = format!("{builder:?}"); // [!code highlight]

assert_eq!(
    builder_debug,
    // Only the fields that were set will be output
    r#"ExampleBuilder { name: "Bon" }"#
);

// Finish building
builder.is_admin(true).call();
```

```rust [Associated method]
use bon::bon;

#[derive(Debug)]
struct Example;

#[bon]
impl Example {
    #[builder(derive(Clone, Debug))] // [!code highlight]
    fn method(
        name: String,
        is_admin: bool,
        level: Option<u32>,
    ) {}

    #[builder(derive(Debug))]
    fn method_with_self(&self) {}
}

let builder = Example::method().name("Bon".to_owned());

// We can clone the builder    // [!code highlight]
let builder = builder.clone(); // [!code highlight]

// We can debug-format the builder          // [!code highlight]
let builder_debug = format!("{builder:?}"); // [!code highlight]

assert_eq!(
    builder_debug,
    // Only the fields that were set will be output
    r#"ExampleMethodBuilder { name: "Bon" }"#
);

// Finish building
builder.is_admin(true).call();

// The debug output of the builder for methods with `self` includes
// the special `self` field with the receiver.
assert_eq!(
    format!("{:?}", Example.method_with_self()),
    "ExampleMethodWithSelfBuilder { self: Example }"
)
```

:::

#### Compile errors

_Requires_ that all members of the builder including the receiver (if this is a builder for an associated method) implement the target trait. For example, this doesn't compile because not all members implement `Clone`:

**Example:**

```rust compile_fail
use bon::Builder;

struct NonClone;

#[derive(Builder)]
#[builder(Clone)]
struct Example {
    // Doesn't derive `Clone`, so this code doesn't compile // [!code error]
    non_clone NonClone,                                     // [!code error]
    cloneable: u32
}
```
