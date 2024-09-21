use super::builder_params::BuilderDerives;
use super::BuilderGenCtx;
use crate::builder::builder_gen::Member;
use crate::util::prelude::*;
use darling::ast::GenericParamExt;
use quote::quote;

impl BuilderGenCtx {
    pub(crate) fn builder_derives(&self) -> TokenStream2 {
        let BuilderDerives { clone, debug } = &self.builder_type.derives;

        let mut tokens = TokenStream2::new();

        if clone.is_present() {
            tokens.extend(self.derive_clone());
        }

        if debug.is_present() {
            tokens.extend(self.derive_debug());
        }

        tokens
    }

    fn builder_component_types(&self) -> impl Iterator<Item = &'_ syn::Type> {
        let receiver_ty = self
            .receiver()
            .map(|receiver| &receiver.without_self_keyword);

        let member_types = self.named_members().map(|member| &member.norm_ty);

        std::iter::empty()
            .chain(receiver_ty)
            .chain(member_types)
            .map(Box::as_ref)
    }

    /// We follow the logic of the standard `#[derive(...)]` macros such as `Clone` and `Debug`.
    /// They add bounds of their respective traits to every generic type parameter on the struct
    /// without trying to analyze if that bound is actually required for the derive to work, so
    /// it's a conservative approach.
    fn where_clause_for_derive(&self, target_trait_bounds: &TokenStream2) -> TokenStream2 {
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

    fn derive_clone(&self) -> TokenStream2 {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;

        let clone = quote!(::core::clone::Clone);

        let clone_receiver = self.receiver().map(|_| {
            quote! {
                __private_receiver: #clone::clone(&self.__private_receiver),
            }
        });

        let clone_start_fn_args = self.start_fn_args().next().map(|_| {
            quote! {
                __private_start_fn_args: #clone::clone(&self.__private_start_fn_args),
            }
        });

        let where_clause = self.where_clause_for_derive(&clone);
        let builder_mod_ident = &self.builder_mod.ident;

        let clone_named_members = self.named_members().map(|member| {
            let member_index = &member.index;

            // The type hints here are necessary to get better error messages
            // that point directly to the types that don't implement `Clone`
            // in the input code using the span info from the type hints.
            let clone_fn = member
                .as_optional_norm_ty()
                .map(|ty| quote!(clone_optional_member::<#ty>))
                .unwrap_or_else(|| {
                    let ty = &member.norm_ty;
                    quote!(clone_required_member::<_, #ty>)
                });

            quote! {
                ::bon::private::derives::#clone_fn(
                    &self.__private_named_members.#member_index
                )
            }
        });

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderTypeState: #builder_mod_ident::State
            >
            #clone for #builder_ident<
                #(#generic_args,)*
                BuilderTypeState
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
                        __private_named_members: (
                            #( #clone_named_members, )*
                        ),
                    }
                }
            }
        }
    }

    fn derive_debug(&self) -> TokenStream2 {
        let format_members = self.members.iter().filter_map(|member| {
            match member {
                Member::Named(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.public_ident().to_string();
                    let member_pascal = &member.norm_ident_pascal;

                    let debug_fn = member.as_optional_norm_ty()
                        .map(|ty| quote!(debug_optional_member::<#ty>))
                        .unwrap_or_else(|| {
                            let ty = &member.norm_ty;
                            quote!(debug_required_member::<_, #ty>)
                        });

                    Some(quote! {
                        // Skip members that are not set to reduce noise
                        if <BuilderTypeState::#member_pascal as ::bon::private::MemberState>::is_set() {
                            output.field(
                                #member_ident_str,
                                ::bon::private::derives::#debug_fn(
                                    &self.__private_named_members.#member_index
                                )
                            );
                        }
                    })
                }
                Member::StartFnArg(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.base.ident.to_string();
                    Some(quote! {
                        output.field(
                            #member_ident_str,
                            &self.__private_start_fn_args.#member_index
                        );
                    })
                }

                // The values for these members are computed only in the finishing
                // function where the builder is consumed, and they aren't stored
                // in the builder itself.
                Member::FinishFnArg(_) | Member::Skipped(_) => None,
            }
        });

        let format_receiver = self.receiver().map(|_| {
            quote! {
                output.field("self", &self.__private_receiver);
            }
        });

        let debug = quote!(::core::fmt::Debug);
        let where_clause = self.where_clause_for_derive(&debug);
        let builder_mod_ident = &self.builder_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let builder_ident_str = builder_ident.to_string();

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                BuilderTypeState: #builder_mod_ident::State
            >
            #debug for #builder_ident<
                #(#generic_args,)*
                BuilderTypeState
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
