# Fallible Builders

With `bon`, you can write a builder that validates its inputs and returns a `Result`. It's possible to do this via the function or associated method syntax. Simply write a constructor function with the `Result` return type and add a `#[builder]` to it.

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

With this approach, the finishing function of the generated builder returns a `Result`. Thus, validations are deferred until you invoke the finishing `build()` or `call()`.

## Fallible Setter

You can do validations earlier instead, right when the setter is called. Use `#[builder(with)]` with a fallible closure to achieve that. The following example is an excerpt from that attribute's [API reference](../../reference/builder/member/with), see more details there in the [Fallible Closure](../../reference/builder/member/with#fallible-closure) section.

<!--@include: ../../reference/builder/member/with.md#fallible-closure-example-->

## None Of This Works. Help!

This is very, **very**(!) unlikely but if you have an elaborate use case where none of the options above are flexible enough, then your last resort is writing a [custom method](../typestate-api/custom-methods) on the builder. You'll need to study the builder's [Typestate API](../typestate-api) to be able to do that. Don't worry, it's rather simple, and you'll gain a lot of power at the end of the day 🐱.

## Future Possibilities

If you have some design ideas for an attributes API to do validations with the builder macros, then feel free to post them in [this Github issue](https://github.com/elastio/bon/issues/34).
