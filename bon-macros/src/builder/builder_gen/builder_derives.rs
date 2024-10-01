use super::builder_params::BuilderDerives;
use super::BuilderGenCtx;
use crate::builder::builder_gen::Member;
use crate::util::prelude::*;
use darling::ast::GenericParamExt;

impl BuilderGenCtx {
    pub(crate) fn builder_derives(&self) -> TokenStream {
        let BuilderDerives { clone, debug } = &self.builder_type.derives;

        let mut tokens = TokenStream::new();

        if clone.is_present() {
            tokens.extend(self.derive_clone());
        }

        if debug.is_present() {
            tokens.extend(self.derive_debug());
        }

        tokens
    }

    /// We follow the logic of the standard `#[derive(...)]` macros such as `Clone` and `Debug`.
    /// They add bounds of their respective traits to every generic type parameter on the struct
    /// without trying to analyze if that bound is actually required for the derive to work, so
    /// it's a conservative approach.
    fn where_clause_for_derive(&self, target_trait_bounds: &TokenStream) -> TokenStream {
        let target_trait_bounds_predicates = self
            .generics
            .decl_without_defaults
            .iter()
            .filter_map(syn::GenericParam::as_type_param)
            .map(|param| {
                let ident = &param.ident;
                quote! {
                    #ident: #target_trait_bounds
                }
            });

        let base_predicates = self.generics.where_clause_predicates();

        quote! {
            where
                #( #base_predicates, )*
                #( #target_trait_bounds_predicates, )*
        }
    }

    fn derive_clone(&self) -> TokenStream {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;

        let clone = quote!(::core::clone::Clone);

        let clone_receiver = self.receiver().map(|receiver| {
            let ty = &receiver.without_self_keyword;
            quote! {
                __private_receiver: <#ty as #clone>::clone(&self.__private_receiver),
            }
        });

        let clone_start_fn_args = self.start_fn_args().next().map(|_| {
            let clone_start_fn_args = self.start_fn_args().map(|arg| {
                let ty = &arg.base.norm_ty;
                let index = &arg.index;
                quote! {
                    <#ty as #clone>::clone(&self.__private_start_fn_args.#index)
                }
            });

            quote! {
                __private_start_fn_args: ( #(#clone_start_fn_args,)* ),
            }
        });

        let where_clause = self.where_clause_for_derive(&clone);
        let state_mod = &self.state_mod.ident;

        let clone_named_members = self.named_members().map(|member| {
            let member_index = &member.index;

            // The type hints here are necessary to get better error messages
            // that point directly to the types that don't implement `Clone`
            // in the input code using the span info from the type hints.
            let ty = member.as_optional_norm_ty().unwrap_or(&member.norm_ty);

            quote! {
                ::bon::private::derives::clone_member::<#ty>(
                    &self.__private_named_members.#member_index
                )
            }
        });

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderState: #state_mod::State
            >
            #clone for #builder_ident<
                #(#generic_args,)*
                BuilderState
            >
            #where_clause
            {
                fn clone(&self) -> Self {
                    Self {
                        __private_phantom: ::core::marker::PhantomData,
                        #clone_receiver
                        #clone_start_fn_args

                        // We clone named members individually instead of cloning
                        // the entire tuple to improve error messages in case if
                        // one of the members doesn't implement `Clone`. This avoids
                        // a sentence that say smth like
                        // ```
                        // required for `(...huge tuple type...)` to implement `Clone`
                        // ```
                        __private_named_members: ( #( #clone_named_members, )* ),
                    }
                }
            }
        }
    }

    fn derive_debug(&self) -> TokenStream {
        let format_members = self.members.iter().filter_map(|member| {
            match member {
                Member::Named(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.public_ident().to_string();
                    let member_ty = member.as_optional_norm_ty().unwrap_or(&member.norm_ty);
                    Some(quote! {
                        if let ::core::option::Option::Some(value) = &self.__private_named_members.#member_index {
                            output.field(
                                #member_ident_str,
                                ::bon::private::derives::as_dyn_debug::<#member_ty>(value)
                            );
                        }
                    })
                }
                Member::StartFnArg(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.base.ident.to_string();
                    let member_ty = &member.base.norm_ty;
                    Some(quote! {
                        output.field(
                            #member_ident_str,
                            ::bon::private::derives::as_dyn_debug::<#member_ty>(
                                &self.__private_start_fn_args.#member_index
                            )
                        );
                    })
                }

                // The values for these members are computed only in the finishing
                // function where the builder is consumed, and they aren't stored
                // in the builder itself.
                Member::FinishFnArg(_) | Member::Skipped(_) => None,
            }
        });

        let format_receiver = self.receiver().map(|receiver| {
            let ty = &receiver.without_self_keyword;
            quote! {
                output.field(
                    "self",
                    ::bon::private::derives::as_dyn_debug::<#ty>(
                        &self.__private_receiver
                    )
                );
            }
        });

        let debug = quote!(::core::fmt::Debug);
        let where_clause = self.where_clause_for_derive(&debug);
        let state_mod = &self.state_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let builder_ident_str = builder_ident.to_string();

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderState: #state_mod::State
            >
            #debug for #builder_ident<
                #(#generic_args,)*
                BuilderState
            >
            #where_clause
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut output = f.debug_struct(#builder_ident_str);

                    #format_receiver
                    #(#format_members)*

                    output.finish()
                }
            }
        }
    }
}
