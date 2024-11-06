# Antipatterns

While [Patterns](./patterns) teach you how to use `bon`, here we'll discuss how **not to use** `bon`.

## Generic Types in Optional Members

Generic type parameters, `impl Trait` or const generics used exclusively in optional members break type inference. This is mostly relevant to `fn`-based builders.

This problem becomes visible when you skip setting an optional member.

### 🔴 Bad

```rust
#[bon::builder]
fn bad<T: Into<String>>(x1: Option<T>) {
    let x1 = x1.map(Into::into);
    // ...
}

// This compiles
bad().x1("&str").call();

// This doesn't
bad().call(); // error[E0283]: type annotations needed // [!code error]
```

The compilation error here is:

```rust ignore
bad().call();
^^^ cannot infer type of the type parameter `T` declared on the function `bad`
```

A similar error would be generated if we used `Option<impl Into<String>>`, although the error would reference a generic parameter [auto-generated](./typestate-api/builders-type-signature#other-generic-parameters) for the function by the builder macro. For simplicity, we'll use a named generic parameter throughout the examples.

The caller of your builder would need to work around this problem by specifying the type `T` explicitly via turbofish:

```rust ignore
// Both String or `&str` would work as a type hint
bad::<String>().call();
```

This is inconvenient, don't do this.

### ✅ Good

Instead, make the member's type non-generic. Move generics to the setter methods' signature. It's easier to understand what it means with an example.

For the case above, the good solution will be [`#[builder(into)]`](../reference/builder/member/into).

```rust
#[bon::builder]
fn good(#[builder(into)] x1: Option<String>) {
    // ...
}

good().x1("&str").call();
good().call();
```

How `#[builder(into)]` is different from `Option<T>` (`T: Into`)?
Let's compare the generated code between them (simplified). Switch between the tabs below:

::: code-group

```rust [#[builder(into)]]
fn good() -> GoodBuilder { /**/ }

impl<S: State> GoodBuilder<S> {
    fn x1(self, value: impl Into<String>) -> GoodBuilder<SetX1<S>> {
        GoodBuilder { /* other fields */, __x1: value.into() }
 }
}
```

<!--
Prettier tries to replace &lt; with a <, but it is intentionally there, because
otherwise, Vue thinks as if we are trying to write an HTML tag e.g. <T></T>
-->
<!-- prettier-ignore -->
```rust [Option&lt;T>]
fn bad<T>() -> BadBuilder<T> { /**/ }

impl<T: Into<String>, S: State> BadBuilder<T, S> {
    fn x1(self, value: T) -> BadBuilder<T, SetX1<S>> {
        BadBuilder { /* other fields */, __x1: value }
 }
}
```

:::

Notice how in the good example of `#[builder(into)]` the starting function `good()` doesn't declare any generic parameters, while in the `Option<T>` example `bad()` does have a generic parameter `T`.

Also, in the case of `#[builder(into)]` the call to `.into()` happens inside of the setter method itself (early). In the case of `Option<T>`, the call to `.into()` is deferred to the finishing function.

This is also visible when you compare the original functions again. Notice how in the `Bad` example, we have to manually call `x1.map(Into::into)`, while in `Good` we already have a concrete `Option<String>` type:

::: code-group

```rust [Good]
#[bon::builder]
fn good(#[builder(into)] x1: Option<String>) {
    // ...
}
```

```rust [Bad]
#[bon::builder]
fn bad<T: Into<String>>(x1: Option<T>) {
    let x1 = x1.map(Into::into);
    // ...
}
```

:::

So, the builder has to carry the generic parameter `T` in its type for you to be able to use that type in your function body merely to invoke an `Into::into` on a parameter. Instead, strive to do such conversions in setters.

If you are doing a conversion with something other than `Into`, then use [`#[builder(with)]`](../reference/builder/member/with) to apply the same technique for getting rid of generic parameters _early_.
