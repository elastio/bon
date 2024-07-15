# `#[builder]`

## Attributes applicability

The attributes described below are all expected to be specified inside of the parentheses in the macro invocation like this:

```js
#[builder(parameter_name = some_value_here)]
```

Pay attention to the **"Applies to"** clause of each attribute that describes the contexts where the parameter is allowed. Examples of each context are shown below.

:::details <Badge text="free functions"/> <Badge type="warning" text="free function arguments"/>
```rust
use bon::builder;

// free functions                       // [!code highlight]
#[builder(finish_fn = bar)]             // [!code highlight]
fn foo(
    // free function arguments          // [!code highlight]
    #[builder(into)]                    // [!code highlight]
    arg: u32
) {}
```
:::

:::details <Badge text="structs"/> <Badge type="warning" text="struct fields"/> <Badge text="associated methods"/> <Badge type="warning" text="associated method arguments"/>
```rust
use bon::{bon, builder};

// structs                              // [!code highlight]
#[builder(finish_fn = bar)]             // [!code highlight]
struct Foo {
    // struct fields                    // [!code highlight]
    #[builder(into)]                    // [!code highlight]
    bar: u32,
}

#[bon]
impl Foo {
    // associated methods               // [!code highlight]
    #[builder(finish_fn = bar)]         // [!code highlight]
    fn foo(
        // associated method arguments  // [!code highlight]
        #[builder(into)]                // [!code highlight]
        arg: u32
    ) {}
}
```
:::

## `finish_fn`

**Applies to:** <Badge text="free functions"/> <Badge text="associated methods"/> <Badge text="structs"/>

This attribute allows overriding the name of the generated builder's method that finishes the building process.

**Example:**

```rust
struct ArticlesClient;

#[bon]
impl ArticlesClient {
    #[builder(finish_fn = send)] // [!code highlight]
    fn get_article(&self, id: u32) -> String {
        format!("Some article with id {id}")
    }
}

let response = ArticlesClient
    .get_article()
    .id(42)
    .send(); // [!code highlight]

assert_eq!(response, "Some article with id 42");
```


## `into`

**Applies to:** <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/> <Badge type="warning" text="struct fields"/>

This attribute forces an `impl Into` conversion to be applied in the generated setter method. Use this to force-override the decision made by [automatic `Into` conversion qualification rules](../guide/into-conversions#types-that-qualify-for-an-automatic-into-conversion).

**Example:**

```rust
use bon::builder;
use std::num::NonZeroU32;

#[builder]
struct Counter {
    #[builder(into)] // [!code highlight]
    value: u32
}

let non_zero_u32 = NonZeroU32::new(1).unwrap();

Counter::builder()
    // value accepts an `impl Into<u32>` here // [!code highlight]
    .value(non_zero_u32)                      // [!code highlight]
    .build();
```
