---
aside: false
---

# Alternatives

There are several other existing alternative crates that generate builders. `bon` was designed based on lessons learned from all of them. Here is a table that compares the builder crates with some additional explanations below.

<!-- If you want to edit the table below make sure to reduce the font size in editor or turn off word wrap to easier view the table -->

Feature                                                  | `bon`              | [`buildstructor`]  | [`typed-builder`]                                                   | [`derive_builder`]
---------------------------------------------------------|--------------------|--------------------|---------------------------------------------------------------------|-------------------
Builder for structs                                      | :white_check_mark: | :white_check_mark: | :white_check_mark:                                                  | :white_check_mark:
Builder for free functions                               | :white_check_mark: |                    |                                                                     |
Builder for associated methods                           | :white_check_mark: | :white_check_mark: |                                                                     |
Panic safe                                               | :white_check_mark: | :white_check_mark: | :white_check_mark:                                                  | `build()` returns a `Result`
Member of `Option` type is optional by default           | :white_check_mark: | :white_check_mark: | <span class="nobr">opt-in `#[builder(default)]`</span>              | <span class="nobr">opt-in `#[builder(default)]`</span>
Making required member optional is compatible by default | :white_check_mark: | :white_check_mark: | <span class="nobr">opt-in `#[builder(setter(strip_option))]`</span> | <span class="nobr">opt-in `#[builder(setter(strip_option))]`</span>
Generates `T::builder()` method                          | :white_check_mark: | :white_check_mark: | :white_check_mark:                                                  | only `Builder::default()`
Automatic `Into` conversion in setters                   | :white_check_mark: | :white_check_mark: |                                                                     |
 `impl Trait` supported for functions                    | :white_check_mark: |                    |                                                                     |
Anonymous lifetimes supported for functions              | :white_check_mark: |                    |                                                                     |
`Self` mentions in functions/structs are supported       | :white_check_mark: |                    |                                                                     |
Positional function is hidden by default                 | :white_check_mark: |                    |                                                                     |
Special setter methods for collections                   | [(see below)][r1]  | :white_check_mark: |                                                                     | :white_check_mark:
Custom methods can be added to the builder type          |                    |                    | :white_check_mark: ([mutators])                                     | :white_check_mark:
Builder may be configured to use &self/&mut self         |                    |                    |                                                                     | :white_check_mark:

## Function builder fallback paradigm

The builder crates `typed-builder` and `derive_builder` have a bunch of attributes that allow users to insert custom behavior into the building process of the struct. However, `bon` and `buildstructor` avoid the complexity of additional config attributes for advanced use cases by proposing the user to fallback to defining a custom function with the `#[builder]` attached to it where it's possible to do anything you want.

However, `bon` still provides some simple attributes for common use cases to configure the behavior without falling back to a more verbose syntax.

## Special setter methods for collections

Other builder crates provide a way to generate methods to build collections one element at a time. For example, `buildstructor` even generates such methods by default:

```rust
#[derive(buildstructor::Builder)]
struct User {
    friends: Vec<String>
}

fn main() {
    User::builder()
        .friend("Foo")
        .friend("Bar")
        .friend("`String` value is also accepted".to_owned())
        .build();
}
```

::: tip

Why is there an explicit `main()` function in this code snippet 🤔? It's a long story explained in a [blog post](../../blog/avoid-child-modules-in-macros) (feel free to skip).

:::

However, `#[bon::builder]` doesn't do that. Also a setter that pushes an element into a collection like that may confuse the reader in case if only one element is pushed. This may hide the fact that there is a `friends` field which is actually a collection.

Constructing a collection is a bit different problem to solve, so `bon` provides a separate solution. `bon` exposes a `bon::vec![]` macro (macros for other collections types are planned for future updates!) that includes automatic `Into` conversion for every argument. So in `bon` syntax it would look like this:

```rust
use bon::builder;

#[builder]
struct User {
    friends: Vec<String>
}

User::builder()
    .friends(bon::vec![
      "Foo",
      "Bar",
      "`String` value is also accepted".to_owned(),
    ])
    .build();
```

Also, fields of collection types are considered required by default, which isn't the case in `buildstructor`.

[`buildstructor`]: https://docs.rs/buildstructor/latest/buildstructor/
[`typed-builder`]: https://docs.rs/typed-builder/latest/typed_builder/
[`derive_builder`]: https://docs.rs/derive_builder/latest/derive_builder/
[mutators]: https://docs.rs/typed-builder/latest/typed_builder/derive.TypedBuilder.html#mutators
[r1]: #special-setter-methods-for-collections