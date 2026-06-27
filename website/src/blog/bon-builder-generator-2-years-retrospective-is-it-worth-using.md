---
title: Bon Builder Generator. 2 Years Retrospective. Is It Worth Using? 🔍
date: 2026-06-27
author: Veetaha
outline: deep
---

Hi! It's been approximately 2 years since I released the crate `bon`. I haven't spoken about it for these years, so here I am. For those who aren't familiar with `bon`, here is the code snippet that demonstrates ~30% of its soul.

```rust
#[bon::builder]
fn greet(name: &str, level: Option<u32>) -> String {
    format!("Hello, {name}! Your level is {}", level.unwrap_or(0))
}

let greeting = greet()
    .name("Bon")
    .level(24) // <- setting `level` is optional, we could omit it
    .call();

assert_eq!(greeting, "Hello, Bon! Your level is 24");
```

The _soul_, the _essensce_, the _paradigm_. Generating type/panic-safe builders for functions, structs and methods based on the typestate design.

## TLDR

`bon` is alive and well. If you are using it, if it solves the problems that are important for you, and you like it, then do not hesitate to keep using it. I am happy it's useful for you! The crate got quite some traffic; it's very stable, feature-rich, and I still maintain it.

Now let's talk about `bon`'s lore and the question in the title in more detail. It's going to be quite personal. So sit back, and let me tell you a bit of my Rust open source evolution story.

## Bon's Lore

It's been 2 years since I released `bon` and shared a bunch of blog posts about it. I am a little embarrassed by how clickbaity and immature the writing in [my posts](../blog) was back then. But, eh.. we all grow up, and I wanna apologise for my past writing style 😅.

