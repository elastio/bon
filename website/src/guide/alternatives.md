---
aside: false
---

# Alternatives

There are several other existing alternative crates for generating builders. `bon` was designed based on many lessons learned from them. A table that compares the builder crates with some additional explanations is below.

<!-- Prevent wrapping in the table -->
<style>
.bon-guide-misc-alternatives-table tr > td:not(:first-child) {
    white-space: nowrap;
}
</style>
<div class="bon-guide-misc-alternatives-table">

<!-- If you want to edit the table below make sure to reduce the font size in the editor or turn off word wrap to view the table easier -->

| Feature                                  | `bon`                              | [`buildstructor`]         | [`typed-builder`]        | [`derive_builder`]                            |
| ---------------------------------------- | ---------------------------------- | ------------------------- | ------------------------ | --------------------------------------------- |
| Builder for structs                      | ✅                                 | ✅                        | ✅                       | ✅                                            |
| Builder for functions                    | ✅                                 |                           |                          |
| Builder for methods                      | ✅                                 | ✅                        |                          |
| Panic safe                               | ✅                                 | ✅                        | ✅                       | `build()` returns `Result`                    |
| `Option<T>` makes members optional       | ✅                                 | ✅                        |                          |                                               |
| `T` -> `Option<T>` is non-breaking       | ✅ [docs][bon-req-to-opt]          | ✅                        | via attr `strip_option`  | via attr [`strip_option`][db-so]              |
| Generates `T::builder()` method          | ✅                                 | ✅                        | ✅                       | only `Builder::default()`                     |
| `Into` conversion in setters             | [opt-in][bon-into]                 | [implicit][bs-into]       | opt-in                   | [opt-in][db-into]                             |
| Validation in the finishing function     | ✅ [docs][bon-fallible-builder]    | ✅ [docs][bs-fall-finish] |                          | ✅ [docs][db-fall-finish]                     |
| Validation in setters (fallible setters) | ✅ attr [`with = closure`][b]      |                           |                          | ✅ `TryInto` via attr [`try_setter`][db-fs]   |
| Custom methods on builder                | ✅ via [direct impl block][bon-ts] |                           | ✅ via attr [mutators]   | ✅ via [direct impl block][db-custom-methods] |
| Custom fields on builder                 | ✅ attr [`field`][bon-field]       |                           | ✅ attr [`via_mutators`] | ✅ attr [`field`][db-field]                   |
| `impl Trait`, elided lifetimes support   | ✅                                 |                           |                          |
| Builder for `fn` hides original `fn`     | ✅                                 |                           |                          |
| Special setters for collections          | [(see below)][collections]         | ✅                        |                          | ✅                                            |
| Builder by `&self`/`&mut self`           |                                    |                           |                          | ✅                                            |
| [Generates nice docs][gen-docs-cmp]      | ✅                                 |                           |                          | ✅                                            |
| Getters on the builder                   | ✅                                 |                           |                          | ✅                                            |

</div>

## Function Builder Paradigm Shift

If you ever hit a wall 🧱 with `typed-builder` or `derive_builder`, you'll have to hack something around their derive attributes syntax on a struct. With `bon` or `buildstructor` you can simply change the syntax from `#[derive(Builder)]` on a struct to a `#[builder]` on a function to gain more flexibility at any time 🤸. It is [guaranteed to preserve compatibility](./basics/compatibility#switching-between-derive-builder-and-builder-on-the-new-method), meaning it's not a breaking change.

### Example

Suppose you already had a struct like the following with a builder derive:

```rust ignore
use bon::Builder;

#[derive(Builder)]
pub struct Line {
    x1: u32,
    y1: u32,

    x2: u32,
    y2: u32,
}

// Suppose this is your users' code
Line::builder().x1(1).y1(2).x2(3).y2(4).build();
```

Then you decided to refactor 🧹 your struct's internal representation by extracting a private utility `Point` type:

```rust compile_fail
use bon::Builder;

#[derive(Builder)]
pub struct Line {
    point1: Point,
    point2: Point,
}

// Private
struct Point {
    x: u32,
    y: u32,
}

// Suppose this is your users' code (it no longer compiles)
Line::builder().x1(1).y1(2).x2(3).y2(4).build(); // [!code error]
//                 ^^- error[E0599]: no method named `x1` found for struct `LineBuilder` // [!code error]
//                                   available methods: `point1(Point)`, `point2(Point)` // [!code error]
```

There are two problems with `#[derive(Builder)]` syntax in this case:

1.  This refactoring becomes a breaking change to `Line`'s builder API.
2.  The private utility `Point` type leaks through the builder API via `point1`, and `point2` setters.

The fundamental problem is that the builder's API is _coupled_ ⛓️ with your struct's internal representation. It's literally `derive`d from the fields of your struct.

### Suffering

If you were using `typed-builder` or `derive_builder`, you'd be stuck for a while trying to find the magical 🪄 combination of attributes that would let you do this change without breaking compatibility or leakage of the private `Point` type.

With no solution in sight, you'd then fall back to writing the same builder manually. You'd probably expand the builder derive macro and edit the generated code directly, which, ugh... hurts.

However, that would be especially painful with `typed-builder`, which generates a complex typestate that is not human-readable and maintainable enough by hand. It also references some internal `#[doc(hidden)]` symbols from the `typed-builder` crate.

::: tip

In contrast, `bon`'s type state **is** human-readable, maintainable, and [documented](./typestate-api)

:::

### Behold the Function-Based Builder

This change is as simple as pie with `bon` or `buildstructor`. You need no more derives on a struct. Its internal representation can be decoupled from the builder by deriving the builder from a method:

