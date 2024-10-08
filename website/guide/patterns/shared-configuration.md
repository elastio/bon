---
outline: deep
---

# Shared Configuration

On this page, you'll learn how to share common configurations for builders to avoid code duplication.

## Problem statement

As an example, let's suppose you want to enable [`Into` conversions](./into-conversions-in-depth) for specific types across all your builders and maybe also override the name of the finishing function that consumes the builder from the default `build` to `finish`. The problem that you'll quickly run into is that you'll need to repeat the same configuration everywhere:

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(
    on(String, into),   // [!code highlight]
    on(Box<_>, into),   // [!code highlight]
    finish_fn = finish, // [!code highlight]
)]
struct MyLovelyStruct1 { /**/ }


#[derive(Builder)]
#[builder(
    on(String, into),   // [!code highlight]
    on(Box<_>, into),   // [!code highlight]
    finish_fn = finish, // [!code highlight]
)]
struct MyLovelyStruct2 { /**/ }
```

::: tip

This code uses the [`#[builder(on(...))]`](../../reference/builder#on) attribute to configure the types of members for which `bon` should enable `Into` conversions.

:::

The annoying thing here is that we need to copy all these configurations on every struct where we derive the builder.

## Solution

### Structs

To overcome this problem we can utilize the [`macro_rules_attribute`] crate. It allows you to declare an [`attribute_alias`](https://docs.rs/macro_rules_attribute/latest/macro_rules_attribute/macro.attribute_alias.html) that defines all the shared configuration for your builders and makes it reusable.

So with the [`macro_rules_attribute`] your code will look like this:

```rust
use macro_rules_attribute::{attribute_alias, apply};

// The alias can also be defined in a separate module.
// Under the hood it creates a macro with `pub(crate)` visibility.
attribute_alias! {
    #[apply(derive_builder!)] =
        #[derive(::bon::Builder)]
        #[builder(
            on(String, into),   // [!code highlight]
            on(Box<_>, into),   // [!code highlight]
            finish_fn = finish, // [!code highlight]
        )];
}

#[apply(derive_builder!)]
struct MyLovelyStruct1 { /**/ }

#[apply(derive_builder!)]
struct MyLovelyStruct2 { /**/ }
```

Use this approach if you have a lot of structs in your crate that need a builder. Adding [`macro_rules_attribute`] to your dependencies shouldn't have a noticeable impact on the compilation performance. This approach [was tested](https://github.com/ayrat555/frankenstein/blob/91ac379a52ed716e09632f78b984852c85f2adaa/src/macros.rs#L3-L14) on a crate with ~320 structs that derive a builder and compile time was the same as before adding the [`macro_rules_attribute`] crate.

### Free functions

A similar approach works with `#[bon::builder]` on free functions.
**Example:**

```rust

use macro_rules_attribute::{attribute_alias, apply};

attribute_alias! {
    #[apply(builder!)] =
        #[::bon::builder(
            on(String, into),   // [!code highlight]
            on(Box<_>, into),   // [!code highlight]
            finish_fn = finish, // [!code highlight]
        )];
}

#[apply(builder!)]
fn my_lovely_fn1(/**/) { /**/ }

#[apply(builder!)]
fn my_lovely_fn2(/**/) { /**/ }
```

### Associated methods

Unfortunately, this technique doesn't quite work with associated methods (functions inside impl blocks) due to the limitations of proc macro attribute expansion order. The `#[bon]` macro on top of the impl block is expanded first before the `#[apply(...)]` macro inside of the impl block, so `#[bon]` doesn't see the configuration expanded from the `#[apply(...)]`.

There is a proposed solution to this problem in the issue [#elastio/bon#144](https://github.com/elastio/bon/issues/144). Add a 👍 to that issue if your use case needs a solution for this, and maybe leave a comment about your particular use case where you'd like to have this feature.

[`macro_rules_attribute`]: https://docs.rs/macro_rules_attribute/latest/macro_rules_attribute/