In fact, the [initial commit](https://github.com/elastio/bon/commit/4943f1fbb5e35567a3394a1ece28af3cf453ab49) in `bon` is going to be exactly 2 years old in a couple of days (29th of June 2024).

::: tip Yeah

It could've been called `buildy`, but the name was already taken on crates.io, and I went for a shorter name.

:::

## Initial Design

At first, I was hyper-inspired and motivated to work on `bon`. I knew exactly the design I wanted. I wanted to fix all the annoyances I had with existing builder generator crates at that time. The comparison table on the [alternatives](../guide/alternatives) page is a good breakdown.

I went through the rapid initial feedback stage and a couple of quick major versions. I wanted `bon` to be perfect, so I rushed its dev lifecycle, completely dedicating all my free time to it. It was a blast.

I remember walking circles in my room, long staring through the window, brainstorming the design of [`bon` v3](./bon-v3-release). I went like this for a week or two. I was lying in my bed, being frustrated with the language limitations and juggling between different design tradeoffs in my head. I collected my design thoughts in a small markdown file and planned `bon`'s current typestate design with the features evolution for the next several minor versions.

I think I did manage to come up with something great. Those several weeks of brainstorming paid off. The design has been coined, and it's been stable for a couple of years now. The crate is very feature rich, well-documented, has experienced little-to-no bugs and very few feature requests. It's very flexible and doesn't need a lot of maintenance. I'd even say it's feature-complete.

## Early Activity

Bon grew with new features and went through a healthy evolution with its own challenges and successes.

The crate is very general-purpose. People come to it with very different backgrounds and use cases. I've been trying to be welcoming of new ideas and feature requests. Maybe even too welcoming in some cases. Not all features really fit the crate's design, or they overcomplicate it. Having to say "No" or suggest a workaround is the most difficult part of the crate's support. It's a bit of a social challenge. Unfortunately, I can't satisfy everyone, and every design has its limitations. Flexibility isn't endless, and simplicity is very alluring.

I've been using `bon` internally at work. It looked quite nice on API clients with lots of optional parameters. It also got spread to the constructor methods deeper in the business logic. I remember I was extremely happy with it as it removed the necessity to pass `None` for optional parameters and reduced the number of changes we had to do when adding new fields/parameters to structs/functions.

I think that _the ability_ not to be forced to pass `None` for `Option<T>` fields/parameters is a very nice feature. I would really like to see it in the language itself. I think the nightly Rust language feature [Default field values](https://github.com/rust-lang/rust/issues/132162) is an awesome start.

Only a _start_? Yeah, well.. It's limited to just struct literals and `const` expressions for defaults. It doesn't cover all the compatibility guarantees that `bon` [offers](../guide/basics/compatibility).

Here is one of the examples of what it lacks in comparison to `bon`. Imagine we had this code.

```rust
// Library code
struct Example {
    x: u32,
}

// Consumer code
let example = Example { x: 42 };
```

Now we want to make the field `x` optional. This shouldn't be a breaking change, should it? Well, it is, and the default field values Rust feature doesn't help with it.

```rust ignore
// Library code
struct Example {
    x: Option<u32> = None,
}

// Consumer code. We are now forced to wrap the value with `Some(...)`
let example = Example { x: Some(42) };
```

Bon was designed to [prevent this specific breaking change in your code](../guide/basics/compatibility#making-a-required-member-optional). Unfortunately, I don't know of any Rust language initiative that may cleanly solve this problem yet.

Frankly, I'd be happy if the language solved most of the problems `bon` solves, so that it would become fully redundant. However, the language doesn't solve them today, and it won't solve them all, definitely not soon. But why? Why do we have to resort to `bon` for such fundamental language features?

Why wouldn't Rust solve my problem? Is it not important enough? Isn't my opinion important?..

_Wow, hey, wait a second. Haven't I seen this line of thought before?_

I've been triaging issues in `bon` and deciding what's important and what's not. I had to say "No" to various feature requests, or even just leave them open with no clear answer. Just like Rust language team decided to keep the problem described above unsolved. Am I or Rust contributors the devil?? _Obviously, no._

## 2 Years Later

The last 2 years changed me and my personal opinion on `bon` and the Rust language itself.

::: tip No, I'm not talking about AI.

It's my work experience and experience working with other developers at my day job.

:::

I haven't spoken about the change of my opinion publicly. However, I've noticed a little spike of stars on the `bon` repo today, and it prompted me to share what's been in my head in this blog post.

I knew exactly the problems `bon` had to solve when starting it, but the fundamental question I didn't ask before starting `bon` was "Are these problems even worth solving?" Are they really annoying that much? Does anyone really care that you have to explicitly pass `None` for every `Option<T>` field/parameter? And most importantly, is it worth adding a proc macro dependency to your project for all this?

You are adding a dependency to your code that increases your [compile times](../guide/benchmarks/compilation), and adds more complexity to your codebase. But... It's a tradeoff. For someone who's using `bon` to protect their public crate API from breaking changes, it may be worth it. For someone who's using `bon` in their private code, in the code they control, I personally think it may not be worth it.

If you are not writing library code and you don't care about protecting your API surface from breaking changes (which is the main feature of `bon` as I see it now), **consider using just plain old boring struct literals instead**. Yes, bon would save you several lines of code at the call site with not having to pass `None` for default fields/parameters explicitly, but I personally don't think it's that big of a deal any more, at least in the code I'm working with today.

::: tip Ugh

I feel really awkward removing the usage of `bon` from my own consumer codebase. For all the effort and time I put into it, and yet I am the one who doesn't use it anymore 🗿.

:::

Does it mean I regret making `bon`, or that I'm deprecating it? Hell no! Yeah, I do feel awkward, but I know `bon` is still useful for many library maintainers. People are using it to keep their APIs stable, and it's almost at 40M downloads today.

Bon is here to stay, and I will keep maintaining it for the foreseeable future. As I mentioned above, it's not even that much of a maintenance work.

## My Cold-blooded Take

I encourage you to reconsider your usage of `bon` in your internal code repositories. I know `bon` looks nice on paper and solves some minor annoyances, but look at it this way. Struct literals are easy to use, extremely cheap to compile, they have excellent support in Rust Analyzer and don't require you to learn `bon`. I love that Rust Analyzer can autofill struct literal fields, providing me with an exhaustive "form" that I need to fill. Builders don't have that dev experience; you have to write the method chain call manually.

Struct literals also provide excellent compile error messages. While bon does aim to provide good error messages, they are definitely a downgrade compared to struct literals:

```log
error[E0277]: the member `bon::__::Unset<field_name>` was not set, but this method requires it to be set
  --> tests/integration/ui/compile_fail/diagnostic_on_unimplemented.rs:14:37
   |
14 |     let _ = Example::builder().x(1).build();
   |                                     ^^^^^ the member `bon::__::Unset<field_name>` was not set, but this method requires it to be set
   |
   = help: the trait `bon::__::IsSet` is not implemented for `bon::__::Unset<field_name>`
```

I can probably list a bunch more disadvantages of using `bon`, but I'll stop here.

That's pretty much my stance today. If you've been using `bon`, I wanna hear your opinion. What do you think about it, especially after I challenged its usage in private code. Maybe not that many people have been using it in app code? I am not sure, I don't have any statistics on that. So I'm curious to hear about your experience and opinion.

## Summary

Hey, you made it till the end? Frankly, I have no idea if it was that interesting to read, but at least it's useful as a mental checkpoint snapshot for my future self. I haven't been active in the Rust community for a while, even though I've been daily driving Rust at work. My interests shifted over the last couple of years. But it feels exciting to do a little comeback, even though it's just for this single awkward blog post.

Yeah, I said it was gonna be personal! You got what was promised :D
