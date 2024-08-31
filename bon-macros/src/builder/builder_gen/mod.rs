mod member;
mod setter_methods;

pub(crate) mod input_func;
pub(crate) mod input_struct;

use member::*;

use super::params::ConditionalParams;
use crate::util::prelude::*;
use quote::{quote, ToTokens};
use setter_methods::{MemberSettersCtx, SettersReturnType};

pub(crate) struct AssocMethodReceiverCtx {
    pub(crate) with_self_keyword: syn::Receiver,
    pub(crate) without_self_keyword: Box<syn::Type>,
}

pub(crate) struct AssocMethodCtx {
    /// The `Self` type of the impl block. It doesn't contain any nested
    /// `Self` keywords in it. This is prohibited by Rust's syntax itself.
    pub(crate) self_ty: Box<syn::Type>,

    /// Present only if the method has a receiver, i.e. `self` or `&self` or
    /// `&mut self` or `self: ExplicitType`.
    pub(crate) receiver: Option<AssocMethodReceiverCtx>,
}

pub(crate) struct BuilderGenCtx {
    pub(crate) members: Vec<Member>,

    pub(crate) conditional_params: Vec<ConditionalParams>,

    pub(crate) generics: Generics,
    pub(crate) vis: syn::Visibility,
    pub(crate) assoc_method_ctx: Option<AssocMethodCtx>,

    pub(crate) start_func: StartFunc,
    pub(crate) finish_func: FinishFunc,

    pub(crate) builder_ident: syn::Ident,
}

pub(crate) struct FinishFunc {
    pub(crate) ident: syn::Ident,
    pub(crate) unsafety: Option<syn::Token![unsafe]>,
    pub(crate) asyncness: Option<syn::Token![async]>,
    /// <https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute>
    pub(crate) must_use: Option<syn::Attribute>,
    pub(crate) body: Box<dyn FinishFuncBody>,
    pub(crate) output: syn::ReturnType,
    pub(crate) docs: String,
}

pub(crate) struct StartFunc {
    pub(crate) ident: syn::Ident,
    pub(crate) attrs: Vec<syn::Attribute>,

    /// Overrides the common generics
    pub(crate) generics: Option<Generics>,

    /// If present overrides the automatic visibility
    pub(crate) vis: Option<syn::Visibility>,
}

pub(crate) trait FinishFuncBody {
    /// Generate `finish` function body from ready-made variables.
    /// The generated function body may assume that there are variables
    /// named the same as the members in scope.
    fn generate(&self, members: &[Member]) -> TokenStream2;
}

pub(crate) struct Generics {
    pub(crate) params: Vec<syn::GenericParam>,
    pub(crate) where_clause: Option<syn::WhereClause>,
}

pub(crate) struct MacroOutput {
    pub(crate) start_func: syn::ItemFn,
    pub(crate) other_items: TokenStream2,
}

impl BuilderGenCtx {
    fn regular_members(&self) -> impl Iterator<Item = &RegularMember> {
        self.members.iter().filter_map(Member::as_regular)
    }

