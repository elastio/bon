# `Into` conversions

If you have members of type `String`, or `PathBuf`, and you need to set them to a hard-coded string literal, then you have to write `.to_owned()` or `.to_string()` or `.into()`.

```rust
use bon::Builder;
use std::path::PathBuf;

#[derive(Builder)]       // [!code focus]
struct Project {         // [!code focus]
    name: String,        // [!code focus]
    description: String, // [!code focus]
    path: PathBuf,       // [!code focus]
}                        // [!code focus]

Project::builder()
    .name("Bon".to_owned())                      // [!code focus]
    .description("Awesome crate 🐱".to_string()) // [!code focus]
    .path("/path/to/bon".into())                 // [!code focus]
    .build();
```

However, you can ask `bon` to generate setters that accept `impl Into<T>` to remove the need for manual conversion.

This can be configured with [`#[builder(into)]`](../reference/builder/member/into) for a single member or with [`#[builder(on({type}, into))]`](../reference/builder/top-level/on) for many members at once.

```rust
use bon::Builder;
use std::path::PathBuf;

// All setters for members of type `String` will accept `impl Into<String>` // [!code highlight]
#[derive(Builder)]                                                          // [!code highlight]
#[builder(on(String, into))]                                                // [!code highlight]
struct Project {
    name: String,
    description: String,

    // The setter only for this member will accept `impl Into<PathBuf>`    // [!code highlight]
    #[builder(into)]                                                       // [!code highlight]
    path: PathBuf,
}

Project::builder()
    .name("Bon") // [!code highlight]
    .description("Awesome crate 🐱") // [!code highlight]
    .path("/path/to/your/heart") // [!code highlight]
    .build();
```

`Into` conversions don't always make sense, and you should be aware of their downsides as well. The article ["Into Conversions In-Depth"](./into-conversions-in-depth) provides recommendations on when it makes sense to use and to avoid `Into` conversions.
