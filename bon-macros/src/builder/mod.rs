mod field;
mod free_fn_item;
mod impl_block;
mod setter_methods;

pub(crate) use free_fn_item::*;
pub(crate) use impl_block::*;

use field::*;

use crate::normalization::NormalizeSelfTy;
use heck::AsPascalCase;
use itertools::Itertools;
use prox::prelude::*;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

struct ImplCtx<'a> {
    self_ty: &'a syn::Type,
    generics: &'a syn::Generics,
}

struct MacroCtx<'a> {
    impl_ctx: Option<ImplCtx<'a>>,
    adapted_func: syn::ItemFn,
    norm_func: syn::ItemFn,
    fields: Vec<Field>,
    builder_ident: syn::Ident,
    builder_private_impl_ident: syn::Ident,
    builder_state_trait_ident: syn::Ident,
}

struct MacroOutput {
    entry_func: syn::ItemFn,
    adapted_func: syn::ItemFn,
    other_items: TokenStream2,
}

impl<'a> MacroCtx<'a> {
    fn new(
        orig_func: syn::ItemFn,
        norm_func: syn::ItemFn,
        impl_block: Option<ImplCtx<'a>>,
    ) -> Result<Self> {
        if let Some(receiver) = norm_func.sig.receiver() {
            if impl_block.is_none() {
                prox::bail!(
                    &receiver.self_token,
                    "Function contains a `self` parameter, but #[bon] attribute \
                    is absent on top of the impl block. This additional #[bon] \
                    attribute on the impl block is required for the macro to see \
                    the type of `Self` and properly generate the builder struct \
                    definition adjacently to the impl block."
                );
            }
        }

        let self_ty_prefix = impl_block
            .as_ref()
            .and_then(|impl_block| match &impl_block.self_ty {
                syn::Type::Path(path) => Some(path.path.segments.last()?.ident.to_string()),
                _ => None,
            });

        let self_ty_prefix = self_ty_prefix.as_deref();

        let pascal_case_func = AsPascalCase(norm_func.sig.ident.to_string());
        let builder_ident = quote::format_ident!(
            "{}{pascal_case_func}Builder",
            self_ty_prefix.unwrap_or_default()
        );
        let builder_private_impl_ident = quote::format_ident!("__{builder_ident}PrivateImpl");
        let builder_state_trait_ident = quote::format_ident!("__{builder_ident}State");

        let fields: Vec<_> = norm_func
            .sig
            .inputs
            .iter()
            .filter_map(syn::FnArg::as_typed)
            .map(Field::from_typed_fn_arg)
            .try_collect()?;

        let ctx = MacroCtx {
            impl_ctx: impl_block,
            adapted_func: adapt_orig_func(self_ty_prefix, orig_func),
            norm_func,
            fields,
            builder_ident,
            builder_private_impl_ident,
            builder_state_trait_ident,
        };

        Ok(ctx)
    }

    fn field_idents(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.fields.iter().map(|field| field.ident.clone())
    }

    fn field_assoc_type_idents(&self) -> impl Iterator<Item = &syn::Ident> {
        self.fields
            .iter()
            .map(|field| &field.state_assoc_type_ident)
    }

    fn unset_state_types(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        self.fields.iter().map(|arg| arg.unset_state_type())
    }

    fn set_state_types(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        self.fields.iter().map(|arg| arg.set_state_type())
    }

    fn norm_func_receiver_ty(&self) -> Option<Box<syn::Type>> {
        let mut ty = self.norm_func.sig.receiver()?.ty.clone();
        let self_ty = &self.impl_ctx.as_ref()?.self_ty;

        NormalizeSelfTy { self_ty }.visit_type_mut(&mut ty);

        Some(ty)
    }

    fn impl_and_norm_func_generics_decl(&self) -> Vec<&syn::GenericParam> {
        let Some(impl_block) = &self.impl_ctx else {
            return self.norm_func_generics_decl().collect();
        };

        merge_generic_params(
            &impl_block.generics.params,
            &self.norm_func.sig.generics.params,
        )
    }

