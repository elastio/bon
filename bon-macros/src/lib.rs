#![doc = include_str!("../README.md")]

mod bon;
mod builder;
mod error;
mod map;
mod normalization;
mod set;
mod util;

use proc_macro::TokenStream;

/// Can be placed on top of a free function or an associated method or a struct
/// declaration. Generates a builder for the item beneath it.
///
/// There documentation for this macro is split into two parts:
/// - [Overview page](https://elastio.github.io/bon/docs/guide/overview)
/// - [Attributes reference](https://elastio.github.io/bon/docs/reference/builder)
///
/// # Quick example
///
/// `bon` can turn a function with positional parameters into a function with "named"
/// parameters via a builder. It's as easy as placing the `#[builder]` macro on top of it.
///
/// ```rust ignore
/// use bon::builder;
///
/// #[builder]
/// fn greet(name: &str, age: u32) -> String {
///     format!("Hello {name} with age {age}!")
/// }
///
/// let greeting = greet()
///     .name("Bon")
///     .age(24)
///     .call();
///
/// assert_eq!(greeting, "Hello Bon with age 24!");
/// ```
///
/// See the [overview](https://elastio.github.io/bon/docs/guide/overview) for the
/// rest of the docs about associated methods, structs, and more.
#[proc_macro_attribute]
pub fn builder(params: TokenStream, item: TokenStream) -> TokenStream {
    syn::parse(item.clone())
        .map_err(Into::into)
        .and_then(|item| builder::generate_for_item(params.into(), item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}

/// Companion macro for [`builder`]. You should place it on top of the `impl` block
/// where you want to define methods with the [`builder`] macro.
///
/// It provides the necessary context to the [`builder`] macros on top of the functions
/// inside of the `impl` block. You'll get compile errors without that context.
///
/// For details on this macro including the reason why it's needed see this
/// paragraph in the [overview](https://elastio.github.io/bon/docs/guide/overview#builder-for-an-associated-method).
///
/// # Quick example
///
/// ```rust ignore
/// use bon::bon;
///
/// struct Counter {
///     val: u32,
/// }
///
/// #[bon] // <- this macro is required on the impl block
/// impl Counter {
///     #[builder]
///     fn new(initial: Option<u32>) -> Self {
///         Self {
///             val: initial.unwrap_or_default(),
///         }
///     }
///
///     #[builder]
///     fn increment(&mut self, diff: u32) {
///         self.val += diff;
///     }
/// }
///
/// let mut counter = Counter::builder()
///     .initial(3)
///     .build();
///
/// counter
///     .increment()
///     .diff(3)
///     .call();
///
/// assert_eq!(counter.val, 6);
/// ```
///
/// [`builder`]: macro@builder
#[proc_macro_attribute]
pub fn bon(params: TokenStream, item: TokenStream) -> TokenStream {
    util::parse_attr_macro_input(params, item.clone())
        .and_then(|(opts, item)| bon::generate(opts, item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}

/// Similar to [`maplit::hashmap!`]/[`maplit::btreemap!`] but converts each key and value with
/// [`Into::into()`].
///
/// There are no separate variants for [`HashMap`] and [`BTreeMap`] since the macro works with any
/// type that implements [`FromIterator<(K, V)>`].
///
/// The macro also performs rudimentary uniqueness checking on keys: syntactically equal keys are
/// rejected with a compile error.
///
/// A good example of the use case for this macro is when you want to create a
/// `HashMap<String, String>` where part of the keys or values are hardcoded string literals of type `&str`
/// and the other part is made of dynamic [`String`] values.
///
/// ```rust
/// # use bon_macros as bon;
/// # use std::collections::HashMap;
/// let address_book: HashMap<String, String> = bon::map! {
///     "jd@example.org": "John Doe",
///     format!("{}@{}.{}", "jane.doe", "exmaple", "com"): "Jane Doe",
///     "roger@example.org": format!("Roger {}", "Simpson"),
///     // "jd@example.org": "Jane Doe", // compile error
/// };
/// ```
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`HashMap`]: std::collections::HashMap
/// [`maplit::hashmap!`]: https://docs.rs/maplit/latest/maplit/macro.hashmap.html
/// [`maplit::btreemap!`]: https://docs.rs/maplit/latest/maplit/macro.btreemap.html
#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    let entries = syn::parse_macro_input!(input with util::parse_map_macro_input);

    map::generate(entries)
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

/// Similar to [`maplit::hashset!`]/[`maplit::btreeset!`] but converts each key and value with
/// [`Into::into()`].
///
/// There are no separate variants for [`HashSet`] and [`BTreeSet`] since the macro works with any
/// type that implements [`FromIterator<T>`].
///
/// The macro also performs rudimentary uniqueness checking on keys: syntactically equal elements are
/// rejected with a compile error.
/// let fruit_basket: HashSet<String> = bon::set! {
///         "apples",
///         format!("b{0}n{0}n{0}s", 'a'),
///         format!("or{:x}ng{:x}s", 10, 14),
///         // "apples", // compile error
/// };
///         "-y",
///         format!("output.{output_extension}"),
///     };
///
///     std::process::Command::new("ffmpeg").args(ffmpeg_args).output()?;
///
///     Ok(())
/// }
/// ```
///
/// [`BTreeSet`]: std::collections::BTreeSet
/// [`HashSet`]: std::collections::HashSet
/// [`maplit::hashset!`]: https://docs.rs/maplit/latest/maplit/macro.hashset.html
/// [`maplit::btreeset!`]: https://docs.rs/maplit/latest/maplit/macro.btreeset.html
#[proc_macro]
pub fn set(input: TokenStream) -> TokenStream {
    use syn::punctuated::Punctuated;

    let entries = syn::parse_macro_input!(input with Punctuated::parse_terminated);

    set::generate(entries)
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
