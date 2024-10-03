mod builder_derives;
mod builder_params;
mod member;
mod models;
mod setter_methods;
mod state_mod;

pub(crate) mod input_fn;
pub(crate) mod input_struct;

use crate::util::prelude::*;
use member::{
    Member, MemberOrigin, NamedMember, PositionalFnArgMember, RawMember, StartFnArgMember,
};
use models::{
    AssocMethodCtx, AssocMethodReceiverCtx, BuilderGenCtx, FinishFn, FinishFnBody, Generics,
};
use setter_methods::MemberSettersCtx;

pub(crate) struct MacroOutput {
    pub(crate) start_fn: syn::ItemFn,
    pub(crate) other_items: TokenStream,
}

impl BuilderGenCtx {
    fn receiver(&self) -> Option<&AssocMethodReceiverCtx> {
        self.assoc_method_ctx.as_ref()?.receiver.as_ref()
    }

    fn named_members(&self) -> impl Iterator<Item = &NamedMember> {
        self.members.iter().filter_map(Member::as_named)
    }

    fn start_fn_args(&self) -> impl Iterator<Item = &StartFnArgMember> {
        self.members.iter().filter_map(Member::as_start_fn_arg)
    }

    fn stateful_members(&self) -> impl Iterator<Item = &NamedMember> {
        self.named_members().filter(|member| member.is_stateful())
    }

