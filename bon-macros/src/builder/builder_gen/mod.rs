mod member;
mod setter_methods;

pub(crate) mod input_func;
pub(crate) mod input_struct;

use member::*;

use itertools::Itertools;
use prox::prelude::*;
use quote::quote;

pub(crate) struct ReceiverCtx {
    pub(crate) with_self_ty: syn::Receiver,
    pub(crate) without_self_ty: Box<syn::Type>,
}

pub(crate) struct BuilderGenCtx {
    pub(crate) members: Vec<Member>,

    pub(crate) generics: Generics,
    pub(crate) vis: syn::Visibility,
    pub(crate) receiver: Option<ReceiverCtx>,

    pub(crate) start_func: StartFunc,
    pub(crate) finish_func: FinishFunc,

    pub(crate) builder_ident: syn::Ident,
    pub(crate) builder_private_impl_ident: syn::Ident,
    pub(crate) builder_state_trait_ident: syn::Ident,
}

pub(crate) struct FinishFunc {
    pub(crate) ident: syn::Ident,
    pub(crate) unsafety: Option<syn::Token![unsafe]>,
    pub(crate) asyncness: Option<syn::Token![async]>,
    pub(crate) body: Box<dyn FinishFuncBody>,
    pub(crate) output: syn::ReturnType,
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
    /// Generate `finish` function body from ready-made expressions.
    fn gen(&self, member_exprs: &[MemberExpr<'_>]) -> TokenStream2;
}

pub(crate) struct MemberExpr<'a> {
    pub(crate) member: &'a Member,
    pub(crate) expr: TokenStream2,
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
    fn member_idents(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.members.iter().map(|member| member.ident.clone())
    }

    fn member_assoc_type_idents(&self) -> impl Iterator<Item = &syn::Ident> {
        self.members
            .iter()
            .map(|member| &member.state_assoc_type_ident)
    }

    fn unset_state_types(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        self.members.iter().map(|arg| arg.unset_state_type())
    }

    fn generic_args(&self) -> impl Iterator<Item = syn::GenericArgument> + '_ {
        self.generics.params.iter().map(generic_param_to_arg)
    }

    pub(crate) fn output(self) -> Result<MacroOutput> {
        let start_func = self.start_func();
        let builder_state_trait_decl = self.builder_state_trait_decl();
        let builder_decl = self.builder_decl();
        let call_method_impl = self.finish_method_impl()?;
        let setter_methods_impls = self.setter_methods_impls()?;

        let other_items = quote! {
            #builder_state_trait_decl
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

    fn start_func(&self) -> syn::ItemFn {
        let builder_ident = &self.builder_ident;

        let docs = &self.start_func.attrs;
        let vis = self.start_func.vis.as_ref().unwrap_or(&self.vis);

        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let start_func_ident = &self.start_func.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generate code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics = self.start_func_generics();

        let generics_decl = &generics.params;
        let where_clause = &generics.where_clause;
        let generic_args = self.generic_args();

        let member_idents = self.member_idents();

        let receiver = self
            .receiver
            .as_ref()
            .map(|receiver| &receiver.with_self_ty);
        let receiver_field_init = self.receiver.as_ref().map(|receiver| {
            let self_token = &receiver.with_self_ty.self_token;
            quote! {
                receiver: #self_token,
            }
        });

        let func = quote! {
            #(#docs)*
            #vis fn #start_func_ident<#(#generics_decl),*>(
                #receiver
            ) -> #builder_ident<
                #(#generic_args,)*
            >
            #where_clause
            {
                #builder_ident {
                    __private_impl: #builder_private_impl_ident {
                        _phantom: ::core::marker::PhantomData,
                        #receiver_field_init
                        #( #member_idents: ::std::default::Default::default(), )*
                    }
                }
            }
        };

        syn::parse_quote!(#func)
    }

