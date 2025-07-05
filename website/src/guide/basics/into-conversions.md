# `Into` conversions

If you have members of type `String`, or `PathBuf`, and you need to set them to a hard-coded string literal, then you have to write `.to_owned()` or `.to_string()` or `.into()`.

::: code-group

```rust [Struct]
use std::path::PathBuf;

#[derive(bon::Builder)]  // [!code focus]
struct Example {         // [!code focus]
    name: String,        // [!code focus]
    description: String, // [!code focus]
    path: PathBuf,       // [!code focus]
}                        // [!code focus]

Example::builder()
    .name("Bon".to_owned())                      // [!code focus]
    .description("Awesome crate 🐱".to_string()) // [!code focus]
    .path("/path/to/bon".into())                 // [!code focus]
    .build();
```

```rust [Function]
use std::path::PathBuf;

#[bon::builder]          // [!code focus]
fn example(              // [!code focus]
    name: String,        // [!code focus]
    description: String, // [!code focus]
    path: PathBuf,       // [!code focus]
) {}                     // [!code focus]

example()
    .name("Bon".to_owned())                      // [!code focus]
    .description("Awesome crate 🐱".to_string()) // [!code focus]
    .path("/path/to/bon".into())                 // [!code focus]
    .call();
```

```rust [Method]
use std::path::PathBuf;

struct Example;

#[bon::bon]
impl Example {
    #[builder]               // [!code focus]
    fn example(              // [!code focus]
        name: String,        // [!code focus]
        description: String, // [!code focus]
        path: PathBuf,       // [!code focus]
    ) {}                     // [!code focus]
}

Example::example()
    .name("Bon".to_owned())                      // [!code focus]
    .description("Awesome crate 🐱".to_string()) // [!code focus]
    .path("/path/to/bon".into())                 // [!code focus]
    .call();
```

:::

However, you can ask `bon` to generate setters that accept `impl Into<T>` to remove the need for manual conversion.

This can be configured with [`#[builder(into)]`](../../reference/builder/member/into) for a single member or with [`#[builder(on({type}, into))]`](../../reference/builder/top-level/on) for many members at once.

::: code-group

```rust [Struct]
use std::path::PathBuf;

// All setters for members of type `String` will accept `impl Into<String>` // [!code highlight]
#[derive(bon::Builder)]                                                     // [!code highlight]
#[builder(on(String, into))]                                                // [!code highlight]
struct Example {
    name: String,
    description: String,

    // The setter only for this member will accept `impl Into<PathBuf>`    // [!code highlight]
    #[builder(into)]                                                       // [!code highlight]
    path: PathBuf,
}

Example::builder()
    .name("Bon") // [!code highlight]
    .description("Awesome crate 🐱") // [!code highlight]
    .path("/path/to/your/heart") // [!code highlight]
    .build();
```

```rust [Function]
use std::path::PathBuf;

// All setters for members of type `String` will accept `impl Into<String>` // [!code highlight]
#[bon::builder(on(String, into))]                                           // [!code highlight]
fn example(
    name: String,
    description: String,

    // The setter only for this member will accept `impl Into<PathBuf>`    // [!code highlight]
    #[builder(into)]                                                       // [!code highlight]
    path: PathBuf,
) {}

example()
    .name("Bon") // [!code highlight]
    .description("Awesome crate 🐱") // [!code highlight]
    .path("/path/to/your/heart") // [!code highlight]
    .call();
```

```rust [Method]
use std::path::PathBuf;

struct Example;

#[bon::bon]
impl Example {
    // All setters for members of type `String` will accept `impl Into<String>` // [!code highlight]
    #[builder(on(String, into))]                                                // [!code highlight]
    fn example(
        name: String,
        description: String,

        // The setter only for this member will accept `impl Into<PathBuf>`    // [!code highlight]
        #[builder(into)]                                                       // [!code highlight]
        path: PathBuf,
    ) {}
}

Example::example()
    .name("Bon") // [!code highlight]
    .description("Awesome crate 🐱") // [!code highlight]
    .path("/path/to/your/heart") // [!code highlight]
    .call();
```

:::

`Into` conversions don't always make sense, and you should be aware of their downsides as well. The article [Into Conversions In-Depth](../patterns/into-conversions-in-depth) provides recommendations on when it makes sense to use and to avoid `Into` conversions.
