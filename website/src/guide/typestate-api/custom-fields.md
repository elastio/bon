# Custom Fields

On this page, you'll learn how to add custom fields to the builder type 🌾.

This is useful if you'd like to have a completely custom state for [custom setters](./custom-methods) in the builder.

To understand how it works, we'll create a builder API _similar_ to [`std::process::Command`](https://doc.rust-lang.org/stable/std/process/struct.Command.html) where you have a couple of methods [`arg`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.arg) and [`args`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.args), that push values into an internal arguments `Vec`.

We'll use the [`#[builder(field)]`](../../reference/builder/member/field) attribute to define a custom field, that will be accessible in our `arg/args` custom setters. The value of this field will be moved into the resulting struct or function from which the builder was generated.

::: tip

See [Custom Methods](./custom-methods) for details on how to write an impl block with additional methods for the builder if you haven't already.

:::

```rust
use bon::Builder;

#[derive(Builder)]
struct Command {
    // Define a private field on the builder without setters. // [!code highlight]
    // It's initialized with `Default::default()` at the start // [!code highlight]
    #[builder(field)] // [!code highlight]
    args: Vec<String>,

    #[builder(into)]
    name: String,
}

// This is the API we'd like to have // [!code highlight]
let cmd = Command::builder()
    .name("ls")
    .arg("-l")
    .args(["foo", "bar"])
    .build();

assert_eq!(cmd.name, "ls");
assert_eq!(cmd.args, ["-l", "foo", "bar"]);

// Now define custom `arg/args` methods on the builder itself. // [!code highlight]
impl<S: command_builder::State> CommandBuilder<S> {
    fn arg(mut self, arg: impl Into<String>) -> Self {
        // We have access to `self.args` private 🔒 field on `CommandBuilder`! // [!code highlight]
        self.args.push(arg.into()); // [!code highlight]
        self
    }

    fn args(mut self, args: impl IntoIterator<Item: Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }
}
```

And that's it! This way you can extend `bon`'s builders with almost any state and behavior, that you want.

You can also specify a custom initial value with `#[builder(field = expr)]`. That `expr` can refer to other members and fields defined higher. See the [evaluation context reference](../../reference/builder/member/field#evaluation-context) for details.
