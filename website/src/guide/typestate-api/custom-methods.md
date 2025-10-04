# Custom Methods

On this page, you'll learn how to add custom methods to the builder type 💪.

This page assumes you've read the previous [Builder's Type Signature](./builders-type-signature) page. If you haven't already, please do.

## `State` and `IsUnset` Traits

When a builder transitions from one type state to another, the compiler must ensure the transition is valid. For example, once a setter is called, calling it again must not be possible.

The generated builder's state module contains a trait called `IsUnset`, which is used to restrict the possible typestates when calling a setter.

There is also a trait called `State`, that stores the type states for every member of the builder in its associated types.

Enough talking, let's see how it works in practice.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,
}
```

This code generates the following setters:

```rust ignore
// Import traits and type states from the generated module
use example_builder::{State, IsUnset, SetX1};

impl<S: State> ExampleBuilder<S> {
    fn x1(self, value: u32) -> ExampleBuilder<SetX1<S>>
    where              // [!code highlight]
        S::X1: IsUnset // [!code highlight]
    { /**/ }
}
```

The main thing here is `where S::X1: IsUnset`. This `where` bound allows only the typestates that don't contain `SetX1<...>` anywhere in the layers of state transitions. In this case, only `Empty` typestate implements `S::X1: IsUnset`.

If we were to have other members, their `Set{OtherMember}<S>` transitions would also implement `S::X1: IsUnset` unless the underlying `S` contains a `SetX1` somewhere in it.

## Custom Setters

Now that you know the `State` and `IsUnset` traits you can write a custom `impl` block for the builder that defines custom setters.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    // Make the default generated setter private, and rename
    // it so it doesn't collide with our custom method name
    #[builder(setters(vis = "", name = x1_internal))]
    x1: u32
}

use example_builder::{IsUnset, State, SetX1};

impl<S: State> ExampleBuilder<S> {
    fn x1(self, value: u32) -> ExampleBuilder<SetX1<S>>
    where
        S::X1: IsUnset,
    {
        self.x1_internal(value * 2)
    }
}

let value = Example::builder()
    .x1(3)
    .build();

assert_eq!(value.x1, 6);
```

This is a simple example of how you can write a custom setter.

You could achieve the same with a [Custom Conversion](../basics/custom-conversions) via `#[builder(with)]`. However, the beauty of this design is that the `impl` block has no magic macros involved. It's rather simple and easy to read.

Also, with this approach, you can make the setter `unsafe`, `async` or even let it set several members.

### Set Several Members In One Setter

I recommend you try doing this yourself to test your intuition of the typestate API.

The goal is to make the following work:

```rust compile_fail
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,
    x2: u32,
}

let value = Example::builder()
    .x1_and_x2(1, 2)
    .build();

assert_eq!(value.x1, 1);
assert_eq!(value.x2, 2);
```

::: details Expand to see the solution 👀

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(setters(vis = ""))] // [!code focus]
    x1: u32,                      // [!code focus]

    #[builder(setters(vis = ""))] // [!code focus]
    x2: u32,                      // [!code focus]
}

use example_builder::{State, IsUnset, SetX1, SetX2};                        // [!code focus]
                                                                            // [!code focus]
impl<S: State> ExampleBuilder<S> {                                          // [!code focus]
    fn x1_and_x2(self, x1: u32, x2: u32) -> ExampleBuilder<SetX2<SetX1<S>>> // [!code focus]
    where                                                                   // [!code focus]
        S::X1: IsUnset,                                                     // [!code focus]
        S::X2: IsUnset,                                                     // [!code focus]
    {                                                                       // [!code focus]
        self.x1(x1).x2(x2)                                                  // [!code focus]
    }                                                                       // [!code focus]
}                                                                           // [!code focus]


let value = Example::builder()
    .x1_and_x2(1, 2)
    .build();

assert_eq!(value.x1, 1);
assert_eq!(value.x2, 2);
```

Pay attention that the type state in the return type of the setter is `SetX2<SetX1<S>>`. If you mess it up, the compiler error will help you by showing the exact type of the typestate.

For example, when first writing this solution I mistakenly wrote `SetX2<SetX1>`, and the compiler was helpful enough to correct me:

```rust ignore
self.x1(x1).x2(x2)
^^^^^^^^^^^^^^^^^^ expected `ExampleBuilder<SetX2<SetX1>>`,
                      found `ExampleBuilder<SetX2<SetX1<S>>>`
