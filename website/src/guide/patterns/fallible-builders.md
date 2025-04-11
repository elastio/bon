# Fallible Builders

With `bon`, you can write a builder that validates its inputs and returns a `Result`. It's possible to do this via the function or associated method syntax. Simply write a constructor function with the `Result` return type and add a `#[builder]` to it.

```rust
use anyhow::Error;
use bon::bon;

pub struct User {
    id: u32,
    name: String,
}

#[bon]
impl User {
    #[builder]
    pub fn new(id: u32, name: String) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Empty name is disallowed"));
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

## Custom Finishing Function

The example above declares the same parameters on the `new` method as there are fields on the `User` struct. Usually, this repetition is fine and even beneficial because it decouples your struct's representation from the building logic allowing both to evolve more independently. For example, this way it's easy to change the type of `id` parameter to a `&str` or some other type, while keeping it as `u32` internally in the struct.

However, if you want to avoid repeating the struct's fields you can override the finishing function of the builder while keeping the builder derived from the struct.

```rust
use anyhow::Error;
use bon::Builder;

#[derive(Builder)]
// Ask `bon` to make the auto-generated finishing function private
// and name it `build_internal()` instead of the default `build()`
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct User {
    id: u32,
    name: String,
}

// Define a custom finishing function as a method on the `UserBuilder`.
// The builder's state must implement the `IsComplete` trait.
// See details about it in the tip below this example.
impl<S: user_builder::IsComplete> UserBuilder<S> {
    pub fn build(self) -> Result<User, Error> {
        // Delegate to `build_internal()` to get the instance of user.
        let user = self.build_internal();

        // Now validate the user or do whatever else you want with it.
        if user.name.is_empty() {
            return Err(anyhow::anyhow!("Empty name is disallowed"));
        }

        Ok(user)
    }
}

let result: Result<User, Error> = User::builder()
    .id(42)
    .name(String::new())
    .build();
```

::: tip

This example uses the `IsComplete` trait which is explained in the ["Custom Methods"](../typestate-api/custom-methods#iscomplete-trait) section.

:::

Note that the signature of the `build()` method is fully under your control. You can make it accept additional parameters, return a different type, etc. The only difference of this approach from the method `new` is that this method accepts the builder as `self` and creates the target object (`User`) before validating the inputs.

This approach suffers from having a state of the object in code where it's constructed but not valid, which is however contained within the custom `build()` method implementation.

## Fallible Setter

It is possible to do validations right when the setter is called. Use `#[builder(with)]` with a fallible closure to achieve that. The following example is an excerpt from that attribute's [API reference](../../reference/builder/member/with), see more details there in the [Fallible Closure](../../reference/builder/member/with#fallible-closure) section.

<!--@include: ../../reference/builder/member/with.md#fallible-closure-example-->

## Future Possibilities

If you have some design ideas for an attributes API to do validations with the builder macros, then feel free to post them in [this Github issue](https://github.com/elastio/bon/issues/34).