    pub(crate) fn output(self) -> Result<MacroOutput> {
        let mut start_fn = self.start_fn();
        let state_mod = self.state_mod();
        let builder_decl = self.builder_decl();
        let builder_impl = self.builder_impl();
        let builder_derives = self.builder_derives();

        let default_allows = syn::parse_quote!(#[allow(
            // We have a `deprecated` lint on any `bon::private` items which we
            // use in the generated code extensively
            deprecated
        )]);

        let allows = self.allow_attrs.iter().cloned().chain([default_allows]);

        // -- Postprocessing --
        // Here we parse all items back and add the `allow` attributes to them.
        let other_items = quote! {
            #state_mod
            #builder_decl
            #builder_derives
            #builder_impl
        };

        let other_items_str = other_items.to_string();

        let other_items: syn::File = syn::parse2(other_items).map_err(|err| {
            err!(
                &Span::call_site(),
                "bug in the `bon` crate: the macro generated code that contains syntax errors; \
                please report this issue at our Github repository: \
                https://github.com/elastio/bon.\n\
                syntax error in generated code: {err:#?}.\n\
                generated code:\n\
                ```rust
                {other_items_str}\n\
                ```",
            )
        })?;

        let mut other_items = other_items.items;

        for item in &mut other_items {
            if let Some(attrs) = item.attrs_mut() {
                attrs.extend(allows.clone());
            }
        }

        start_fn.attrs.extend(allows);

        Ok(MacroOutput {
            start_fn,
            other_items: quote!(#(#other_items)*),
        })
    }

    fn builder_impl(&self) -> TokenStream {
        let finish_fn = self.finish_fn();
        let transition_type_state_fn = self.transition_type_state_fn();
        let setter_methods = self
            .named_members()
            .map(|member| MemberSettersCtx::new(self, member).setter_methods());

        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;
        let builder_ident = &self.builder_type.ident;
        let state_mod = &self.state_mod.ident;

        let allows = allow_warnings_on_member_types();

        quote! {
            #allows
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderState: #state_mod::State
            >
            #builder_ident<
                #(#generic_args,)*
                BuilderState
            >
            #where_clause
            {
                #finish_fn
                #(#setter_methods)*
                #transition_type_state_fn
            }
        }
    }

    /// Generates code that has no meaning to the compiler, but it helps
    /// IDEs to provide better code highlighting, completions and other
    /// hints.
    fn ide_hints(&self) -> TokenStream {
        let type_patterns = self
            .on_params
            .iter()
            .map(|params| &params.type_pattern)
            .collect::<Vec<_>>();

        if type_patterns.is_empty() {
            return quote! {};
        }

        quote! {
            // This is wrapped in a special cfg set by `rust-analyzer` to enable this
            // code for rust-analyzer's analysis only, but prevent the code from being
            // compiled by `rustc`. Rust Analyzer should be able to use the syntax
            // provided inside of the block to figure out the semantic meaning of
            // the tokens passed to the attribute.
            #[cfg(rust_analyzer)]
            {
                // Let IDEs know that these are type patterns like the ones that
                // could be written in a type annotation for a variable. Note that
                // we don't initialize the variable with any value because we don't
                // have any meaningful value to assign to this variable, especially
                // because its type may contain wildcard patterns like `_`. This is
                // used only to signal the IDEs that these tokens are meant to be
                // type patterns by placing them in the context where type patterns
                // are expected.
                let _: (#(#type_patterns,)*);
            }
        }
    }

    fn transition_type_state_fn(&self) -> TokenStream {
        let builder_ident = &self.builder_type.ident;
        let state_mod = &self.state_mod.ident;

        let maybe_receiver_field = self
            .receiver()
            .map(|_| quote!(__private_receiver: self.__private_receiver,));

        let maybe_start_fn_args_field = self
            .start_fn_args()
            .next()
            .map(|_| quote!(__private_start_fn_args: self.__private_start_fn_args,));

        let generic_args = &self.generics.args;

        quote! {
            #[deprecated =
                "this method is an implementation detail; it should not be used directly; \
                if you found yourself needing it, then you are probably doing something wrong; \
                feel free to open an issue/discussion in our GitHub repository \
                (https://github.com/elastio/bon) or ask for help in our Discord server \
                (https://discord.gg/QcBYSamw4c)"
            ]
            #[inline(always)]
            fn __private_transition_type_state<__NewBuilderState: #state_mod::State>(self)
            -> #builder_ident<#(#generic_args,)* __NewBuilderState>
            {
                #builder_ident {
                    __private_phantom: ::core::marker::PhantomData,
                    #maybe_receiver_field
                    #maybe_start_fn_args_field
                    __private_named_members: self.__private_named_members,
                }
            }
        }
    }

    fn start_fn(&self) -> syn::ItemFn {
        let builder_ident = &self.builder_type.ident;
        let attrs = &self.start_fn.attrs;
        let vis = &self.start_fn.vis;

        let start_fn_ident = &self.start_fn.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generated code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics = self.start_fn.generics.as_ref().unwrap_or(&self.generics);

        let generics_decl = &generics.decl_without_defaults;
        let where_clause = &generics.where_clause;
        let generic_args = &self.generics.args;

        let receiver = self.receiver();

        let receiver_field_init = receiver.map(|receiver| {
            let self_token = &receiver.with_self_keyword.self_token;
            quote! {
                __private_receiver: #self_token,
            }
        });

        let receiver = receiver.map(|receiver| {
            let receiver = &receiver.with_self_keyword;
            quote! { #receiver, }
        });

        let start_fn_params = self
            .start_fn_args()
            .map(|member| member.base.fn_input_param());

        let mut start_fn_arg_exprs = self
            .start_fn_args()
            .map(|member| member.base.maybe_into_ident_expr())
            .peekable();

        let start_fn_args_field_init = start_fn_arg_exprs.peek().is_some().then(|| {
            quote! {
                __private_start_fn_args: (#(#start_fn_arg_exprs,)*),
            }
        });

        let ide_hints = self.ide_hints();

        // `Default` trait implementation is provided only for tuples up to 12
        // elements in the standard library 😳:
        // https://github.com/rust-lang/rust/blob/67bb749c2e1cf503fee64842963dd3e72a417a3f/library/core/src/tuple.rs#L213
        let named_members_field_init = if self.named_members().take(13).count() <= 12 {
            quote!(::core::default::Default::default())
        } else {
            let none = quote!(::core::option::Option::None);
            let nones = self.named_members().map(|_| &none);
            quote! {
                (#(#nones,)*)
            }
        };

        syn::parse_quote! {
            #(#attrs)*
            #[inline(always)]
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We normalize `Self` references intentionally to simplify code generation
                clippy::use_self,
                // Let's keep it as non-const for now to avoid restricting ourselfves to only
                // const operations.
                clippy::missing_const_for_fn
            )]
            #vis fn #start_fn_ident<#(#generics_decl),*>(
                #receiver
                #(#start_fn_params,)*
            ) -> #builder_ident<#(#generic_args,)*>
            #where_clause
            {
                #ide_hints

                #builder_ident {
                    __private_phantom: ::core::marker::PhantomData,
                    #receiver_field_init
                    #start_fn_args_field_init
                    __private_named_members: #named_members_field_init,
                }
            }
        }
    }

    fn phantom_data(&self) -> TokenStream {
        let member_types = self.members.iter().filter_map(|member| {
            match member {
                // The types of these members already appear in the struct in the types
                // of __private_named_members and __private_start_fn_args fields.
                Member::Named(_) | Member::StartFnArg(_) => None,
                Member::FinishFnArg(member) => Some(member.norm_ty.as_ref()),
                Member::Skipped(member) => Some(member.norm_ty.as_ref()),
            }
        });

        let receiver_ty = self
            .assoc_method_ctx
            .as_ref()
            .map(|ctx| ctx.self_ty.as_ref());

        let generic_args = &self.generics.args;
        let generic_types = generic_args.iter().filter_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => Some(ty),
            _ => None,
        });

        let types = receiver_ty
            .into_iter()
            .chain(member_types)
            .chain(generic_types)
            .map(|ty| {
                // Wrap `ty` in another phantom data because it can be `?Sized`,
                // and simply using it as a type of the tuple member would
                // be wrong, because tuple's members must be sized
                quote!(fn() -> ::core::marker::PhantomData<#ty>)
            });

        quote! {
            ::core::marker::PhantomData<(
                // There is an interesting quirk with lifetimes in Rust, which is the
                // reason why we thoughtlessly store all the function parameter types
                // in phantom data here.
                //
                // Suppose a function was defined with an argument of type `&'a T`
                // and we then generate an impl block (simplified):
                //
                // ```
                // impl<'a, T, U> for Foo<U>
                // where
                //     U: Into<&'a T>,
                // {}
                // ```
                // Then compiler will complain with the message "the parameter type `T`
                // may not live long enough". So we would need to manually add the bound
                // `T: 'a` to fix this. However, it's hard to infer such a bound in macro
                // context. A workaround for that would be to store the `&'a T` inside of
                // the struct itself, which auto-implies this bound for us implicitly.
                //
                // That's a weird implicit behavior in Rust, I suppose there is a reasonable
                // explanation for it, I just didn't care to research it yet ¯\_(ツ)_/¯.
                #(#types,)*

                // A special case of zero members requires storing `BuilderState` in
                // phantom data otherwise it would be reported as an unused type parameter.
                fn() -> ::core::marker::PhantomData<BuilderState>,
            )>
        }
    }

    fn builder_decl(&self) -> TokenStream {
        let builder_vis = &self.builder_type.vis;
        let builder_ident = &self.builder_type.ident;
        let generics_decl = &self.generics.decl_with_defaults;
        let where_clause = &self.generics.where_clause;
        let phantom_data = self.phantom_data();
        let state_mod = &self.state_mod.ident;

        let private_field_attrs = quote! {
            // The fields can't be hidden using Rust's privacy syntax.
            // The details about this are described in [the blog post]
            // (https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
            //
            // We could use `#[cfg(not(rust_analyzer))]` to hide the private fields in IDE.
            // However, RA would then not be able to type-check the generated
            // code, which may or may not be a problem, because the main thing
            // is that the type signatures would still work in RA.
            #[doc(hidden)]
            #[deprecated =
                "this field is an implementation detail; it should not be used directly; \
                if you found yourself needing it, then you are probably doing something wrong; \
                feel free to open an issue/discussion in our GitHub repository \
                (https://github.com/elastio/bon) or ask for help in our Discord server \
                (https://discord.gg/QcBYSamw4c)"]
        };

        let receiver_field = self.receiver().map(|receiver| {
            let ty = &receiver.without_self_keyword;
            quote! {
                #private_field_attrs
                __private_receiver: #ty,
            }
        });

        let must_use_message = format!(
            "the builder does nothing until you call `{}()` on it to finish building",
            self.finish_fn.ident
        );

        let allows = allow_warnings_on_member_types();

        let mut start_fn_arg_types = self
            .start_fn_args()
            .map(|member| &member.base.norm_ty)
            .peekable();

        let start_fn_args_field = start_fn_arg_types.peek().is_some().then(|| {
            quote! {
                #private_field_attrs
                __private_start_fn_args: (#(#start_fn_arg_types,)*),
            }
        });

        let named_members_types = self.named_members().map(NamedMember::underlying_norm_ty);

        let docs = &self.builder_type.docs;

        quote! {
            #[must_use = #must_use_message]
            #(#docs)*
            #allows
            #[allow(
                // We use `__private` prefix for all fields intentionally to hide them
                clippy::struct_field_names,

                // This lint doesn't emerge until you manually expand the macro. Just
                // because `bon` developers need to expand the macros a lot it makes
                // sense to just silence it to avoid some noise. This lint is triggered
                // by the big PhantomData type generated by the macro
                clippy::type_complexity
            )]
            #builder_vis struct #builder_ident<
                #(#generics_decl,)*
                // Having the `State` trait bound on the struct declaration is important
                // for future proofing. It will allow us to use this bound in the `Drop`
                // implementation of the builder if we ever add one. @Veetaha already did
                // some experiments with `MaybeUninit` that requires a custom drop impl,
                // so this could be useful in the future.
                //
                // On the flip side, if we have a custom `Drop` impl, then partially moving
                // the builder will be impossible. So.. it's a trade-off, and it's probably
                // not a big deal to remove this bound from here if we feel like it.
                BuilderState: #state_mod::State = #state_mod::AllUnset
            >
            #where_clause
            {
                #private_field_attrs
                __private_phantom: #phantom_data,

                #receiver_field
                #start_fn_args_field

                #private_field_attrs
                __private_named_members: (
                    #(
                        ::core::option::Option<#named_members_types>,
                    )*
                )
            }
        }
    }

    fn finish_fn_member_expr(member: &Member) -> TokenStream {
        let member = match member {
            Member::Named(member) => member,
            Member::Skipped(member) => {
                return member
                    .value
                    .as_ref()
                    .as_ref()
                    .map(|value| quote! { (|| #value)() })
                    .unwrap_or_else(|| quote! { ::core::default::Default::default() });
            }
            Member::StartFnArg(member) => {
                let index = &member.index;
                return quote! { self.__private_start_fn_args.#index };
            }
            Member::FinishFnArg(member) => {
                return member.maybe_into_ident_expr();
            }
        };

        let index = &member.index;

        let member_field = quote! {
            self.__private_named_members.#index
        };

        let param_default = member
            .params
            .default
            .as_ref()
            .map(|default| default.value.as_ref());

        match param_default {
            Some(Some(default)) => {
                let has_into = member.params.into.is_present();
                let default = if has_into {
                    quote! { ::core::convert::Into::into((|| #default)()) }
                } else {
                    quote! { #default }
                };

                quote! {
                    ::core::option::Option::unwrap_or_else(#member_field, || #default)
                }
            }
            Some(None) => {
                quote! {
                    ::core::option::Option::unwrap_or_default(#member_field)
                }
            }
            None => {
                // For `Option` the default value is always `None`. So we can just return
                // the value of the member field itself (which is already an `Option<T>`).
                if !member.params.transparent.is_present() && member.norm_ty.is_option() {
                    return member_field.to_token_stream();
                }

                quote! {
                    unsafe {
                        // SAFETY: we know that the member is set because we are in
                        // the `finish` function because this method uses the trait
                        // bounds of `IsSet` for every required member. It's also
                        // not possible to intervene with the builder's state from
                        // the outside because all members of the builder are considered
                        // private (we even generate random names for them to make it
                        // impossible to access them from the outside in the same module).
                        ::core::option::Option::unwrap_unchecked(#member_field)
                    }
                }
            }
        }
    }

    fn finish_fn(&self) -> TokenStream {
        let members_vars_decls = self.members.iter().map(|member| {
            let expr = Self::finish_fn_member_expr(member);
            let var_ident = member.orig_ident();

            // The type hint is necessary in some cases to assist the compiler
            // in type inference.
            //
            // For example, if the expression is passed to a function that accepts
            // an impl Trait such as `impl Default`, and the expression itself looks
            // like `Default::default()`. In this case nothing hints to the compiler
            // the resulting type of the expression, so we add a type hint via an
            // intermediate variable here.
            let ty = member.norm_ty();

            quote! {
                let #var_ident: #ty = #expr;
            }
        });

        let state_mod = &self.state_mod.ident;

        let finish_fn_params = self
            .members
            .iter()
            .filter_map(Member::as_finish_fn_arg)
            .map(PositionalFnArgMember::fn_input_param);

        let body = &self.finish_fn.body.generate(&self.members);
        let asyncness = &self.finish_fn.asyncness;
        let unsafety = &self.finish_fn.unsafety;
        let must_use = &self.finish_fn.must_use;
        let attrs = &self.finish_fn.attrs;
        let finish_fn_vis = self
            .finish_fn
            .vis
            .as_ref()
            .unwrap_or(&self.builder_type.vis);
        let finish_fn_ident = &self.finish_fn.ident;
        let output = &self.finish_fn.output;

        quote! {
            #(#attrs)*
            #[inline(always)]
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,

                // This lint flags any function that returns a possibly `!Send` future.
                // However, it doesn't apply in the generic context where the future is
                // `Send` if the generic parameters are `Send` as well, so we just suppress
                // this lint. See the issue: https://github.com/rust-lang/rust-clippy/issues/6947
                clippy::future_not_send,
            )]
            #must_use
            #finish_fn_vis #asyncness #unsafety fn #finish_fn_ident(self, #(#finish_fn_params,)*) #output
            where
                BuilderState: #state_mod::IsComplete
            {
                #(#members_vars_decls)*
                #body
            }
        }
    }
}

fn allow_warnings_on_member_types() -> TokenStream {
    quote! {
        // This warning may occur when the original unnormalized syntax was
        // using parens around an `impl Trait` like that:
        // ```
        // &(impl Clone + Default)
        // ```
        // in which case the normalized version will be:
        // ```
        // &(T)
        // ```
        //
        // And it triggers the warning. We just suppress it here.
        #[allow(unused_parens)]
    }
}