```

:::

## `IsSet` Trait

There is a counterpart to the `IsUnset` trait called `IsSet`. Ideally, we'd use negative trait impls and have `!IsSet` syntax, but this language feature is [not stable yet](https://github.com/rust-lang/rust/issues/68318). Therefore, `IsSet` and `IsUnset` are two separate traits, and they are implemented for mutually exclusive type states.

By the name of the `IsSet` trait, you've already probably figured out that it is implemented for type states where a particular member was set. This is useful for trait bounds on the finishing function. It must be callable only when all required members are set.

Let's see how it _could_ work in practice before we discuss the `IsComplete` trait.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,
    x2: Option<u32>,
}
```

This code _could_ generate the following finishing function:

```rust ignore
use example_builder::{State, IsSet};

impl<S: State> ExampleBuilder<S> {
    fn build(self) -> Example
    where
        S::X1: IsSet,
    { /**/ }
}
```

Notice how the `build` method requires only `x1` to be set. It doesn't care if `x2` was set or not, because `x2` is optional.

This `where S::X1: IsSet` is the core pillar of type safety. This way, the `build()` method knows it is always called with `x1` set and it doesn't have to do runtime validations: no `Result`, no `panic!()`.

## `IsComplete` Trait

The problem with the approach described higher that uses only the `IsSet` trait is that the number of bounds in the `where` clause of the finishing function grows with the number of required members.

Why is this a problem? Imagine you'd want to write a function, that accepts a closure that takes and returns a builder with all required members filled.

```rust
use bon::Builder;

#[derive(Builder)]
struct ExampleParams {
    x1: u32,
    x2: u32,
    x3: Option<u32>,
}

// Our goal is to have this API
example(|builder| builder.x1(1).x2(2));

// Below is how we could achieve this

// Import traits from the generated module
use example_params_builder::{State, IsSet};

fn example<S>(f: impl FnOnce(ExampleParamsBuilder) -> ExampleParamsBuilder<S>)
where
    S: State,
    S::X1: IsSet, // [!code highlight]
    S::X2: IsSet, // [!code highlight]
{
    let builder = f(ExampleParams::builder());
    let params = builder.build();
}
```

This doesn't scale well, because you need to write `S::{Member}: IsSet` for every required member. So, `where` bounds here are coupled with the number of required members and their names.

You can simplify this code by using the trait `IsComplete`. This trait is defined literally like this in the generated code:

```rust ignore
trait IsComplete: State {}

impl<S: State> IsComplete for S
where
    S::X1: IsSet,
    S::X2: IsSet,
{}
```

Thus, instead of writing `S::{Member}: IsSet` you can write a single bound `S: IsComplete`:

```rust ignore
fn example<S>(/**/)
where
    S: State,     // [!code --]
    S::X1: IsSet, // [!code --]
    S::X2: IsSet, // [!code --]
    S: IsComplete // [!code ++]
```

## Implied Bounds

Rust `1.79.0` added [associated trait bounds](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#bounds-in-associated-type-position) syntax, which can be used to make bounds on generic associated types implied.

This feature is useful for the trait `IsComplete`. If you enable the `implied-bounds` cargo feature, builder macros use the new syntax for bounds in
associated type position, which enables implied `IsSet` bounds for the type state
of required members.

The definition of the trait `IsComplete` changes like this:

```rust ignore
trait IsComplete: State {} // [!code --]
trait IsComplete: State<X1: IsSet, X2: IsSet> {} // [!code ++]
```

To understand how this is useful consider the following example:

```rust
#[derive(bon::Builder)]
struct Example {
    x1: u32,
    x2: Option<u32>,
}

use example_builder::{IsUnset, IsComplete};

impl<State: example_builder::State> ExampleBuilder<State> {
    fn build_with_default_x2(self) -> Example
    where
        State: IsComplete,
        State::X2: IsUnset,
    {
        self.x2(42).build()
    }
}
```

This code wouldn't compile without the `implied-bounds` cargo feature enabled. Without it, `State: IsComplete`
doesn't automatically imply `State::X1: IsSet`, so the builder type state returned
after `self.x2()` doesn't imply that the member `x1` is set, and thus `build()`
can't be called.

This is implemented as a cargo feature to make sure `bon` maintains a lower MSRV by default. If you need this, then enable this in your `Cargo.toml`, but beware that it increases your MSRV to `1.79.0`:

<!-- If you change this, make sure to update `scripts/sync-version.sh` -->

```toml
[dependencies]
bon = { version = "3.8", features = ["implied-bounds"] }
```
