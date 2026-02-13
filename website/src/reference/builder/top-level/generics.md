# `generics` :microscope:

**Applies to:** <Badge text="structs"/> <Badge text="functions"/> <Badge text="methods"/>

::: danger 🔬 **Experimental**

This attribute is available under the cargo feature `experimental-generics-setters`. Breaking changes may occur between **minor** releases but not between patch releases.

:::

Generates methods to overwrite generic type parameters. This feature is useful for type-level state machines: starting with marker types and changing them as you progress through states. See examples below for inspiration.

## How It Works

For each generic type parameter (not lifetimes or const generics), a conversion method is generated that:

1. Takes the current builder and returns a new builder with the type parameter changed
2. Preserves all fields that don't use the converted generic parameter

This allows you to start with placeholder types (like `()`) and convert them to concrete types as you build.

## Basic Example

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(generics(setters = "with_{}"))] // [!code highlight]
struct Example<TData> {
    data: TData,
    count: u32,
}

// Start with unit types, then convert to concrete types
let example = Example::<()>::builder()
    .count(42)
    // Change TData from () to i32. Only possible if none  // [!code highlight]
    // of the members that use `TData` generic are set     // [!code highlight]
    .with_t_data::<i32>()
    .data(100)
    .build();

assert_eq!(example.data, 100_i32);
assert_eq!(example.count, 42_u32);
```

```rust [Function]
use bon::builder;

#[builder(generics(setters = "with_{}"))] // [!code highlight]
fn example<TData>(
    data: TData,
    count: u32
) -> (TData, u32) {
    (data, count)
}

// Start with unit types, then convert to concrete types
let example = example::<()>()
    .count(42)
    // Change TData from () to i32. Only possible if none  // [!code highlight]
    // of the members that use `TData` generic are set     // [!code highlight]
    .with_t_data::<i32>()
    .data(100)
    .call();

assert_eq!(example.0, 100_i32);
assert_eq!(example.1, 42_u32);
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(generics(setters = "with_{}"))] // [!code highlight]
    fn example<TData>(
        data: TData,
        count: u32
    ) -> (TData, u32) {
        (data, count)
    }
}

// Start with unit types, then convert to concrete types
let example = Example::example::<()>()
    .count(42)
    // Change TData from () to i32. Only possible if none  // [!code highlight]
    // of the members that use `TData` generic are set     // [!code highlight]
    .with_t_data::<i32>()
    .data(100)
    .call();

assert_eq!(example.0, 100_i32);
assert_eq!(example.1, 42_u32);
```

:::

## No-Turbofish Optional Generic Members

This feature can be used to provide default values for generic types that can be overwritten with a setter.
This is one of the solutions for the [optional generic members turbofish problem](../../../guide/patterns/optional-generic-members).
It's quite verbose at definition site, so it should be used as a last resort. You may also wrap this pattern with your own macro if you are going to use it alot.

::: tip NOTE

This example defines [custom methods](../../../guide/typestate-api/custom-methods). Consult the referenced guide page for details if needed.

:::

::: code-group

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(
    generics(setters = "with_{}"), // [!code highlight]
    // Privatize the starting function, we'll define our own
    start_fn(vis = "", name = builder_internal),
)]
struct Data<T> {
    // Privatize the setters, we'll define our own
    #[builder(setters(name = value_internal, vis = ""))]
    value: Option<T>,
}

// Custom `builder()` method that begins with `T = ()`
impl Data<()> {
    pub fn builder() -> DataBuilder<()> {
        Data::builder_internal()
    }
}

use data_builder::{State, SetValue, IsUnset};

// Custom setter, that overwrites `T` with the type of the provided value.
impl<T, S: State> DataBuilder<T, S> {
    fn value<NewT>(self, value: NewT) -> DataBuilder<NewT, SetValue<S>>
    where
        S::Value: IsUnset,
    {
        self.with_t().value_internal(value) // [!code highlight]
    }
}

// By default, the generic parameter is `()`. No type hints are required.
let data_unit = Data::builder().build();
assert_eq!(data_unit.value, None);

let data_u32 = Data::builder()
    .value(42)
    .build();

assert_eq!(data_u32.value, Some(42));
```

:::

## Config

### `setters`

Configures the generic parameter overwrite methods.

**Short syntax** configures just the name pattern:

```attr
#[builder(generics(setters = "with_{}"))]
```

**Long syntax** provides more flexibility. The `name` parameter is required, while others are optional:

```attr
#[builder(
    generics(setters(
        name = "with_{}",
        vis = "pub(crate)",
        doc {
            /// Custom documentation
        }
    ))
)]
```

#### `name`

**Required.** A pattern string where `{}` will be replaced with the `snake_case` name of each generic parameter. For example, with generic types `<TData, TError>` and pattern `"with_{}"`, methods `with_t_data()` and `with_t_error()` will be generated.

#### `vis`

The visibility for the generated conversion methods. Must be enclosed with quotes. Use `""` or [`"pub(self)"`](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself) for private visibility.

The default visibility matches the visibility of the [`builder_type`](./builder_type#vis).

#### `doc`

Custom documentation for the generated conversion methods. The syntax expects a block with doc comments:

```attr
doc {
    /// Custom documentation
}
```

Simple documentation is generated by default explaining what the method does.

## See Also

- [Typestate API](../../../guide/typestate-api) - Understanding the builder's type system
- [`builder_type`](./builder_type) - Configuring the builder type itself
