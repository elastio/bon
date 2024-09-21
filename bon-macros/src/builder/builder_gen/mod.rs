mod builder_derives;
mod builder_params;
mod member;
mod setter_methods;

pub(crate) mod input_func;
pub(crate) mod input_struct;

use crate::util::prelude::*;
use builder_params::{BuilderDerives, OnParams};
use member::{Member, MemberOrigin, NamedMember, RawMember, StartFnArgMember};
use quote::{quote, ToTokens};
use setter_methods::{MemberSettersCtx, SettersReturnType};
use std::hint::unreachable_unchecked;

struct AssocMethodReceiverCtx {
    with_self_keyword: syn::Receiver,
    without_self_keyword: Box<syn::Type>,
}

struct AssocMethodCtx {
    /// The `Self` type of the impl block. It doesn't contain any nested
    /// `Self` keywords in it. This is prohibited by Rust's syntax itself.
    self_ty: Box<syn::Type>,

    /// Present only if the method has a receiver, i.e. `self` or `&self` or
    /// `&mut self` or `self: ExplicitType`.
    receiver: Option<AssocMethodReceiverCtx>,
}

pub(crate) struct BuilderGenCtx {
    members: Vec<Member>,

    /// Lint suppressions from the original item that will be inherited by all items
    /// generated by the macro. If the original syntax used `#[expect(...)]`,
    /// then it must be represented as `#[allow(...)]` here.
    allow_attrs: Vec<syn::Attribute>,
    on_params: Vec<OnParams>,

    generics: Generics,
    vis: syn::Visibility,
    assoc_method_ctx: Option<AssocMethodCtx>,

    builder_type: BuilderType,
    start_func: StartFunc,
    finish_func: FinishFunc,
}

struct FinishFunc {
    ident: syn::Ident,

    /// Additional attributes to apply to the item
    attrs: Vec<syn::Attribute>,

    unsafety: Option<syn::Token![unsafe]>,
    asyncness: Option<syn::Token![async]>,
    /// <https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute>
    must_use: Option<syn::Attribute>,
    body: Box<dyn FinishFuncBody>,
    output: syn::ReturnType,
}

struct StartFunc {
    ident: syn::Ident,

    /// Additional attributes to apply to the item
    attrs: Vec<syn::Attribute>,

    /// Overrides the common generics
    generics: Option<Generics>,

    /// If present overrides the automatic visibility
    vis: Option<syn::Visibility>,
}

struct BuilderType {
    ident: syn::Ident,

    derives: BuilderDerives,

    /// Optional docs override
    docs: Option<Vec<syn::Attribute>>,
}

pub(crate) trait FinishFuncBody {
    /// Generate the `finish` function body from the ready-made variables.
    /// The generated function body may assume that there are variables
    /// named the same as the members in scope.
    fn generate(&self, members: &[Member]) -> TokenStream2;
}

struct Generics {
    where_clause: Option<syn::WhereClause>,

    /// Original generics that may contain default values in them. This is only
    /// suitable for use in places where default values for generic parameters
    /// are allowed.
    decl_with_defaults: Vec<syn::GenericParam>,

    /// Generic parameters without default values in them. This is suitable for
    /// use as generics in function signatures or impl blocks.
    decl_without_defaults: Vec<syn::GenericParam>,

    /// Mirrors the `decl` representing how generic params should be represented
    /// when these parameters are passed through as arguments in a turbofish.
    args: Vec<syn::GenericArgument>,
}

