
# `into`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

::: tip

This attribute is also configurable via the top-level [`#[builder(on(...))]`](../top-level/on)

:::

Changes the signature of the setters to accept [`impl Into<T>`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html), where `T` is the type of the member.

For [optional members](../../../guide/optional-members), the `maybe_{member}()` setter method will accept an `Option<impl Into<T>>` type instead of just `Option<T>`.

For members that use `#[builder(default = expression)]`, the `expression` will be converted with `Into::into`.

This parameter is often used with the `String` type, which allows you to pass `&str` into the setter without calling `.to_owned()` or `.to_string()` on it.

See the ["Into Conversions In-Depth"](../../../guide/into-conversions-in-depth) page that shows the common patterns and antipatterns of `impl Into<T>`.

## Examples

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(into)] // [!code highlight]
    name: String,

    #[builder(into)] // [!code highlight]
    description: Option<String>,

    // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
    #[builder(into, default = "anon")]                                      // [!code highlight]
    group: String
}

Example::builder()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .build();
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn example(
    #[builder(into)] // [!code highlight]
    name: String,

    #[builder(into)] // [!code highlight]
    description: Option<String>,

    // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
    #[builder(into, default = "anon")]                                      // [!code highlight]
    group: String
) {}

example()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .call();
```

```rust [Associated method argument]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(into)] // [!code highlight]
        name: String,

        #[builder(into)] // [!code highlight]
        description: Option<String>,

        // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
        #[builder(into, default = "anon")]                                      // [!code highlight]
        group: String
    ) {}
}

Example::example()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .call();
```

:::