    fn phantom_data(&self) -> TokenStream2 {
        let member_types = self.members.iter().map(|member| &member.ty);

        quote! {
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
                #(#member_types,)*

                // A special case of zero members requires storing `__State` in phantom data
                // otherwise it would be reported as an unused type parameter.
                __State,
            )>
        }
    }

    fn builder_state_trait_decl(&self) -> TokenStream2 {
        let trait_ident = &self.builder_state_trait_ident;
        let assoc_types_idents = self.member_assoc_type_idents().collect_vec();
        let vis = &self.vis;

        quote! {
            #[doc(hidden)]
            #vis trait #trait_ident {
                #( type #assoc_types_idents; )*
            }

            impl<#(#assoc_types_idents),*> #trait_ident for (#(#assoc_types_idents,)*) {
                #( type #assoc_types_idents = #assoc_types_idents; )*
            }
        }
    }

    fn builder_decl(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let generics_decl = &self.generics.params;
        let where_clause = &self.generics.where_clause;
        let generic_args = self.generic_args();
        let unset_state_types = self.unset_state_types();
        let phantom_data = self.phantom_data();

        let receiver_field = self.receiver.as_ref().map(|receiver| {
            let ty = &receiver.without_self_ty;
            quote! {
                receiver: #ty,
            }
        });

        let members = self.members.iter().map(|member| {
            let ident = &member.ident;
            let assoc_type_ident = &member.state_assoc_type_ident;
            quote! {
                #ident: __State::#assoc_type_ident,
            }
        });

        quote! {
            #vis struct #builder_ident<
                #(#generics_decl,)*
                __State: #builder_state_trait_ident = (#(#unset_state_types,)*),
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
                /// ```rustdoc_hidden
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
                /// ```rustdoc_hidden
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
                _phantom: #phantom_data,
                #receiver_field
                #(#members)*
            }
        }
    }

    fn member_expr<'f>(&self, member: &'f Member) -> Result<MemberExpr<'f>> {
        let maybe_default = member
            .as_optional()
            // For `Option` members we don't need any `unwrap_or_[else/default]`.
            // We pass them directly to the function unchanged.
            .filter(|_| !member.ty.is_option())
            .map(|_| {
                member
                    .params
                    .default
                    .as_ref()
                    .and_then(|val| val.as_ref().as_ref())
                    .map(|default| {
                        let qualified_for_into =
                            self.member_qualifies_for_into(member, &member.ty)?;
                        let default = if qualified_for_into {
                            quote! { std::convert::Into::into((|| #default)()) }
                        } else {
                            quote! { #default }
                        };

                        Result::<_>::Ok(quote! { .unwrap_or_else(|| #default) })
                    })
                    .unwrap_or_else(|| Ok(quote! { .unwrap_or_default() }))
            })
            .transpose()?;

        let member_ident = &member.ident;

        let expr = quote! {
            bon::private::IntoSet::into_set(self.__private_impl.#member_ident)
                .into_inner()
                #maybe_default
        };

        Ok(MemberExpr { member, expr })
    }

    fn finish_method_impl(&self) -> Result<TokenStream2> {
        let member_exprs: Vec<_> = self
            .members
            .iter()
            .map(|member| self.member_expr(member))
            .try_collect()?;

        let body = &self.finish_func.body.gen(&member_exprs);
        let asyncness = &self.finish_func.asyncness;
        let unsafety = &self.finish_func.unsafety;
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let finish_func_ident = &self.finish_func.ident;
        let output = &self.finish_func.output;
        let generics_decl = &self.generics.params;
        let generic_builder_args = self.generic_args();
        let where_clause_predicates = self
            .generics
            .where_clause
            .as_ref()
            .into_iter()
            .flat_map(|where_clause| &where_clause.predicates);

        let state_where_predicates = self.members.iter().map(|member| {
            let member_assoc_type_ident = &member.state_assoc_type_ident;
            let set_state_type_param = member.set_state_type_param();
            quote! {
                __State::#member_assoc_type_ident:
                    bon::private::IntoSet<#set_state_type_param>
            }
        });

        Ok(quote! {
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
                #( #state_where_predicates, )*
            {
                /// Finishes building and performs the requested action.
                #vis #asyncness #unsafety fn #finish_func_ident(self) #output {
                    #body
                }
            }
        })
    }

    fn setter_methods_impls(&self) -> Result<TokenStream2> {
        self.members
            .iter()
            .map(|member| self.setter_methods_impls_for_member(member))
            .collect()
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
            prox::bail!(
                &doc.span(),
                "The documentation for the member should not reference `Self` \
                because it will be moved to the builder struct context where \
                `Self` changes meaning. Use explicit type names instead.",
            );
        }
    }

    Ok(())
}
