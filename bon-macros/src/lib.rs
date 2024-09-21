#![doc = include_str!("../README.md")]
#![allow(
    clippy::redundant_pub_crate,
    clippy::wildcard_imports,
    clippy::map_unwrap_or,
    clippy::items_after_statements,
    clippy::missing_const_for_fn,
    clippy::option_option,
    clippy::option_if_let_else,
    clippy::enum_glob_use,
    clippy::too_many_lines
)]

mod bon;
mod builder;
mod collections;
mod error;
mod normalization;
mod util;

use proc_macro::TokenStream;
use quote::ToTokens;

/// Generates a builder for the function or method it's placed on.
///
/// ## Quick examples
///
/// You can turn a function with positional parameters into a function with
/// named parameters just by placing the `#[builder]` attribute on top of it.
///
/// ```rust ignore
/// use bon::builder;
///
/// #[builder]
/// fn greet(name: &str, level: Option<u32>) -> String {
///     let level = level.unwrap_or(0);
///
///     format!("Hello {name}! Your level is {level}")
/// }
///
/// let greeting = greet()
///     .name("Bon")
///     .level(24) // <- setting `level` is optional, we could omit it
///     .call();
///
/// assert_eq!(greeting, "Hello Bon! Your level is 24");
/// ```
///
/// You can also use the `#[builder]` attribute with associated methods:
///
/// ```rust ignore
/// use bon::bon;
///
/// struct User {
///     id: u32,
///     name: String,
/// }
///
/// #[bon] // <- this attribute is required on impl blocks that contain `#[builder]`
/// impl User {
///     #[builder]
///     fn new(id: u32, name: String) -> Self {
///         Self { id, name }
///     }
///
///     #[builder]
///     fn greet(&self, target: &str, level: Option<&str>) -> String {
///         let level = level.unwrap_or("INFO");
///         let name = &self.name;
///
///         format!("[{level}] {name} says hello to {target}")
///     }
/// }
///
/// // The method named `new` generates `builder()/build()` methods
/// let user = User::builder()
///     .id(1)
///     .name("Bon".to_owned())
///     .build();
///
/// // All other methods generate `method_name()/call()` methods
/// let greeting = user
///     .greet()
///     .target("the world")
///     // `level` is optional, we can omit it here
///     .call();
///
/// assert_eq!(user.id, 1);
/// assert_eq!(user.name, "Bon");
/// assert_eq!(greeting, "[INFO] Bon says hello to the world");
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// See the full documentation for more details:
/// - [Guide](https://elastio.github.io/bon/guide/overview)
/// - [Attributes reference](https://elastio.github.io/bon/reference/builder)
#[proc_macro_attribute]
pub fn builder(params: TokenStream, item: TokenStream) -> TokenStream {
    builder::generate_from_attr(params.into(), item.into()).into()
}

/// Derives a builder for the struct it's placed on.
///
/// ## Quick example
///
/// Add a `#[derive(Builder)]` attribute to your struct to generate a `builder()` method for it.
///
/// ```rust ignore
/// use bon::{bon, builder, Builder};
///
/// #[derive(Builder)]
/// struct User {
///     name: String,
///     is_admin: bool,
///     level: Option<u32>,
/// }
///
/// let user = User::builder()
///     .name("Bon".to_owned())
///     // `level` is optional, we could omit it here
///     .level(24)
///     // call setters in any order
///     .is_admin(true)
///     .build();
///
/// assert_eq!(user.name, "Bon");
/// assert_eq!(user.level, Some(24));
/// assert!(user.is_admin);
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// See the full documentation for more details:
/// - [Guide](https://elastio.github.io/bon/guide/overview)
/// - [Attributes reference](https://elastio.github.io/bon/reference/builder)
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    builder::generate_from_derive(item.into()).into()
}

/// Companion macro for [`builder`]. You should place it on top of the `impl` block
/// where you want to define methods with the [`builder`] macro.
///
/// It provides the necessary context to the [`builder`] macros on top of the functions
/// inside of the `impl` block. You'll get compile errors without that context.
///
/// # Quick example
///
/// ```rust ignore
/// use bon::bon;
///
/// struct User {
///     id: u32,
///     name: String,
/// }
///
/// #[bon] // <- this attribute is required on impl blocks that contain `#[builder]`
/// impl User {
///     #[builder]
///     fn new(id: u32, name: String) -> Self {
///         Self { id, name }
///     }
///
///     #[builder]
///     fn greet(&self, target: &str, level: Option<&str>) -> String {
///         let level = level.unwrap_or("INFO");
///         let name = &self.name;
///
///         format!("[{level}] {name} says hello to {target}")
///     }
/// }
///
/// // The method named `new` generates `builder()/build()` methods
/// let user = User::builder()
///     .id(1)
///     .name("Bon".to_owned())
///     .build();
///
/// // All other methods generate `method_name()/call()` methods
/// let greeting = user
///     .greet()
///     .target("the world")
///     // `level` is optional, we can omit it here
///     .call();
///
/// assert_eq!(user.id, 1);
/// assert_eq!(user.name, "Bon");
/// assert_eq!(greeting, "[INFO] Bon says hello to the world");
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// For details on this macro including the reason why it's needed see
/// [this paragraph in the overview](https://elastio.github.io/bon/guide/overview#builder-for-an-associated-method).
///
/// [`builder`]: macro@builder
#[proc_macro_attribute]
pub fn bon(params: TokenStream, item: TokenStream) -> TokenStream {
    bon::generate(params.into(), item.into()).into()
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
    let entries = syn::parse_macro_input!(input with collections::map::parse_macro_input);

    collections::map::generate(entries).into()
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

    collections::set::generate(entries).into()
}