    fn generic_args(&self) -> impl Iterator<Item = syn::GenericArgument> + '_ {
        self.generics.params.iter().map(generic_param_to_arg)
    }

    pub(crate) fn output(self) -> Result<MacroOutput> {
        let start_func = self.start_func();
        let builder_decl = self.builder_decl();
        let call_method_impl = self.finish_method_impl()?;
        let setter_methods_impls = self.setter_methods_impls()?;

        let other_items = quote! {
            #builder_decl
            #call_method_impl
            #setter_methods_impls
        };

        Ok(MacroOutput {
            start_func,
            other_items,
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
            .conditional_params
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

    fn start_func(&self) -> syn::ItemFn {
        let builder_ident = &self.builder_ident;

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

        let generics_decl = &generics.params;
        let where_clause = &generics.where_clause;
        let generic_args = self.generic_args();

        let receiver = self
            .assoc_method_ctx
            .as_ref()
            .and_then(|ctx| ctx.receiver.as_ref());

        let receiver_field_init = receiver.map(|receiver| {
            let self_token = &receiver.with_self_keyword.self_token;
            quote! {
                __private_receiver: #self_token,
            }
        });

        let receiver = receiver.map(|receiver| &receiver.with_self_keyword);

        let unset_state_literals = self
            .regular_members()
            .map(|_| quote!(::bon::private::Unset));

        let ide_hints = self.ide_hints();

        let func = quote! {
            #(#docs)*
            #[inline(always)]
            #vis fn #start_func_ident<#(#generics_decl),*>(
                #receiver
            ) -> #builder_ident<#(#generic_args,)*>
            #where_clause
            {
                #ide_hints

                #builder_ident {
                    __private_phantom: ::core::marker::PhantomData,
                    #receiver_field_init
                    __private_members: (#( #unset_state_literals, )*)
                }
            }
        };

        syn::parse_quote!(#func)
    }

    fn phantom_data(&self) -> TokenStream2 {
        let member_types = self.members.iter().map(Member::norm_ty);
        let receiver_ty = self
            .assoc_method_ctx
            .as_ref()
            .map(|ctx| ctx.self_ty.as_ref());

        let generic_args: Vec<_> = self.generic_args().collect();
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

                // A special case of zero members requires storing `_State` in phantom data
                // otherwise it would be reported as an unused type parameter.
                ::core::marker::PhantomData<_State>
            )>
        }
    }

    fn builder_decl(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let generics_decl = &self.generics.params;
        let where_clause = &self.generics.where_clause;
        let phantom_data = self.phantom_data();

        let private_field_doc = "\
            Please don't touch this field. It's an implementation \
            detail that is exempt from the API stability guarantees. \
            This field couldn't be hidden using Rust's privacy syntax. \
            The details about this are described in [the blog post]\
            (https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        ";

        let receiver_field = self.assoc_method_ctx.as_ref().and_then(|receiver| {
            let ty = &receiver.receiver.as_ref()?.without_self_keyword;
            Some(quote! {
                #[doc = #private_field_doc]
                __private_receiver: #ty,
            })
        });

        let must_use_message = format!(
            "the builder does nothing until you call `{}()` on it to finish building",
            self.finish_func.ident
        );

        let docs = format!(
            "Use builder syntax to set the required parameters and finish \
            by calling the method [`Self::{}()`].",
            self.finish_func.ident
        );

        let allows = allow_warnings_on_member_types();

        let initial_state_type_alias_ident =
            quote::format_ident!("__{}InitialState", builder_ident.raw_name());

        let unset_state_types = self
            .regular_members()
            .map(|_| quote!(::bon::private::Unset));

        quote! {
            // This type alias exists just to shorten the type signature of
            // the default generic argument of the builder struct. It's not
            // really important for users to see what this type alias expands to.
            //
            // If they want to see how "bon works" they should just expand the
            // macro manually where they'll see this type alias.
            #[doc(hidden)]
            #vis type #initial_state_type_alias_ident = (#(#unset_state_types,)*);

            #[must_use = #must_use_message]
            #[doc = #docs]
            #allows
            #vis struct #builder_ident<
                #(#generics_decl,)*
                _State = #initial_state_type_alias_ident
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

                #[doc = #private_field_doc]
                __private_members: _State
            }
        }
    }

    fn member_expr(&self, member: &Member) -> Result<TokenStream2> {
        let member = match member {
            Member::Regular(member) => member,
            Member::Skipped(member) => {
                let expr = member
                    .value
                    .as_ref()
                    .as_ref()
                    .map(|value| quote! { #value })
                    .unwrap_or_else(|| quote! { ::core::default::Default::default() });

                return Ok(expr);
            }
        };

        let maybe_default = member
            .as_optional_norm_ty()
            // For `Option` members we don't need any `unwrap_or_[else/default]`.
            // The implementation of `From<Unset> for Set<Option<T>>` already
            // returns an `Option<T>`.
            .filter(|_| !member.norm_ty.is_option())
            .map(|_| {
                member
                    .param_default()
                    .flatten()
                    .map(|default| {
                        let has_into = member.param_into(&self.conditional_params)?;
                        let default = if has_into {
                            quote! { ::core::convert::Into::into((|| #default)()) }
                        } else {
                            quote! { #default }
                        };

                        Result::<_>::Ok(quote! { .unwrap_or_else(|| #default) })
                    })
                    .unwrap_or_else(|| Ok(quote! { .unwrap_or_default() }))
            })
            .transpose()?;

        let index = &member.index;

        let expr = if member.is_optional() {
            quote! {
                ::core::convert::Into::<::bon::private::Set<_>>::into(
                    self.__private_members.#index
                )
                .0
                #maybe_default
            }
        } else {
            quote! {
                self.__private_members.#index.0
            }
        };

        Ok(expr)
    }

    fn finish_method_impl(&self) -> Result<TokenStream2> {
        let members_vars_decls = self
            .members
            .iter()
            .map(|member| {
                let expr = self.member_expr(member)?;
                let var_ident = member.ident();

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
        let docs = &self.finish_func.docs;
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let finish_func_ident = &self.finish_func.ident;
        let output = &self.finish_func.output;
        let generics_decl = &self.generics.params;
        let generic_builder_args = self.generic_args();
        let where_clause = &self.generics.where_clause;

        let builder_state_types = self.regular_members().map(|member| {
            if member.is_optional() {
                member.generic_var_ident.to_token_stream()
            } else {
                member.set_state_type()
            }
        });

        let optional_members_generics_decls = self
            .regular_members()
            // Only optional members can have two source states, that we
            // we need to represent with a trait bound. They can either
            // be `Unset` (setter never called) or `Set<T>` (setter was called)
            .filter(|member| member.is_optional())
            .map(|member| {
                let ident = &member.generic_var_ident;
                let set_state_type = member.set_state_type();
                Some(quote! {
                    #ident: ::core::convert::Into<#set_state_type>
                })
            });

        let allows = allow_warnings_on_member_types();

        Ok(quote! {
            #allows
            impl<
                #(#generics_decl,)*
                #(#optional_members_generics_decls,)*
            >
            #builder_ident<
                #(#generic_builder_args,)*
                (#(#builder_state_types,)*)
            >
            #where_clause
            {
                #[doc = #docs]
                #[inline(always)]
                #must_use
                #vis #asyncness #unsafety fn #finish_func_ident(self) #output {
                    #(#members_vars_decls)*
                    #body
                }
            }
        })
    }

    fn setter_methods_impls(&self) -> Result<TokenStream2> {
        let generics_decl = &self.generics.params;
        let generic_builder_args = self.generic_args().collect::<Vec<_>>();
        let builder_ident = &self.builder_ident;
        let where_clause = &self.generics.where_clause;

        let state_type_vars = self
            .regular_members()
            .map(|member| &member.generic_var_ident)
            .collect::<Vec<_>>();

        let allows = allow_warnings_on_member_types();

        let next_state_trait_ident =
            quote::format_ident!("__{}SetMember", builder_ident.raw_name());

        let next_states_decls = self.regular_members().map(|member| {
            let member_pascal = &member.ident_pascal;
            quote! {
                type #member_pascal;
            }
        });

        let setters = self
            .regular_members()
            .map(|member| {
                let state_types = self.regular_members().map(|other_member| {
                    if other_member.ident == member.ident {
                        member.set_state_type().to_token_stream()
                    } else {
                        other_member.generic_var_ident.to_token_stream()
                    }
                });

                let member_pascal = &member.ident_pascal;

                let next_state = quote! {
                    #builder_ident<
                        #(#generic_builder_args,)*
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

        let setter_methods = setters.iter().map(|(setter_methods, _)| setter_methods);
        let next_states_defs = setters.iter().map(|(_, next_state)| next_state);

        Ok(quote! {
            #[cfg(doc)]
            trait #next_state_trait_ident {
                #(#next_states_decls)*
            }

            #[cfg(doc)]
            #allows
            impl<
                #(#generics_decl,)*
                #(#state_type_vars,)*
            >
                #next_state_trait_ident
            for
                #builder_ident<
                    #(#generic_builder_args,)*
                    (#(#state_type_vars,)*)
                >
            #where_clause
            {
                #(#next_states_defs)*
            }

            #allows
            impl<
                #(#generics_decl,)*
                #(#state_type_vars,)*
            >
            #builder_ident<
                #(#generic_builder_args,)*
                (#(#state_type_vars,)*)
            >
            #where_clause
            {
                #(#setter_methods)*
            }
        })
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

fn reject_self_references_in_docs(docs: &[syn::Attribute]) -> Result {
    for doc in docs {
        let Some(doc) = &doc.as_doc() else { continue };

        let syn::Expr::Lit(doc) = &doc else { continue };

        let syn::Lit::Str(doc) = &doc.lit else {
            continue;
        };

        let self_references = ["[`Self`]", "[Self]"];

        if self_references
            .iter()
            .any(|self_ref| doc.value().contains(self_ref))
        {
            bail!(
                &doc.span(),
                "The documentation for the member should not reference `Self` \
                because it will be moved to the builder struct context where \
                `Self` changes meaning. Use explicit type names instead.",
            );
        }
    }

    Ok(())
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
