# Builder's Type Signature

On this page, you'll learn how to spell the exact builder's type 📝.

## Builder's Type Name

The builder's type name is derived from the underlying item from which it was generated by default.

<!--@include: ../../reference/builder/top-level/builder_type.md#name-->

It can also be overridden with [`#[builder(builder_type = NewName)]`](../../reference/builder/top-level/builder_type).

## Generic Typestate Parameter

Builders generated by `bon` macros use the typestate pattern. The builder's typestate doesn't depend on the syntax from which it was generated (struct or function).

Every time you call a setter the builder's type changes. The builder always contains a generic parameter `S` (stands for "state") at the end. This parameter holds the type state that describes what members were set in the builder.

It's probably easier to understand with an example. Don't worry, the next paragraph will explain everything 🐈.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    x1: u32,
    x2: u32,
}

// Import type states from the generated module
use example_builder::{SetX1, SetX2};

let builder: ExampleBuilder               = Example::builder();
let builder: ExampleBuilder<SetX1>        = builder.x1(1);
let builder: ExampleBuilder<SetX2<SetX1>> = builder.x2(2);
```

Notice the pattern here. Every time we set a member, we wrap the previous type state with a new `Set{Member}<S>` type state transition.

There is a special `Empty` type state, which is used as the default value for the generic parameter `S` in two places:

- The builder type itself: `ExampleBuilder<S = Empty>`
- The type state transitions: `Set{Member}<S = Empty>`

This is why we didn't have to mention the generic parameter for the first `ExampleBuilder`, and for `SetX1`.

The type states come from the builder's state module. The name of that module is the `snake_case` version of the builder's type name.

## Visibility

The type state module is private by default and only accessible within the module where the builder macro was used. The type states and other symbols, that we haven't covered yet, in that module all inherit their visibility from the builder's type visibility.

Visibility of the builder's type is by default derived from the visibility of the underlying struct or function from which the builder was generated.

Here is a simplified view of the generated builder type and state module with their visibility.

```rust ignore
// Let's suppose we had this derive on a struct with `pub` visibility
// #[derive(bon::Builder)]
pub struct Example {
    x1: u32
}

// Builder type inherits the `pub` visibility from the underlying `Example` struct
// from which it was generated.
pub struct ExampleBuilder<S: /**/> { /**/ }

// Typestate module is private by default. It means it is accessible only within
// the surrounding module.
mod example_builder {
    // The type states inherit the builder type's visibility, which is `pub`
    pub struct SetX1<S = Empty> { /**/ }
    pub struct Empty { /**/ }
    // ...
}

```

Notice how we have `pub` symbols defined inside of a private module. Such a pattern ensures that the builder type is _accessible_, but _unnameable_ outside of the module where it was generated. This is similar to a sealed trait, but it's a "sealed" type in this case.

If you want to expose your builder's type signature, you need to add [`#[builder(state_mod(vis = "..."))]`](../../reference/builder/top-level/state_mod), where `...` can be `pub` or `pub(crate)` or any other visibility you want to assign to the state module instead of the default private visibility.

## Example Rustdoc

You can see the `rustdoc` API reference generated for this example [here](https://docs.rs/bon-sandbox/latest/bon_sandbox/state_mod/minimal/). Note that it was generated with `#[builder(state_mod(vis = "pub"))]`, otherwise, it wouldn't appear in public documentation.

## Other Generic Parameters

The builder inherits all generic parameters from the struct or function from which it was generated.

Functions may even use anonymous/elided lifetimes and `impl Trait` syntax. Every such anonymous/elided lifetime or `impl Trait` will get a separate generic parameter generated in the builder's type automatically.

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example<'a, T> {
    x1: &'a T,
}

//                         'a┐
let builder: ExampleBuilder<'_, bool, _> = Example::builder().x1(&true);
//                              ^^^^  ^- type state (always last)
//                             T┘
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    x1: &impl Clone
) {}

// lifetime param from `&...`┐
let builder: ExampleBuilder<'_, bool, _> = example().x1(&true);
//                              ^^^^  ^- type state (always last)
//  type param from `impl Clone`┘
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn method(x1: &impl Clone) {}
}

//       lifetime param from `&...`┐
let builder: ExampleMethodBuilder<'_, bool, _> = Example::method().x1(&true);
//                                    ^^^^  ^- type state (always last)
//        type param from `impl Clone`┘
```

:::

If there is a mix of named and anonymous lifetimes or named generic types and `impl Trait`, then the generated generic lifetimes and types will be appended at the end of the list of named lifetimes and other named generic parameters respectively.

## What's Next?

Now you know the mechanics of how a builder's type is built, so you can denote it when declaring function parameters or return type, or storing the builder in a struct.

However, to be able to write useful custom methods on the builder, you'll need to know the traits behind the type states. Go to the next page to learn more.