impl Generics {
    fn new(
        decl_with_defaults: Vec<syn::GenericParam>,
        where_clause: Option<syn::WhereClause>,
    ) -> Self {
        let decl_without_defaults = decl_with_defaults
            .iter()
            .cloned()
            .map(|mut param| {
                match &mut param {
                    syn::GenericParam::Type(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Const(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Lifetime(_) => {}
                }
                param
            })
            .collect();

        let args = decl_with_defaults
            .iter()
            .map(generic_param_to_arg)
            .collect();

        Self {
            where_clause,
            decl_with_defaults,
            decl_without_defaults,
            args,
        }
    }

    fn where_clause_predicates(&self) -> impl Iterator<Item = &syn::WherePredicate> {
        self.where_clause
            .as_ref()
            .into_iter()
            .flat_map(|clause| &clause.predicates)
    }
}

pub(crate) struct MacroOutput {
    pub(crate) start_func: syn::ItemFn,
    pub(crate) other_items: TokenStream2,
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

    fn builder_state_trait_ident(&self) -> syn::Ident {
        quote::format_ident!("{}State", self.builder_ident.raw_name())
    }

    pub(crate) fn output(self) -> Result<MacroOutput> {
        let mut start_func = self.start_func()?;
        let builder_decl = self.builder_decl();
        let builder_impl = self.builder_impl()?;
        let builder_derives = self.builder_derives();
        // let builder_type_macro = self.builder_type_macro();

        // let all = quote! {
        //     #builder_decl
        //     #builder_derives
        //     #builder_impl
        //     #builder_type_macro
        // };

        // eprintln!("{}", all.to_string());

        // -- Postprocessing --
        // Here we parse all items back and add the `allow` attributes to them.
        let other_items: syn::File = syn::parse_quote! {
            #builder_decl
            #builder_derives
            #builder_impl
            // #builder_type_macro
        };

        let mut other_items = other_items.items;

        for item in &mut other_items {
            if let Some(attrs) = item.attrs_mut() {
                attrs.extend(self.allow_attrs.iter().cloned());
            }
        }

        start_func.attrs.extend(self.allow_attrs);

        Ok(MacroOutput {
            start_func,
            other_items: quote!(#(#other_items)*),
        })
    }

    fn builder_type_macro(&self) -> TokenStream2 {
        let private_macro_ident = quote::format_ident!("__{}", self.builder_ident.raw_name());

        let vis = &self.vis;
        let builder_ident = &self.builder_ident;

        let member_types = self.named_members().map(|member| &member.norm_ty);
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;

        let schema = self.named_members().map(|member| {
            let member_ident = &member.public_ident();
            if member.is_optional() {
                quote!(#member_ident: optional)
            } else {
                quote!(#member_ident: required)
            }
        });

        quote! {
            impl <#(#generics_decl,)*>
                ::bon::private::state::Members for #builder_ident<#(#generic_args,)*>
            #where_clause
            {
                type Members = (#(#member_types,)*);
            }

            macro_rules! #private_macro_ident {
                ($($tt:tt)*) => {
                    ::bon::__builder_type!(
                        #builder_ident {
                            #(#schema,)*
                        }
                        $($tt)*
                    )
                }
            }

            #vis use #private_macro_ident as #builder_ident;
        }
    }

    fn builder_impl(&self) -> Result<TokenStream2> {
        let finish_method = self.finish_method()?;
        let (setter_methods, items_for_rustdoc) = self.setter_methods()?;

        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let where_clause = &self.generics.where_clause;
        let builder_ident = &self.builder_type.ident;
        let builder_state_trait_ident = self.builder_state_trait_ident();

        let allows = allow_warnings_on_member_types();

        let named_members_labels = self.named_members().map(|member| self.member_label(member));

        let vis = &self.vis;

        Ok(quote! {
            #items_for_rustdoc

            #(
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                #vis struct #named_members_labels;
            )*

            #allows
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderState: #builder_state_trait_ident
            >
            #builder_ident<
                #(#generic_args,)*
                BuilderState
            >
            #where_clause
            {
                #finish_method
                #setter_methods
            }
        })
    }

    fn start_func_generics(&self) -> &Generics {
        self.start_func.generics.as_ref().unwrap_or(&self.generics)
    }

    /// Generates code that has no meaning to the compiler, but it helps
    /// IDEs to provide better code highlighting, completions and other
    /// hints.
    fn ide_hints(&self) -> TokenStream2 {
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

    fn start_func(&self) -> Result<syn::ItemFn> {
        let builder_ident = &self.builder_type.ident;

        let docs = &self.start_func.attrs;
        let vis = self.start_func.vis.as_ref().unwrap_or(&self.vis);

        let start_func_ident = &self.start_func.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generated code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics = self.start_func_generics();

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
            .map(|member| member.base.fn_input_param(&self.on_params))
            .collect::<Result<Vec<_>>>()?;

        let start_fn_arg_exprs = self
            .start_fn_args()
            .map(|member| member.base.maybe_into_ident_expr(&self.on_params))
            .collect::<Result<Vec<_>>>()?;

        let start_fn_args_field_init = (!start_fn_arg_exprs.is_empty()).then(|| {
            quote! {
                __private_start_fn_args: (#(#start_fn_arg_exprs,)*),
            }
        });

        let ide_hints = self.ide_hints();

        let named_members_init = self.named_members().map(|member| {
            let member_ident = &member.norm_ident;
            if member.is_optional() {
                quote! {
                    #member_ident: None
                }
            } else {
                quote! {
                    #member_ident: ::bon::private::MemberCell::uninit()
                }
            }
        });

        let func = quote! {
            #(#docs)*
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
            #vis fn #start_func_ident<#(#generics_decl),*>(
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
                    #(#named_members_init,)*
                }
            }
        };

        Ok(syn::parse_quote!(#func))
    }

    fn phantom_data(&self) -> TokenStream2 {
        let member_types = self.members.iter().map(Member::norm_ty);
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
                quote!(::core::marker::PhantomData<#ty>)
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

                // A special case of zero members requires storing `BuilderState` in phantom data
                // otherwise it would be reported as an unused type parameter.
                ::core::marker::PhantomData<BuilderState>
            )>
        }
    }

    fn builder_decl(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_type.ident;
        let generics_decl = &self.generics.decl_with_defaults;
        let where_clause = &self.generics.where_clause;
        let phantom_data = self.phantom_data();

        let private_field_doc = "\
            Please don't touch this field. It's an implementation \
            detail that is exempt from the API stability guarantees. \
            This field couldn't be hidden using Rust's privacy syntax. \
            The details about this are described in [the blog post]\
            (https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        ";

        let receiver_field = self.receiver().map(|receiver| {
            let ty = &receiver.without_self_keyword;
            quote! {
                #[doc = #private_field_doc]
                __private_receiver: #ty,
            }
        });

        let must_use_message = format!(
            "the builder does nothing until you call `{}()` on it to finish building",
            self.finish_func.ident
        );

        let docs = self.builder_type.docs.clone().unwrap_or_else(|| {
            let doc = format!(
                "Use builder syntax to set the required parameters and finish \
                by calling the method [`Self::{}()`].",
                self.finish_func.ident
            );

            vec![syn::parse_quote! {
                #[doc = #doc]
            }]
        });

        let allows = allow_warnings_on_member_types();

        let state_all_unset = quote::format_ident!("{}StateAllUnset", builder_ident.raw_name());

        let unset_state_types = self.named_members().map(|member| {
            let label = self.member_label(member);
            quote!(::bon::private::Unset<#label>)
        });

        let mut start_fn_arg_types = self
            .start_fn_args()
            .map(|member| &member.base.norm_ty)
            .peekable();

        let start_fn_args_field = start_fn_arg_types.peek().is_some().then(|| {
            quote! {
                #[doc = #private_field_doc]
                __private_start_fn_args: (#(#start_fn_arg_types,)*),
            }
        });

        let member_fields = self.named_members().map(|member| {
            let ident = &member.norm_ident;

            if let Some(ty) = member.as_optional_norm_ty() {
                quote! {
                    #[doc = #private_field_doc]
                    #ident: ::core::option::Option<#ty>
                }
            } else {
                let ty = &member.norm_ty;
                let member_pascal = &member.norm_ident_pascal;
                quote! {
                    #[doc = #private_field_doc]
                    #ident: ::bon::private::MemberCell<BuilderState::#member_pascal, #ty>
                }
            }
        });

        let builder_state_trait_ident = self.builder_state_trait_ident();
        let named_members_pascal_idents: Vec<_> = self
            .named_members()
            .map(|member| &member.norm_ident_pascal)
            .collect();

        let total_named_members = named_members_pascal_idents.len();

        let state_transition_aliases = self.named_members().map(|member| {
            let alias_name = quote::format_ident!(
                "{}Set{}",
                self.builder_ident.raw_name(),
                member.norm_ident_pascal.raw_name()
            );

            let states = self.named_members().map(|other_member| {
                if other_member.orig_ident == member.orig_ident {
                    let label = self.member_label(member);
                    quote! {
                        ::bon::private::Set<#label>
                    }
                } else {
                    let member_pascal = &other_member.norm_ident_pascal;
                    quote! {
                        S::#member_pascal
                    }
                }
            });

            if total_named_members == 1 {
                return quote! {
                    #vis type #alias_name = ( #(#states,)* );
                };
            }

            quote! {
                #vis type #alias_name<S: #builder_state_trait_ident> = ( #(#states,)* );
            }
        });

        quote! {
            #( #state_transition_aliases )*

            #vis trait #builder_state_trait_ident {
                #(type #named_members_pascal_idents: ::bon::private::MemberState; )*
            }

            impl< #(#named_members_pascal_idents: ::bon::private::MemberState,)* > #builder_state_trait_ident
            for ( #(#named_members_pascal_idents,)* )
            {
                #( type #named_members_pascal_idents = #named_members_pascal_idents; )*
            }

            /// Initial state of the builder where all named members are unset
            #vis type #state_all_unset = (#(#unset_state_types,)*);

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
            #vis struct #builder_ident<
                #(#generics_decl,)*
                BuilderState: #builder_state_trait_ident = #state_all_unset
            >
            #where_clause
            {
                // We could use `#[cfg(not(rust_analyzer))]` to hide these.
                // However, RA would then not be able to type-check the generated
                // code, which may or may not be a problem, because the main thing
                // is that the type signatures would still work in RA.
                #[doc = #private_field_doc]
                __private_phantom: #phantom_data,

                #receiver_field
                #start_fn_args_field

                #( #member_fields, )*
            }
        }
    }

    fn member_expr(&self, member: &Member) -> Result<TokenStream2> {
        let member = match member {
            Member::Named(member) => member,
            Member::Skipped(member) => {
                let expr = member
                    .value
                    .as_ref()
                    .as_ref()
                    .map(|value| quote! { #value })
                    .unwrap_or_else(|| quote! { ::core::default::Default::default() });

                return Ok(expr);
            }
            Member::StartFnArg(member) => {
                let index = &member.index;
                return Ok(quote! { self.__private_start_fn_args.#index });
            }
            Member::FinishFnArg(member) => {
                return member.maybe_into_ident_expr(&self.on_params);
            }
        };

        let expr = member
            .as_optional_norm_ty()
            .map(|_| {
                // For `Option` members we don't need any `unwrap_or_[else/default]`.
                // The implementation of `From<Unset> for Set<Option<T>>` already
                // returns an `Option<T>`.
                if member.norm_ty.is_option() {
                    return None;
                }

                let default = member
                    .param_default()
                    .flatten()
                    .map(|default| {
                        let has_into = member.param_into(&self.on_params)?;
                        let default = if has_into {
                            quote! { ::core::convert::Into::into((|| #default)()) }
                        } else {
                            quote! { #default }
                        };

                        Result::<_>::Ok(quote! { .unwrap_or_else(|| #default) })
                    })
                    .unwrap_or_else(|| Ok(quote! { .unwrap_or_default() }));

                Some(default)
            })
            .map(Option::transpose)
            .transpose()?
            .map(|default| {
                let ident = &member.norm_ident;
                quote! {
                    self.#ident #default
                }
            })
            .unwrap_or_else(|| {
                let ident = &member.norm_ident;
                quote! {
                    self.#ident.into_inner()
                }
            });

        Ok(expr)
    }

    /// Name of the dummy struct that is generated just to give a name for
    /// the member in the error message when `IntoSet` trait is not implemented.
    fn member_label(&self, member: &NamedMember) -> syn::Ident {
        quote::format_ident!(
            "{}__{}",
            self.builder_type.ident.raw_name(),
            member.public_ident()
        )
    }

    fn finish_method(&self) -> Result<TokenStream2> {
        let members_vars_decls = self
            .members
            .iter()
            .map(|member| {
                let expr = self.member_expr(member)?;
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

                Ok(quote! {
                    let #var_ident: #ty = #expr;
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let body = &self.finish_func.body.generate(&self.members);
        let asyncness = &self.finish_func.asyncness;
        let unsafety = &self.finish_func.unsafety;
        let must_use = &self.finish_func.must_use;
        let attrs = &self.finish_func.attrs;
        let vis = &self.vis;
        let finish_func_ident = &self.finish_func.ident;
        let output = &self.finish_func.output;

        let where_bounds = self
            .named_members()
            .filter(|member| !member.is_optional())
            .map(|member| {
                let member_pascal = &member.norm_ident_pascal;
                quote! {
                    BuilderState::#member_pascal: ::bon::private::IsSet
                }
            });

        let finish_fn_params = self
            .members
            .iter()
            .filter_map(Member::as_finish_fn_arg)
            .map(|member| member.fn_input_param(&self.on_params))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
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
            #vis #asyncness #unsafety fn #finish_func_ident(
                self,
                #(#finish_fn_params,)*
            ) #output
            where
                #(#where_bounds,)*
            {
                #(#members_vars_decls)*
                #body
            }
        })
    }

    fn setter_methods(&self) -> Result<(TokenStream2, TokenStream2)> {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let where_clause = &self.generics.where_clause;

        let state_type_vars = self
            .named_members()
            .map(|member| &member.generic_var_ident)
            .collect::<Vec<_>>();

        let allows = allow_warnings_on_member_types();

        let next_state_trait_ident =
            quote::format_ident!("__{}SetMember", builder_ident.raw_name());

        let next_states_decls = self.named_members().map(|member| {
            let member_pascal = &member.norm_ident_pascal;
            quote! {
                type #member_pascal;
            }
        });

        let setters = self
            .named_members()
            .map(|member| {
                let state_types = self.named_members().map(|other_member| {
                    if other_member.orig_ident == member.orig_ident {
                        let ty = member.set_state_type_param();
                        quote!(::bon::private::Set<#ty>)
                    } else {
                        other_member.generic_var_ident.to_token_stream()
                    }
                });

                let member_pascal = &member.norm_ident_pascal;

                let next_state = quote! {
                    #builder_ident<
                        #(#generic_args,)*
                        (#(#state_types,)*)
                    >
                };

                let return_type = SettersReturnType {
                    doc_true: quote!(<Self as #next_state_trait_ident>::#member_pascal),
                    doc_false: next_state.clone(),
                };

                let setter_methods =
                    MemberSettersCtx::new(self, member, return_type).setter_methods()?;

                let next_state = quote!(type #member_pascal = #next_state;);

                Ok((setter_methods, next_state))
            })
            .collect::<Result<Vec<_>>>()?;
        let next_states_defs = setters.iter().map(|(_, next_state)| next_state);

        let items_for_rustdoc = quote! {
            // This item is under `cfg(doc)` because it's used only to make the
            // documentation less noisy (see `SettersReturnType` for more info).
            #[cfg(doc)]
            trait #next_state_trait_ident {
                #(#next_states_decls)*
            }

            // This item is under `cfg(doc)` because it's used only to make the
            // documentation less noisy (see `SettersReturnType` for more info).
            #[cfg(doc)]
            #allows
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #(#state_type_vars,)*
            >
                #next_state_trait_ident
            for
                #builder_ident<
                    #(#generic_args,)*
                    (#(#state_type_vars,)*)
                >
            #where_clause
            {
                #(#next_states_defs)*
            }
        };

        let setter_methods = setters
            .into_iter()
            .map(|(setter_methods, _)| setter_methods)
            .concat();

        Ok((setter_methods, items_for_rustdoc))
    }
}

pub(crate) fn generic_param_to_arg(param: &syn::GenericParam) -> syn::GenericArgument {
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

fn allow_warnings_on_member_types() -> TokenStream2 {
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

/// Validates the docs for the presence of `Self` mentions to prevent users from
/// shooting themselves in the foot where they would think that `Self` resolves
/// to the current item the docs were placed on, when in fact the docs are moved
/// to a different context where `Self` has a different meaning.
fn reject_self_mentions_in_docs(context: &'static str, attrs: &[syn::Attribute]) -> Result {
    for attr in attrs {
        let doc = match attr.as_doc() {
            Some(doc) => doc,
            _ => continue,
        };

        let doc = match &doc {
            syn::Expr::Lit(doc) => doc,
            _ => continue,
        };

        let doc = match &doc.lit {
            syn::Lit::Str(doc) => doc,
            _ => continue,
        };

        let self_references = ["[`Self`]", "[Self]"];

        if self_references
            .iter()
            .any(|self_ref| doc.value().contains(self_ref))
        {
            bail!(
                &doc.span(),
                "the documentation should not reference `Self` because it will \
                be moved to the {context} where `Self` changes meaning, which \
                may confuse the reader of this code; use explicit type names instead.",
            );
        }
    }

    Ok(())
}