    fn norm_func_generics_decl(&self) -> impl Iterator<Item = &syn::GenericParam> + '_ {
        self.norm_func.sig.generics.params.iter()
    }

    fn generic_param_to_arg(param: &syn::GenericParam) -> syn::GenericArgument {
        match param {
            syn::GenericParam::Lifetime(param) => {
                syn::GenericArgument::Lifetime(param.lifetime.clone())
            }
            syn::GenericParam::Type(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Type(syn::parse_quote!(#ident))
            }
            syn::GenericParam::Const(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Const(syn::parse_quote!(#ident))
            }
        }
    }

    fn impl_and_norm_func_generic_args(&self) -> impl Iterator<Item = syn::GenericArgument> + '_ {
        self.impl_and_norm_func_generics_decl()
            .into_iter()
            .map(MacroCtx::generic_param_to_arg)
    }

    fn impl_and_norm_func_where_clause(&self) -> Option<syn::WhereClause> {
        let func_where_clause = self.norm_func.sig.generics.where_clause.clone();
        let impl_where_clause = self
            .impl_ctx
            .as_ref()
            .and_then(|impl_block| impl_block.generics.where_clause.clone());

        [impl_where_clause, func_where_clause]
            .into_iter()
            .flatten()
            .reduce(|mut combined, clause| {
                combined.predicates.extend(clause.predicates);
                combined
            })
            .map(|clause| syn::WhereClause {
                where_token: clause.where_token,
                predicates: clause.predicates,
            })
    }

    fn output(self) -> MacroOutput {
        let entry_func = self.entry_func();
        let builder_state_trait_decl = self.builder_state_trait_decl();
        let builder_decl = self.builder_decl();
        let call_method_impl = self.exit_method_impl();
        let setter_methods_impls = self.setter_methods_impls();

        let other_items = quote! {
            #builder_state_trait_decl
            #builder_decl
            #call_method_impl
            #setter_methods_impls
        };

        MacroOutput {
            entry_func,
            adapted_func: self.adapted_func,
            other_items,
        }
    }

    fn entry_func(&self) -> syn::ItemFn {
        let builder_ident = &self.builder_ident;

        let docs = self.norm_func.attrs.iter().filter(|attr| attr.is_doc());

        let current_mod_vis = &self.norm_func.vis;
        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let entry_func_ident = &self.norm_func.sig.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generate code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics_decl = self.norm_func_generics_decl();
        let generic_args = self.impl_and_norm_func_generic_args();
        let where_clause = &self.norm_func.sig.generics.where_clause;

        let field_idents = self.field_idents();
        let phantom_field_init = self.phantom_field_init();

        let unset_state_types = self.unset_state_types();

        let receiver = self.norm_func.sig.receiver();
        let receiver_field_init = receiver.map(|receiver| {
            let self_token = &receiver.self_token;
            quote! {
                receiver: #self_token,
            }
        });

        let func = quote! {
            #(#docs)*
            #current_mod_vis fn #entry_func_ident<#(#generics_decl),*>(
                #receiver
            ) -> #builder_ident<
                #(#generic_args,)*
                (#(#unset_state_types,)*)
            >
            #where_clause
            {
                #builder_ident {
                    __private_impl: #builder_private_impl_ident {
                        #phantom_field_init
                        #receiver_field_init
                        #( #field_idents: ::std::default::Default::default(), )*
                    }
                }
            }
        };

        syn::parse_quote!(#func)
    }

    fn phantom_data(&self) -> Option<TokenStream2> {
        let func_generics = &self.norm_func.sig.generics;
        let generic_lifetimes = func_generics.lifetimes().collect_vec();
        let generic_type_params = func_generics.type_params().collect_vec();

        if generic_type_params.is_empty() && generic_lifetimes.is_empty() && !self.fields.is_empty()
        {
            return None;
        }

        let fn_arg_types = self.fields.iter().map(|field| &field.ty);

        // A special case of zero fields requires storing `__State` in phantom data
        // otherwise it would be reported as an unused type parameter. Another way we
        // could solve it is by special-casing the codegen by not adding the __State
        // generic type parameter to the builder type at all if it has no fields, but
        // to keep code simpler we just do this one small crutch here for a really
        // unlikely case of a builder with zero fields.
        let state = self.fields.is_empty().then(|| quote! { __State });

        Some(quote! {
            ::core::marker::PhantomData<(
                // There is an interesting quirk with lifetimes in Rust, which is the
                // reason why we thoughtlessly store all the function parameter types
                // in phantom data here.
                //
                // Suppose a function was defined with an argument of type `&'a T`
                // and we then generate the an impl block (simplified):
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
                #(#fn_arg_types,)*

                #state
            )>
        })
    }

    fn phantom_field_init(&self) -> Option<TokenStream2> {
        self.phantom_data().map(|_| {
            quote! {
                _phantom: ::core::marker::PhantomData,
            }
        })
    }

    fn builder_state_trait_decl(&self) -> TokenStream2 {
        let trait_ident = &self.builder_state_trait_ident;
        let assoc_types_idents = self.field_assoc_type_idents().collect_vec();

        quote! {
            trait #trait_ident {
                #( type #assoc_types_idents; )*
            }

            impl<#(#assoc_types_idents),*> #trait_ident for (#(#assoc_types_idents,)*) {
                #( type #assoc_types_idents = #assoc_types_idents; )*
            }
        }
    }

    fn builder_decl(&self) -> TokenStream2 {
        let vis = &self.norm_func.vis;
        let builder_ident = &self.builder_ident;
        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let field_idents = self.field_idents();
        let fields_assoc_type_idents = self.field_assoc_type_idents().collect_vec();
        let generics_decl = self.impl_and_norm_func_generics_decl();
        let where_clause = self.impl_and_norm_func_where_clause();
        let generic_args = self.impl_and_norm_func_generic_args();

        let phantom_field = self.phantom_data().map(|phantom_data| {
            quote! {
                _phantom: #phantom_data,
            }
        });

        let receiver_field = self.norm_func_receiver_ty().map(|receiver_ty| {
            quote! {
                receiver: #receiver_ty,
            }
        });

        quote! {
            #vis struct #builder_ident<
                #(#generics_decl,)*

                // We could set default values for `fields_states_vars` here
                // to their initial unset states, but we don't do that and
                // pass these generic params explicitly to workaround the following
                // bug in rust-analyzer where it stops providing completions for
                // builder methods completely if we rely on default generic type
                // params values. See the issue in rust-analyzer for details:
                // https://github.com/rust-lang/rust-analyzer/issues/17515
                __State: #builder_state_trait_ident,
            >
            #where_clause
            {
                /// Please don't touch this field. It's an implementation
                /// detail that is exempt from the API stability guarantees.
                /// It's visible to you only because of the limitations of
                /// the Rust language.
                ///
                /// The limitation is that we can't make the fields of the
                /// generated struct private other than by placing its
                /// declaration inside of a nested submodule. However, we
                /// can't do that because this breaks support for fn items
                /// declared inside of other fn items like this:
                ///
                /// ```ignore
                /// use bon::builder;
                ///
                /// fn foo() {
                ///     struct Foo;
                ///
                ///     #[builder]
                ///     fn nested(foo: Foo) {}
                ///
                ///     nested().foo(Foo).call();
                /// }
                /// ```
                ///
                /// If we were to generate a child module like this then code
                /// in that child module would lose access to the symbol `Foo`
                /// in the parent module. The following code doesn't compile.
                ///
                /// ```ignore
                /// fn foo() {
                ///     struct Foo;
                ///
                ///     mod __private_child_module {
                ///         use super::*;
                ///
                ///         pub(super) struct Builder {
                ///             foo: Foo,
                ///         }
                ///     }
                /// }
                /// ```
                ///
                /// `Foo` symbol is inaccessible inside of `__private_child_module`
                /// because it is defined inside of the function `foo()` and not
                /// inside of the parent module.
                ///
                /// Child modules are kinda implicitly "hoisted" to the top-level of
                /// the module and they can't see the local symbols defined inside
                /// of the same function scope.
                __private_impl: #builder_private_impl_ident<
                    #(#generic_args,)*
                    __State
                >
            }

            /// This struct exists only to reduce the number of private fields
            /// that pop up in IDE completions for developers. It groups all
            /// the private fields in it leaving the builder type higher with
            /// just a single field of this type that documents the fact that
            /// the developers shouldn't touch it.
            struct #builder_private_impl_ident<
                #(#generics_decl,)*
                __State: #builder_state_trait_ident
            >
            #where_clause
            {
                #phantom_field
                #receiver_field
                #( #field_idents: __State::#fields_assoc_type_idents, )*
            }
        }
    }

    fn exit_method_impl(&self) -> TokenStream2 {
        let asyncness = &self.norm_func.sig.asyncness;
        let maybe_await = asyncness.is_some().then(|| quote!(.await));

        let adapted_func_ident = &self.adapted_func.sig.ident;
        let unsafety = &self.norm_func.sig.unsafety;
        let vis = &self.norm_func.vis;
        let builder_ident = &self.builder_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let fields_assoc_type_idents = self.field_assoc_type_idents().collect_vec();
        let set_state_types = self.set_state_types();
        let generics_decl = self.impl_and_norm_func_generics_decl();
        let where_clause_predicates = self
            .impl_and_norm_func_where_clause()
            .into_iter()
            .flat_map(|where_clause| where_clause.predicates);

        let generic_builder_args = self.impl_and_norm_func_generic_args();

        // Filter out lifetime generic arguments, because they are not needed
        // to be specified explicitly when calling the function. This also avoids
        // the problem that it's not always possible to specify lifetimes in
        // the turbofish syntax. See the problem of late-bound lifetimes specification
        // in the issue https://github.com/rust-lang/rust/issues/42868
        let generic_adapted_fn_args = self
            .adapted_func
            .sig
            .generics
            .params
            .iter()
            .filter(|arg| !matches!(arg, syn::GenericParam::Lifetime(_)))
            .map(MacroCtx::generic_param_to_arg);

        // Associated methods of an impl block without the receiver likely create an instance of
        // `Self` so we have a bit different convention for default exit function name in this case.
        let exit_fn_ident = if self.impl_ctx.is_some() && self.norm_func.sig.receiver().is_none() {
            "build"
        } else {
            "call"
        };

        // Bind the span of the original function to call such that "Go to definition"
        // invoked on it in IDEs leads to the original function.
        let exit_fn_ident = syn::Ident::new(exit_fn_ident, self.norm_func.sig.ident.span());

        let prefix = self
            .norm_func
            .sig
            .receiver()
            .map(|receiver| {
                let self_token = &receiver.self_token;
                quote!(#self_token.__private_impl.receiver.)
            })
            .or_else(|| {
                let self_ty = &self.impl_ctx.as_ref()?.self_ty;
                Some(quote!(<#self_ty>::))
            });

        let output_type = &self.norm_func.sig.output;

        let arg_exprs = self.fields.iter().map(|field| {
            let maybe_default = field
                .as_optional()
                // For `Option` fields we don't need any `unwrap_or_[else/default]`.
                // We pass them directly to the function unchanged.
                .filter(|_| !field.ty.is_option())
                .map(|_| {
                    field
                        .params
                        .default
                        .as_ref()
                        .and_then(|val| val.as_ref().as_ref())
                        .map(|default| quote! { .unwrap_or_else(|| #default) })
                        .unwrap_or_else(|| quote! { .unwrap_or_default() })
                });

            let field_ident = &field.ident;

            quote! {
                self.__private_impl
                    .#field_ident
                    .into()
                    .into_inner()
                    #maybe_default
            }
        });

        quote! {
            impl<
                #(#generics_decl,)*
                __State: #builder_state_trait_ident
            >
            #builder_ident<
                #(#generic_builder_args,)*
                __State
            >
            where
                #( #where_clause_predicates, )*
                #(__State::#fields_assoc_type_idents: Into<#set_state_types>,)*
            {
                #vis #asyncness #unsafety fn #exit_fn_ident(self) #output_type {
                    #prefix #adapted_func_ident::<#(#generic_adapted_fn_args,)*>(
                        #( #arg_exprs ),*
                    )
                    #maybe_await
                }
            }
        }
    }

    fn setter_methods_impls(&self) -> TokenStream2 {
        self.fields
            .iter()
            .map(|field| self.setter_methods_impls_for_field(field))
            .collect()
    }
}

fn adapt_orig_func(self_ty_prefix: Option<&str>, mut orig: syn::ItemFn) -> syn::ItemFn {
    let orig_ident = orig.sig.ident.clone();
    orig.sig.ident = quote::format_ident!("__orig_{}", orig_ident.to_string());

    // Change to an implementation function's visibility to private inside of a
    // child module to avoid exposing it to the surrounding code. The surrounding
    // code is supposed to use this function through the builder only.
    orig.vis = syn::Visibility::Inherited;

    strip_known_attrs_from_args(&mut orig.sig);

    // Remove all doc comments from the function itself to avoid docs duplication
    // which may lead to duplicating doc tests, which in turn implies repeated doc
    // tests execution, which means worse tests performance.
    orig.attrs.retain(|attr| !attr.is_doc());

    let builder_entry_fn_link = format!(
        "{}{orig_ident}",
        self_ty_prefix
            .map(|self_ty_prefix| { format!("{self_ty_prefix}::") })
            .unwrap_or_default(),
    );

    let doc = format!(
        "Positional function equivalent of [`{builder_entry_fn_link}()`].\n\
        See its docs for details.",
    );

    orig.attrs.push(syn::parse_quote!(#[doc = #doc]));

    // It's fine if there are too many positional arguments in the function
    // because the whole purpose of this macro is to fight with this problem
    // at the call site by generating a builder, while keeping the fn definition
    // site the same with tons of positional arguments which don't harm readability
    // there because their names are explicitly specified at the definition site.
    orig.attrs
        .push(syn::parse_quote!(#[allow(clippy::too_many_arguments)]));

    orig
}

/// Remove all doc comments attributes from function arguments, because they are
/// not valid in that position in regular Rust code. The cool trick is that they
/// are still valid syntactically when a proc macro like this one pre-processes
/// them and removes them from the expanded code. We use the doc comments to put
/// them on the generated setter methods.
///
/// We also strip all `builder(...)` attributes because this macro processes them
/// and they aren't needed in the output.
fn strip_known_attrs_from_args(sig: &mut syn::Signature) {
    for arg in &mut sig.inputs {
        arg.attrs_mut()
            .retain(|attr| !attr.is_doc() && !attr.path().is_ident("builder"));
    }
}

/// To merge generic params we need to make sure lifetimes are always the first
/// in the resulting list according to Rust syntax restrictions.
fn merge_generic_params<'a>(
    left: &'a Punctuated<syn::GenericParam, syn::Token![,]>,
    right: &'a Punctuated<syn::GenericParam, syn::Token![,]>,
) -> Vec<&'a syn::GenericParam> {
    // False-positive. Peek is used inside of `peeking_take_while`
    #[allow(clippy::unused_peekable)]
    let (mut left, mut right) = (left.iter().peekable(), right.iter().peekable());

    let is_lifetime = |param: &&_| matches!(param, &&syn::GenericParam::Lifetime(_));

    let left_lifetimes = left.peeking_take_while(is_lifetime);
    let right_lifetimes = right.peeking_take_while(is_lifetime);

    let mut generic_params = left_lifetimes.chain(right_lifetimes).collect_vec();
    generic_params.extend(left.chain(right));
    generic_params
}
