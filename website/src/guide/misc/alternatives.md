---
aside: false
---

# Alternatives

There are several other existing alternative crates for generating builders. `bon` was designed based on many lessons learned from them. A table that compares the builder crates with some additional explanations is below.

<!-- Prevent separating wrapping in the table -->
<style>
.bon-guide-misc-alternatives-table tr > td:not(:first-child) {
    white-space: nowrap;
}
</style>
<div class="bon-guide-misc-alternatives-table">

<!-- If you want to edit the table below make sure to reduce the font size in the editor or turn off word wrap to view the table easier -->

| Feature                                  | `bon`                              | [`buildstructor`]         | [`typed-builder`]                    | [`derive_builder`]                            |
| ---------------------------------------- | ---------------------------------- | ------------------------- | ------------------------------------ | --------------------------------------------- |
| Builder for structs                      | ✅                                 | ✅                        | ✅                                   | ✅                                            |
| Builder for free functions               | ✅                                 |                           |                                      |
| Builder for associated methods           | ✅                                 | ✅                        |                                      |
| Panic safe                               | ✅                                 | ✅                        | ✅                                   | `build()`&nbsp;returns&nbsp;`Result`          |
| `Option<T>` makes members optional       | ✅                                 | ✅                        |                                      |                                               |
| `T` -> `Option<T>` is non-breaking       | ✅ [docs][bon-req-to-opt]          | ✅                        | via attr `strip_option`              | via attr [`strip_option`][db-so]              |
| Generates `T::builder()` method          | ✅                                 | ✅                        | ✅                                   | only `Builder::default()`                     |
| `Into` conversion in setters             | [opt-in][bon-into]                 | [implicit][bs-into]       | opt-in                               | [opt-in][db-into]                             |
| Validation in the finishing function     | ✅ [docs][bon-fallible-builder]    | ✅ [docs][bs-fall-finish] |                                      | ✅ [docs][db-fall-finish]                     |
| Validation in setters (fallible setters) | ✅&nbsp;attr [`with = closure`][b] |                           |                                      | ✅ `TryInto` via attr [`try_setter`][db-fs]   |
| Custom methods on builder                | ✅ via [direct impl block][bon-ts] |                           | ✅&nbsp; via [mutators]&nbsp;(attrs) | ✅ via [direct impl block][db-custom-methods] |
| `impl Trait`, elided lifetimes support   | ✅                                 |                           |                                      |
| Builder for `fn` hides original `fn`     | ✅                                 |                           |                                      |
| Special setters for collections          | [(see below)][r1]                  | ✅                        |                                      | ✅                                            |
| Builder by `&self`/`&mut self`           |                                    |                           |                                      | ✅                                            |

</div>

## Function Builder Paradigm Shift

If you ever hit a wall with `typed-builder` or `derive_builder`, you have no other choice but to hack something around their derive attributes syntax on a struct.

With `bon` and `buildstructor` you can simply change the syntax from `#[derive(Builder)]` on a struct to a `#[builder]` on a function, which gives you much more flexibility. It is [guaranteed to preserve compatibility](../misc/compatibility#switching-between-derivebuilder-and-builder-on-the-new-method) (non a breaking change). It allows you to create fallible, `async` or even `unsafe` builders naturally.

For example, suppose one day you found a need to count the number of fields that were default-initialized in the builder.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(default)]
    x1: u32,

    #[builder(default)]
    x2: u32,

    #[builder(skip = /* What? How can I calculate this here? 🤔 */)]
    defaults_counter: u32
}
```

::: tip

The attribute [`#[builder(skip)]`](../../reference/builder/member/skip) skips generating setters for a member. The field is initialized with the given expression instead.

:::

The attribute [`#[builder(skip)]`](../../reference/builder/member/skip) is the first obvious candidate for this use case. It also has analogues in `typed-builder` and `derive_builder`. However, it's actually too limited for this use case, because it doesn't have the required context.

At this point, you'd probably give up on `typed-builder` and `derive_builder`, because there is no way to express the required behavior with their attributes' syntax. However, it's as simple as pie with `bon` or `buildstructor`:

```rust
use bon::bon;

struct Example {
    x1: u32,
    x2: u32,
    defaults_counter: u32
}

#[bon]
impl Example {
    #[builder]
    fn new(x1: Option<u32>, x2: Option<u32>) -> Self {
        let mut defaults_counter = 0;
        let x1 = x1.unwrap_or_else(|| { defaults_counter += 1; 0 });
        let x2 = x2.unwrap_or_else(|| { defaults_counter += 1; 0 });
        Self { x1, x2, defaults_counter }
    }
}

assert_eq!(Example::builder().build().defaults_counter, 2);
assert_eq!(Example::builder().x1(1).build().defaults_counter, 1);
assert_eq!(Example::builder().x1(1).x2(2).build().defaults_counter, 0);
```

Ah... Simple just like regular Rust, isn't it? 😌

The chances of hitting a wall with function builders are close to zero. Even if you ever hit the wall with function builders you still have access to the [Typestate API](../typestate-api/) in `bon` for even more flexibility.

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

| `Vec<T>`             | `[T; N]`             | `*Map<K, V>`         | `*Set<K, V>`         |
| -------------------- | -------------------- | -------------------- | -------------------- |
| [`bon::vec![]`][vec] | [`bon::arr![]`][arr] | [`bon::map!{}`][map] | [`bon::set![]`][set] |

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
[Mutators]: https://docs.rs/typed-builder/latest/typed_builder/derive.TypedBuilder.html#mutators
[bon-on]: ../../reference/builder/top-level/on
[bon-into]: ../../reference/builder/member/into
[bon-req-to-opt]: ../misc/compatibilitymaking-a-required-member-optional
[bs-into]: https://docs.rs/buildstructor/latest/buildstructor/#into-field
[db-into]: https://docs.rs/derive_builder/latest/derive_builder/#generic-setters
[db-so]: https://docs.rs/derive_builder/latest/derive_builder/#setters-for-option
[bon-fallible-builder]: ../patterns/fallible-builders
[bs-fall-finish]: https://docs.rs/buildstructor/latest/buildstructor/#fallible
[db-fall-finish]: https://docs.rs/derive_builder/latest/derive_builder/#pre-build-validation
[b]: ../../reference/builder/member/with#fallible-closure
[db-custom-methods]: https://docs.rs/derive_builder/latest/derive_builder/#custom-setters-skip-autogenerated-setters
[db-fs]: https://docs.rs/derive_builder/latest/derive_builder/#fallible-setters
[r1]: #special-setter-methods-for-collections
[bon-ts]: ../typestate-api
