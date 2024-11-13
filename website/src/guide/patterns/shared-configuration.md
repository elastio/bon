---
outline: deep
---

# Shared Configuration

On this page, you'll learn how to share common configurations for builders to avoid code duplication.

## Problem statement

As an example, let's suppose you want to enable [`Into` conversions](../patterns/into-conversions-in-depth) for specific types across all your builders and maybe also override the name of the finishing function that consumes the builder from the default `build` to `finish`.

You'll quickly run into a problem, where you need to repeat the same configuration for every usage of the builder macro.

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

This code uses the [`#[builder(on(...))]`](../../reference/builder/top-level/on) attribute to configure the types of members for which `bon` should enable `Into` conversions.

:::

## Solution

To overcome this problem you can utilize the [`macro_rules_attribute`] crate. It allows you to declare an [`attribute_alias`](https://docs.rs/macro_rules_attribute/latest/macro_rules_attribute/macro.attribute_alias.html) that defines all the shared configuration for your builders and makes it reusable.

Use this approach if you have a lot of structs/functions in your crate that need a builder.

### Structs

```rust
use macro_rules_attribute::{attribute_alias, apply};

// The alias can also be defined in a separate module.
// Under the hood, it creates a macro with `pub(crate)` visibility.
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

### Functions

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

### Methods

Unfortunately, this technique doesn't quite work with associated methods (functions inside impl blocks) due to the limitations of proc macro attribute expansion order. The `#[bon]` macro on top of the impl block is expanded first before the `#[apply(...)]` macro inside of the impl block, so `#[bon]` doesn't see the configuration expanded from the `#[apply(...)]`.

There is a proposed solution to this problem in the issue [#144](https://github.com/elastio/bon/issues/144). Add a 👍 to that issue if your use case needs a solution for this. It would be even better if you left a comment describing your particular use case where you'd like to have this feature.

[`macro_rules_attribute`]: https://docs.rs/macro_rules_attribute/latest/macro_rules_attribute/
