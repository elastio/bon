# Builder's Type Signature

On this page, you'll learn how to spell the exact builder's type 📝.

## Builder's Type Name

The builder's type name is derived from the underlying item from which it was generated by default.

<!--@include: ../../reference/builder/top-level/builder_type.md#name-->

It can also be overridden with [`#[builder(builder_type = NewName)]`](../../reference/builder/top-level/builder_type).

## Generic Typestate Parameter

Builders generated by `bon` macros use the typestate pattern. Every time you call a setter the builder's type changes. The builder's typestate doesn't depend on the syntax from which it was generated (struct or function).

The builder always contains a generic parameter `S` (stands for "state") at the end. This parameter holds the type state that describes what members were set in the builder.

It's probably easier to understand with an example. Don't worry, the next paragraph will explain it.

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

The type states come from the builder's state module. The name of that module is the `snake_case` version of the builder's name.

## Visibility

The type state module is private by default, and only accessible within the module where the builder macro was used. This way the builder type becomes unnameable outside of the module where it was generated.

If you want to expose your builder's type signature, you need to add [`#[builder(state_mod(vis = "..."))]`](../../reference/builder/top-level/state_mod), where `...` can be `pub` or `pub(crate)` or any other visibility you want to expose the state module under.

## Example Rustdoc

You can see the `rustdoc` API reference generated for this example [here](https://docs.rs/bon/latest/bon/examples/minimal/). Note that it was generated with `#[builder(state_mod(vis = "pub"))]`, otherwise, it wouldn't appear in public documentation.

## Other Generic Parameters

The builder inherits all generic parameters from the struct or function from which it was generated.

Functions may even use anonymous lifetimes and `impl Trait` syntax. Every such anonymous lifetime or `impl Trait` will get a separate generic parameter generated in the builder's type automatically.

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example<'a, T> {
    x1: &'a T,
}

let builder: ExampleBuilder<'_, bool, _> = Example::builder().x1(&true);
                                   // ^- type state (always last)
```

```rust [Function]
use bon::builder;

#[builder]
fn example(
    x1: &impl Clone
) {}

let builder: ExampleBuilder<'_, bool, _> = example().x1(&true);
                                   // ^- type state (always last)
```

```rust [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn method(x1: &impl Clone) {}
}

let builder: ExampleMethodBuilder<'_, bool, _> = Example::method().x1(&true);
                                         // ^- type state (always last)
```

:::

If there is a mix of named and anonymous lifetimes or named generic types and `impl Trait`, then the generated generic lifetimes and types will be appended at the end of the list of named lifetimes and other named generic parameters respectively.

## What's Next?

Now you know the mechanics of how a builder's type is built, so you can denote it when it's returned from a function or stored in a struct.

However, to be able to write useful custom methods on the builder, you'll need to know the `trait`s behind the type states. Go to the next page to learn more.