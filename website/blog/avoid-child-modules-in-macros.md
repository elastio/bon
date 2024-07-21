---
title: 'Avoid child modules in macros'
date: 2024-07-21
author: Veetaha
outline: deep
---

## Context

I've been working on a crate [`bon`](/) that generates builders for structs and functions. I designed it based on lessons learned from using [`buildstructor`](https://docs.rs/buildstructor/latest/buildstructor/).

When writing some examples for the [alternatives](../docs/guide/alternatives#special-setter-methods-for-collections) section that compares `bon` with `buildstructor` I stumbled with a doc test build failure for the following really simple example of code:

```rust compile_fail
#[derive(buildstructor::Builder)]
struct User {
    name: String
}

User::builder().name("Foo").build();
```

which produces the following compile error:

```log
cannot find type `User` in this scope
 --> doc-test/path/here.rs
  |
2 | struct User {
  |        ^^^^ not found in this scope
```

And then I started digging, which lead me to a discovery of how using child modules can break doc tests and some other things...


## So what's the problem with child modules in macros?

To explain the problem, first, let's build an understanding of how name resolution works for "local" items.

### Name resolution for local items

It's possible to define an item such as a `struct`, `impl` block or `fn` inside of any block expression in Rust. For example, this code defines a "local" anonymous struct inside of a function block:

```rust
fn example() {
    struct User;

    let user = User;
}
```

Here, the `User` struct is only accessible inside of the scope of the function block. We can't reference it outside of this function:

```rust ignore
fn example() {
    struct User;
}

// error: cannot find type `User` in this scope // [!code error]
type Foo = User;                                // [!code error]

mod child_module {
    // error: unresolved import `super::User`; no `User` in the root // [!code error]
    use super::User;                                                 // [!code error]
}
```

This doesn't work because, logically, there should be something in the path that says `{fn example()}::User`. However, there is no syntax in Rust to express the `{fn example()}` scope.

But what does `std::any::type_name()` return for that `User` struct then? Let's figure this out:

```rust
fn example() -> &'static str {
    struct User;

    std::any::type_name::<User>()
}

fn main() {
    println!("{}", example());
}
```

This outputs the following:

```log
crate_name::example::User
```

So, the function name becomes part of the path as it was just a simple module. However, this isn't true, or at least this behavior isn't exposed in the language. If we try to reference the `User` from the surrounding scope using that syntax, we are still out of luck:

```rust ignore
fn example() {
    struct User;
}

type Foo = example::User; // [!code error]
```
This generates a compile error:

```log
error[E0433]: failed to resolve: function `example` is not a crate or module
 --> path/to/code.rs
  |
6 | type Foo = example::User;
  |            ^^^^^^^ function `example` is not a crate or module
```

So there is just no way to refer to the `User` struct outside of the function scope, right?... Wrong 🐱! There is a way to do this, but it's so complicated that let's just assume we can't do that in production code.

If your are curious, first, try to solve this yourself:

```rust
fn example() {
    struct User;
}

type Foo = /* how can we get the `User` type from `example` function here? */
```

and then take a look at the solution below:

::: details Solution for referring to a local item outside of the function body.

The idea is to implement a trait for the local type and then use that trait in the outside scope to get the local type.

```rust
trait TypeChannel {
    type Type;
}

struct GetUserType;

fn example() {
    struct User;

    // We can implement a trait from the surroudning scope
    // that uses the local item in it.
    // Local item's visibility is still full-module scoped.
    impl TypeChannel for GetUserType {
        type Type = User;
    }
}

type Foo = <GetUserType as TypeChannel>::Type;
```

Now this compiles... but well, I'd rather burn this code with fire 🔥

:::

---

Now, let's see what happens if we define the `mod child_module` inside of the function block.

```rust ignore
fn example() {
    struct User;

    mod child_module {
        use super::User; // [!code error]
    }
}
```

Does this compile? Surely, it should compile, because the child module becomes child of the anonymous function scope, so it should have access to symbols defined in the function, right?... Wrong 🐱!

It still doesn't compile with the error ``unresolved import `super::User`; no `User` in the root``. This is because `super` doesn't refer to the parent function scope, instead it refers to the top-level module (called `root` by the compiler in the error message) that defines the `example()` function. For example, this code compiles:

```rust ignore
struct TopLevelStruct;

fn example() {
    struct User;

    mod child_module {
        use super::TopLevelStruct; // [!code highlight]
    }
}
```

As you can see we took `TopLevelStruct` from `super`, so it means `super` refers to the surrounding module of the `example` function, and we already know ~~we can't~~ how hacky it is to access the symbols defined inside of that `example` function from within the surrounding module.


So.. this brings us to the following dilemma.

### How does this affect macros?

Macros generate code, and that code must not always be fully accessible to the scope where the macro was invoked. For example, macro that generates a builder struct would like to restrict access to the private fields of the generated builder struct for the surrounding module.

I'll use `bon`'s macros syntax to showcase this.

```rust
use bon::builder;

#[builder]
struct User {
    name: String,
}
```

Let's see how the generated code for this example may look like (very simplified).

```rust
struct User {
    name: String,
}

#[derive(Default)]
struct UserBuilder {
    name: Option<String>,
}

// impl blocks for `UserBuilder` that defines setters...

fn example() {
    let builder = UserBuilder::default();

    // oops, we can access builder's internal fields here // [!code highlight]
    let _ = builder.name;                                 // [!code highlight]
}
```

The problem with this approach is that `UserBuilder` is defined in the same module scope as the `User` struct. It means all fields of `UserBuilder` are accessible by this module. This is how visibility of private fields works in Rust - the entire module where the struct is defined has access to the private fields of that struct.

The way to avoid this problem is to define the builder in a nested child module, such that private fields of the builder struct will be accessible only within that child module.

```rust
struct User {
    name: String,
}

use user_builder::UserBuilder;

mod user_builder { // [!code highlight]
    use super::*;

    pub(super) struct UserBuilder {
        name: Option<String>,
    }
}

fn example() {
    let builder = UserBuilder::default();

    // Nope, we can't access builder's fields now. // [!code highlight]
    // let _ = builder.name;                       // [!code highlight]
}
```

So... problem solved, right?... Wrong 🐱!

Now imagine our builder macro is invoked for a struct defined inside of a local function scope:

```rust
use bon::builder;

fn example() {
    struct Password(String);

    #[builder]
    struct User {
        password: Password,
    }
}
```

If `#[builder]` creates a child module, then we have a problem. Let's see the generated code:

```rust
fn example() {
    struct Password(String);

    struct User {
        password: Password,
    }

    mod user_builder {                  // [!code highlight]
        use super::*;                   // [!code highlight]
                                        // [!code highlight]
        pub(super) struct UserBuilder { // [!code highlight]
            password: Option<Password>, // [!code error]
        }                               // [!code highlight]
    }                                   // [!code highlight]
}
```

This doesn't compile with the error:

```log
password: Option<Password>,
                 ^^^^^^^^ not found in this scope
```

Why is that? As we discussed higher child modules defined inside of function blocks can't access symbols from the function's scope. The `use super::*` imports items from the surrounding top-level module instead of the function scope.

It means, that if we want to support local items in our macro we just can't use a child module if code inside of that child module needs to reference types (or any items) from the surrounding scope.

The core problem is the conflict:
- We want to make builder's fields private, so we need to define the builder struct inside of a child module.
- We want to reference types from the surrounding scope in the builder's fields, including local items, so we can't define the builder struct inside of the child module.

This is the problem that I found in `buildstructor`. The only way to solve this is to do a compromise, which I did when implementing `#[bon::builder]`. The compromise is not to use a child module, obfuscate the private fields of the builder struct with leading `__` and `#[doc(hidden)]` attributes to make it hard for the user to access them (even though not physically impossible).

But then... Defining types inside of functions is rather a niche use case. How do child modules in macro-generated code break the doc test mentioned in beginning of this article?

### How does this break doc tests?

Doc tests are usually code snippets that run some code defined on the top level. They don't typically contain an explicit `main()` function.

For example, a doc test like this:

```rust
let foo = 1 + 1;
assert_eq!(foo, 2);
```

is [implicitly wrapped by `rustdoc`](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#pre-processing-examples) in a `main()` function like this:

```rust
fn main() {
    let foo = 1 + 1;
    assert_eq!(foo, 2);
}
```

So... If we write a code example in a doc comment with a macro that generates a child module, the doc test will probably not compile. This is what happened in the original doc test featuring `buildstructor`.

Let's bring it up again:

```rust compile_fail
#[derive(buildstructor::Builder)]
struct User {
    name: String
}

User::builder().name("Foo").build();
```

When preprocessing the doc test `rustdoc` wraps this code in `main()`:

```rust compile_fail
fn main() {
    #[derive(buildstructor::Builder)]
    struct User {
        name: String
    }

    User::builder().name("Foo").build();
}
```

Then `buildstructor` generates a child module, that refers to `User` (next code is simplified):

```rust compile_fail
fn main() {
    struct User {
        name: String
    }

    mod user_builder {
        use super::*;

        struct UserBuilder {
            name: Option<String>
        }

        impl UserBuilder {
            // `User` is inaccessible here // [!code error]
            fn build(self) -> User {       // [!code error]
                /* */
            }
        }
    }
}
```

## Summary

Does this mean generating child modules for privacy in macros is generally a bad idea? I'd say *mostly*, the main thing is not to reference items from the surrounding scope in the child module. For example, if you need to add `use super::*` in your macro-generated code, then this is already a bad call. You should think of local items and doc test when you do this.