/// Private proc macro! Don't use it directly, it's an implementation detail.
///
/// This macro takes a function and overrides its return type with the provided one.
/// It's used in combination with `cfg_attr` to conditionally change the return type
/// of a function based on the `cfg(doc)` value.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn __return_type(ret_ty: TokenStream, item: TokenStream) -> TokenStream {
    let mut func: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(func) => func,
        Err(err) => return error::error_into_token_stream(err.into(), item.into()).into(),
    };

    let ret_ty = proc_macro2::TokenStream::from(ret_ty);

    func.sig.output = syn::parse_quote!(-> #ret_ty);

    func.into_token_stream().into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __gen_tuple_traits(total: TokenStream) -> TokenStream {
    use crate::util::prelude::*;
    use quote::quote;

    let total = syn::parse_macro_input!(total as syn::LitInt);
    let total: u16 = total.base10_parse().unwrap();

    let traits = (1..=total).map(|i| {
        let tuple_trait = quote::format_ident!("Tuple{i}");
        let item_type = quote::format_ident!("T{i}");

        let tuple_impls = (i..=total).map(|j| {
            let generics = (1..=j).map(|k| quote::format_ident!("T{k}"));
            let generics2 = generics.clone();

            quote! {
                impl<#(#generics,)*> #tuple_trait for (#(#generics2,)*) {
                    type #item_type = #item_type;
                }
            }
        });

        let maybe_super_trait = if i > 1 {
            let prev = quote::format_ident!("Tuple{}", i - 1);
            quote!(: #prev)
        } else {
            quote! {}
        };

        quote! {
            trait #tuple_trait #maybe_super_trait {
                type #item_type;
            }

            #(#tuple_impls)*
        }
    });

    traits.concat().into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __gen_tuple_traits2(total: TokenStream) -> TokenStream {
    use quote::quote;

    let total = syn::parse_macro_input!(total as syn::LitInt);
    let total: u16 = total.base10_parse().unwrap();

    let items = (1..=total).map(|i| quote::format_ident!("T{i}"));

    let impls = (1..=total).map(|i| {
        let covered = (1..=i).map(|j| quote::format_ident!("T{j}"));
        let covered2 = covered.clone();
        let covered3 = covered.clone();
        let rest = (i + 1..=total).map(|j| quote::format_ident!("T{j}"));

        quote! {
            impl<#(#covered,)*> Tuple for (#(#covered2,)*) {
                #( type #covered3 = #covered3; )*
                #( type #rest = N; )*
            }
        }
    });

    quote! {
        pub trait Tuple {
            #( type #items;)*
        }

        #(#impls)*
    }
    .into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __builder_type(params: TokenStream) -> TokenStream {
    use quote::quote;

    enum MemberKind {
        Required,
        Optional,
    }

    mod kw {
        syn::custom_keyword!(required);
        syn::custom_keyword!(optional);
    }

    struct Input {
        builder_ident: syn::Ident,
        members: Vec<(syn::Ident, MemberKind)>,
        set_members: Vec<syn::Ident>,
    }

    impl syn::parse::Parse for Input {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let builder_ident = input.parse()?;

            let schema;

            syn::braced!(schema in input);

            let parse_members = |input: syn::parse::ParseStream<'_>| {
                let key: syn::Ident = input.parse()?;
                input.parse::<syn::Token![:]>()?;

                let lookahead = input.lookahead1();

                let kind = if lookahead.peek(kw::required) {
                    input.parse::<kw::required>()?;
                    MemberKind::Required
                } else if lookahead.peek(kw::optional) {
                    input.parse::<kw::optional>()?;
                    MemberKind::Optional
                } else {
                    return Err(lookahead.error());
                };

                Ok((key, kind))
            };

            let members = schema.parse_terminated(parse_members, syn::Token![,])?;
            let members = Vec::from_iter(members);

            let set_members = input.parse_terminated(syn::Ident::parse, syn::Token![,])?;
            let set_members = Vec::from_iter(set_members);

            Ok(Self {
                builder_ident,
                members,
                set_members,
            })
        }
    }

    let input = syn::parse_macro_input!(params as Input);

    let Input {
        builder_ident,
        members,

        // TODO: validate only correct members are specified,
        // maybe add completions
        set_members,
    } = input;

    let p = quote!(::bon::private);

    let state_types = members.iter().enumerate().map(|(i, (ident, kind))| {
        let is_set = set_members.contains(ident);

        if is_set {
            let tuple_item = quote::format_ident!("T{}", i + 1);
            quote! {
                #p::Set<
                    <
                        <#builder_ident as #p::state::Members>::Members
                        as
                        #p::state::Tuple
                    >::#tuple_item
                >
            }
        } else {
            match kind {
                MemberKind::Required => quote! { #p::Unset<#p::Required> },
                MemberKind::Optional => quote! { #p::Unset<#p::Optional> },
            }
        }
    });

    // TODO: deliver regular generics from input
    quote! {
        #builder_ident<(#(#state_types,)*)>
    }
    .into()
}
