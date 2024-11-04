# `derive`

**Applies to:** <Badge text="structs"/> <Badge text="functions"/> <Badge text="methods"/>

_⚠️ Do not confuse this with `#[derive(bon::Builder)]`⚠️_

Generates additional derives for the builder struct itself. The syntax is similar to the regular `#[derive(...)]` attribute, but it must be wrapped in `#[builder(derive(...))]`. Expects one or more of the supported derives separated by a comma.

The following derives are supported: `Clone`, `Debug`.

::: warning
The format of the `Debug` output of the builder is not stable, and it may change between patch versions of `bon`.
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

```rust [Function]
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

```rust [Method]
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

## Generic types handling

If the underlying `struct` or `fn` contains generic type parameters, then the generated impl block will include a `where` bound requiring the respective trait (`Clone` or `Debug`) to be implemented by all of them. This follows the behaviour of the [standard `derive` macros](https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable).

This works fine in most cases, but sometimes the generated bounds may be overly restrictive. To fix that, you can manually specify the bounds using the syntax `#[builder(derive(Trait(bounds(...))))]`, where `...` is a comma-separated list of `where` bounds.

See the example of this problem, and how it can be fixed (click on the tab `Fixed` in the code snippet):

::: code-group

```rust compile_fail [Overly restrictive]
use bon::Builder;
use std::rc::Rc;

#[derive(Builder)]
#[builder(derive(Clone))]
struct Example<T, U> {
    x: Rc<T>,
    y: U,
}

struct NonCloneable;

let builder = Example::<_, ()>::builder().x(Rc::new(NonCloneable));

// `Rc` can be cloned even if `T` is not `Clone`, but this code   // [!code error]
// doesn't compile, because the `Clone` impl for `ExampleBuilder` // [!code error]
// conservatively requires `T: Clone`                             // [!code error]
builder.clone(); // [!code error]
```

```rust [Fixed]
use bon::Builder;
use std::rc::Rc;

#[derive(Builder)]
// Only a bound `U: Clone` is needed in this case // [!code highlight]
#[builder(derive(Clone(bounds(U: Clone))))] // [!code highlight]
struct Example<T, U> {
    x: Rc<T>,
    y: U,
}

struct NonCloneable;

let builder = Example::<_, ()>::builder().x(Rc::new(NonCloneable));

// Now this works, because there is no bound `T: Clone` // [!code highlight]
builder.clone();
```

:::

::: tip
If you'd like to know why this attribute is this dumb and doesn't just add a `where Rc<T>: Clone` bound instead, then check this article about the ["Implied bounds and perfect derive"](https://smallcultfollowing.com/babysteps/blog/2022/04/12/implied-bounds-and-perfect-derive/) by Niko Matsakis 📖.

:::

## Compile errors

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