```rust
use bon::bon;

pub struct Line {
    point1: Point,
    point2: Point,
}

struct Point {
    x: u32,
    y: u32,
}

#[bon]
impl Line {
    #[builder]
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            point1: Point { x: x1, y: y1 } ,
            point2: Point { x: x2, y: y2 } ,
        }
    }
}

// Suppose this is your users' code (it compiles after this change, yay 🎉!)
Line::builder().x1(1).y1(2).x2(3).y2(4).build();
```

Moreover, this approach offers you a completely new dimension of flexibility:

- Need some validation? Just change the `new()` method to return a `Result`. The generated `build()` method will then become fallible.
- Need to do an `async` operation in the constructor? Just make your constructor `async` and your `build()` will return a `Future`.
- Need some adrenaline 💉? Just add `unsafe`, and... you get the idea 😉.

---

The chances of hitting a wall with function builders are close to zero, and even if you ever do, you still have access to the [Typestate API](./typestate-api) in `bon` for even more flexibility.

## Generated Docs Comparison

Here is a table that compares the `rustdoc` output for builders generated by different crates based on different syntax.

| Underlying syntax | `bon`                | `buildstructor`   | `typed-builder`   | `derive_builder`  |
| ----------------- | -------------------- | ----------------- | ----------------- | ----------------- |
| Struct            | [Link][struct-bon]   | [Link][struct-bs] | [Link][struct-tb] | [Link][struct-db] |
| Function          | [Link][function-bon] |
| Method            | [Link][method-bon]   | [Link][method-bs] |

[struct-bon]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/bon/
[struct-bs]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/buildstructor/
[struct-tb]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/typed_builder/
[struct-db]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/structs/derive_builder/
[function-bon]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/functions/bon/
[method-bon]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/methods/bon/
[method-bs]: https://docs.rs/bon-sandbox/latest/bon_sandbox/docs_comparison/methods/buildstructor/

All builders were configured to produce roughly similar builder APIs. The notable exceptions are:

- `buildstructor` doesn't support `#[builder(default)]` and `#[builder(into)]`-like annotations;
- `buildstructor` doesn't support doc comments on function arguments;
- `derive_builder` doesn't support typestate-based builders;

Docs generated by `typed-builder` and `buildstructor` suffer from the problem of noisy generics. This problem significantly worsens with the number of fields/arguments in structs/functions. `bon` solves this problem by using a trait-based design for its [typestate](./typestate-api).

`bon` also includes the default values assigned via [`#[builder(default)]`](../reference/builder/member/default) in the docs ([more examples here](https://docs.rs/bon-sandbox/latest/bon_sandbox/attr_default/struct.ExampleBuilder.html)).

## Special Setter Methods for Collections

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

It is possible to implement this with `bon` using custom fields and custom setters. See an example of how this can be done [here](../guide/typestate-api/builder-fields).

Alternatively, `bon` provides a separate solution. `bon` exposes the following macros that provide convenient syntax to create collections.

| `Vec<T>`             | `[T; N]`             | `*Map<K, V>`         | `*Set<K, V>`         |
| -------------------- | -------------------- | -------------------- | -------------------- |
| [`bon::vec![]`][vec] | [`bon::arr![]`][arr] | [`bon::map!{}`][map] | [`bon::set![]`][set] |

These macros share a common feature that every element of the collection is converted with `Into` to shorten the syntax if you, for example, need to initialize a `Vec<String>` with items of type `&str`. Use these macros only if you need this behaviour, or ignore them if you want to be explicit in code and avoid implicit `Into` conversions.

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

Another difference is that members of collection types are considered required by default in `bon`, which isn't the case in `buildstructor`.

[`buildstructor`]: https://docs.rs/buildstructor/latest/buildstructor/
[`typed-builder`]: https://docs.rs/typed-builder/latest/typed_builder/
[`derive_builder`]: https://docs.rs/derive_builder/latest/derive_builder/
[vec]: https://docs.rs/bon/latest/bon/macro.vec.html
[arr]: https://docs.rs/bon/latest/bon/macro.arr.html
[map]: https://docs.rs/bon/latest/bon/macro.map.html
[set]: https://docs.rs/bon/latest/bon/macro.set.html
[collections]: #special-setter-methods-for-collections
[gen-docs-cmp]: #generated-docs-comparison

<!-- bon -->

[bon-on]: ../reference/builder/top-level/on
[bon-into]: ../reference/builder/member/into
[bon-req-to-opt]: ./basics/compatibility#making-a-required-member-optional
[bon-fallible-builder]: ./patterns/fallible-builders
[bon-ts]: ./typestate-api
[b]: ../reference/builder/member/with#fallible-closure
[bon-field]: ../reference/builder/member/field

<!-- buildstructor -->

[bs-into]: https://docs.rs/buildstructor/latest/buildstructor/#into-field
[bs-fall-finish]: https://docs.rs/buildstructor/latest/buildstructor/#fallible

<!-- typed-builder -->

[mutators]: https://docs.rs/typed-builder/latest/typed_builder/derive.TypedBuilder.html#mutators
[`via_mutators`]: https://docs.rs/typed-builder/latest/typed_builder/derive.TypedBuilder.html#mutators

<!-- derive_builder -->

[db-into]: https://docs.rs/derive_builder/latest/derive_builder/#generic-setters
[db-so]: https://docs.rs/derive_builder/latest/derive_builder/#setters-for-option
[db-fall-finish]: https://docs.rs/derive_builder/latest/derive_builder/#pre-build-validation
[db-custom-methods]: https://docs.rs/derive_builder/latest/derive_builder/#custom-setters-skip-autogenerated-setters
[db-fs]: https://docs.rs/derive_builder/latest/derive_builder/#fallible-setters
[db-field]: https://docs.rs/derive_builder/latest/derive_builder/#completely-custom-fields-in-the-builder
