---
aside: false
---

# Alternatives

There are several other existing alternative crates that generate builders. `bon` was designed with many lessons learned from them. Here is a table that compares the builder crates with some additional explanations below.

<!-- If you want to edit the table below make sure to reduce the font size in the editor or turn off word wrap to easier view the table -->

Feature                                                  | `bon`                                                        | [`buildstructor`]               | [`typed-builder`]                                                   | [`derive_builder`]
---------------------------------------------------------|--------------------------------------------------------------|---------------------------------|---------------------------------------------------------------------|-------------------
Builder for structs                                      | :white_check_mark:                                           | :white_check_mark:              | :white_check_mark:                                                  | :white_check_mark:
Builder for free functions                               | :white_check_mark:                                           |                                 |                                                                     |
Builder for associated methods                           | :white_check_mark:                                           | :white_check_mark:              |                                                                     |
Panic safe                                               | :white_check_mark:                                           | :white_check_mark:              | :white_check_mark:                                                  | `build()` returns a `Result`
Member of `Option` type is optional by default           | :white_check_mark:                                           | :white_check_mark:              | <span class="nobr">opt-in `#[builder(default)]`</span>              | <span class="nobr">opt-in `#[builder(default)]`</span>
Making required member optional is compatible by default | :white_check_mark:                                           | :white_check_mark:              | <span class="nobr">opt-in `#[builder(setter(strip_option))]`</span> | <span class="nobr">opt-in `#[builder(setter(strip_option))]`</span>
Generates `T::builder()` method                          | :white_check_mark:                                           | :white_check_mark:              | :white_check_mark:                                                  | only `Builder::default()`
`Into` conversion in setters                             | opt-in ([members subset][bon-on], [single member][bon-into]) | [implicit (automatic)][bs-into] | opt-in (all members + out-out, single member)                       | [opt-in (all members, single member)][db-into]
 `impl Trait` supported for functions                    | :white_check_mark:                                           |                                 |                                                                     |
Anonymous lifetimes supported for functions              | :white_check_mark:                                           |                                 |                                                                     |
`Self` mentions in functions/structs are supported       | :white_check_mark:                                           |                                 |                                                                     |
Positional function is hidden by default                 | :white_check_mark:                                           |                                 |                                                                     |
Special setter methods for collections                   | [(see below)][r1]                                            | :white_check_mark:              |                                                                     | :white_check_mark:
Custom methods can be added to the builder type          |                                                              |                                 | :white_check_mark: ([mutators])                                     | :white_check_mark:
Builder may be configured to use &self/&mut self         |                                                              |                                 |                                                                     | :white_check_mark:

## Function builder fallback paradigm

The builder crates `typed-builder` and `derive_builder` have a bunch of attributes that allow users to insert custom behaviour into the building process of the struct. However, `bon` and `buildstructor` avoid the complexity of additional config attributes for advanced use cases by proposing the user fallback to defining a custom function with the `#[builder]` attached to it where it's possible to do anything you want.

However, `bon` still provides some simple attributes for common use cases to configure the behaviour without falling back to a more verbose syntax.

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

Why is there an explicit `main()` function in this code snippet 🤔? It's a long story explained in a [blog post](/blog/the-weird-of-function-local-types-in-rust) (feel free to skip).

:::

This feature isn't available today in `bon`, but it's planned for the future. However, it won't be enabled by default, but rather be opt-in like it is in `derive_builder`.

The problem with this feature is that a setter that pushes an element into a collection like that may confuse the reader in case if only one element is pushed. This may hide the fact that the member is actually a collection called `friends` in the plural. However, this feature is still useful to provide backwards compatibility when changing the type of a member from `T` or `Option<T>` to `Collection<T>`.

Alternatively, `bon` provides a separate solution. `bon` exposes the following macros that provide convenient syntax to create collections.

`Vec<T>`             | `[T; N]`             | `*Map<K, V>`         | `*Set<K, V>`
---------------------|----------------------|----------------------|---------------------
[`bon::vec![]`][vec] | [`bon::arr![]`][arr] | [`bon::map!{}`][map] | [`bon::set![]`][set]

These macros share a common feature that every element of the collection is converted with `Into` to shorten the syntax if you, for example, need to initialize a `Vec<String>` with items of type `&str`. Use these macros only if you need this behaviour, or ignore them if you want to be explicit in code and avoid implicit `Into` conversions.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
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

Another difference is that fields of collection types are considered required by default, which isn't the case in `buildstructor`.

[`buildstructor`]: https://docs.rs/buildstructor/latest/buildstructor/
[`typed-builder`]: https://docs.rs/typed-builder/latest/typed_builder/
[`derive_builder`]: https://docs.rs/derive_builder/latest/derive_builder/
[vec]: https://docs.rs/bon/latest/bon/macro.vec.html
[arr]: https://docs.rs/bon/latest/bon/macro.arr.html
[map]: https://docs.rs/bon/latest/bon/macro.map.html
[set]: https://docs.rs/bon/latest/bon/macro.set.html
[mutators]: https://docs.rs/typed-builder/latest/typed_builder/derive.TypedBuilder.html#mutators
[bon-on]: ../../reference/builder/top-level/on
[bon-into]: ../../reference/builder/member/into
[bs-into]: https://docs.rs/buildstructor/latest/buildstructor/#into-field
[db-into]: https://docs.rs/derive_builder/latest/derive_builder/#generic-setters
[r1]: #special-setter-methods-for-collections
