# Fallible Builders

With `bon`, you can write a builder that validates its inputs and returns a `Result`. It's possible to do this only via the function or associated method syntax.

If you need to build a struct and do some validation, you won't be able to use the `#[derive(Builder)]` annotation on the struct for that. You'll *have to* define your logic in the associated constructor method (e.g. `new`).

**Example:**

```rust
use anyhow::Error;
use bon::bon;

struct User {
    id: u32,
    name: String,
}

#[bon]
impl User {
    #[builder]
    fn new(id: u32, name: String) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Empty name is disallowed (user id: {id})"));
        }

        Ok(Self { id, name })
    }
}

// The `build()` method returns a `Result`
let result = User::builder()
    .id(42)
    .name(String::new())
    .build();

if let Err(error) = result {
    // Handle the error
}
```

If you have a use case for some convenience attributes to do automatic validations using the `#[derive(Builder)]` macro with the struct syntax, then add a 👍 reaction to [this Github issue](https://github.com/elastio/bon/issues/34).
