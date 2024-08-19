#![doc = include_str!("../README.md")]

mod bon;
mod builder;
mod error;
mod map;
mod normalization;
mod set;
mod util;

use proc_macro::TokenStream;
use syn::parse::Parser;

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
    let meta = util::ide::parse_comma_separated_meta
        .parse2(params.clone().into())
        .unwrap_or_default();

    let completions = util::ide::generate_completions(meta);

    let main = syn::parse(item.clone())
        .map_err(Into::into)
        .and_then(|item| builder::generate_for_item(params.into(), item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()));

    quote::quote! {
        #completions
        #main
    }
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

/// Creates any map-like collection that implements [`FromIterator<(K, V)>`].
///
/// It automatically converts each key and value to the target type using [`Into`].
/// This way you can write a map of `String`s without the need to call `.to_owned()`
/// or `.to_string()` on every string literal:
///
/// ```rust
/// # use bon_macros as bon;
/// # use std::collections::HashMap;
/// let map: HashMap<String, String> = bon::map! {
///     "key1": "value1",
///     format!("key{}", 2): "value2",
///     "key3": format!("value{}", 3),
/// };
/// ```
///
/// There is no separate variant for [`BTreeMap`] and [`HashMap`]. Instead, you
/// should annotate the return type of this macro with the desired type or make
/// sure the compiler can infer the collection type from other context.
///
/// # Compile errors
///
/// The macro conservatively rejects duplicate keys in the map with a compile error.
/// This check works for very simple expressions that involve only literal values.
///
/// ```rust compile_fail
/// # use bon_macros as bon;
/// # use std::collections::HashMap;
/// let map: HashMap<String, String> = bon::map! {
///     "key1": "value1",
///     "key2": "value2"
///     "key1": "value3", // compile error: `duplicate key in the map`
/// };
/// ```
///
/// [`FromIterator<(K, V)>`]: https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html
/// [`Into`]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
/// [`BTreeMap`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
/// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    let entries = syn::parse_macro_input!(input with map::parse_macro_input);

    map::generate(entries).into()
}

/// Creates any set-like collection that implements [`FromIterator<T>`].
///
/// It automatically converts each value to the target type using [`Into`].
/// This way you can write a set of `String`s without the need to call `.to_owned()`
/// or `.to_string()` on every string literal:
///
/// ```rust
/// # use bon_macros as bon;
/// # use std::collections::HashSet;
/// let set: HashSet<String> = bon::set![
///     "value1",
///     format!("value{}", 2),
///     "value3",
/// ];
/// ```
///
/// There is no separate variant for [`BTreeSet`] and [`HashSet`]. Instead, you
/// should annotate the return type of this macro with the desired type or make
/// sure the compiler can infer the collection type from other context.
///
/// # Compile errors
///
/// The macro conservatively rejects duplicate values in the set with a compile error.
/// This check works for very simple expressions that involve only literal values.
///
/// ```rust compile_fail
/// # use bon_macros as bon;
/// # use std::collections::HashSet;
/// let set: HashSet<String> = bon::set![
///     "value1",
///     "value2"
///     "value1", // compile error: `duplicate value in the set`
/// ];
/// ```
///
/// [`FromIterator<T>`]: https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html
/// [`Into`]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
/// [`BTreeSet`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html
/// [`HashSet`]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
#[proc_macro]
pub fn set(input: TokenStream) -> TokenStream {
    use syn::punctuated::Punctuated;

    let entries = syn::parse_macro_input!(input with Punctuated::parse_terminated);

    set::generate(entries).into()
}
