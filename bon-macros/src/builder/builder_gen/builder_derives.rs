use super::top_level_config::{DeriveConfig, DerivesConfig};
use super::BuilderGenCtx;
use crate::builder::builder_gen::Member;
use crate::util::prelude::*;
use darling::ast::GenericParamExt;

impl BuilderGenCtx {
    pub(crate) fn builder_derives(&self) -> TokenStream {
        let DerivesConfig { clone, debug } = &self.builder_type.derives;

        let mut tokens = TokenStream::new();

        if let Some(derive) = clone {
            tokens.extend(self.derive_clone(derive));
        }

        if let Some(derive) = debug {
            tokens.extend(self.derive_debug(derive));
        }

        tokens
    }

    /// We follow the logic of the standard `#[derive(...)]` macros such as `Clone` and `Debug`.
    /// They add bounds of their respective traits to every generic type parameter on the struct
    /// without trying to analyze if that bound is actually required for the derive to work, so
    /// it's a conservative approach.
    ///
    /// However, the user can also override these bounds using the `bounds(...)` attribute for
    /// the specific derive.
    fn where_clause_for_derive(
        &self,
        target_trait_bounds: &TokenStream,
        derive: &DeriveConfig,
    ) -> TokenStream {
        let derive_specific_predicates = derive
            .bounds
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(|| {
                let bounds = self
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

                quote! {
                    #( #bounds, )*
                }
            });

        let inherent_item_predicates = self.generics.where_clause_predicates();

        quote! {
            where
                #( #inherent_item_predicates, )*
                #derive_specific_predicates
        }
    }

    fn derive_clone(&self, derive: &DeriveConfig) -> TokenStream {
        let bon = &self.bon;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;

        let phantom_field = &self.ident_pool.phantom;
        let receiver_field = &self.ident_pool.receiver;
        let start_fn_args_field = &self.ident_pool.start_fn_args;
        let named_members_field = &self.ident_pool.named_members;

        let clone = quote!(::core::clone::Clone);

        let clone_receiver = self.receiver().map(|receiver| {
            let ty = &receiver.without_self_keyword;
            quote! {
                #receiver_field: <#ty as #clone>::clone(&self.#receiver_field),
            }
        });

        let clone_start_fn_args = self.start_fn_args().next().map(|_| {
            let types = self.start_fn_args().map(|arg| &arg.base.ty.norm);
            let indices = self.start_fn_args().map(|arg| &arg.index);

            quote! {
                // We clone named members individually instead of cloning
                // the entire tuple to improve error messages in case if
                // one of the members doesn't implement `Clone`. This avoids
                // a sentence that say smth like
                // ```
                // required for `(...big type...)` to implement `Clone`
                // ```
                #start_fn_args_field: (
                    #( <#types as #clone>::clone(&self.#start_fn_args_field.#indices), )*
                ),
            }
        });

        let where_clause = self.where_clause_for_derive(&clone, derive);
        let state_mod = &self.state_mod.ident;

        let clone_named_members = self.named_members().map(|member| {
            let member_index = &member.index;

            // The type hint here is necessary to get better error messages
            // that point directly to the type that doesn't implement `Clone`
            // in the input code using the span info from the type hint.
            let ty = member.underlying_norm_ty();

            quote! {
                #bon::__::derives::clone_member::<#ty>(
                    &self.#named_members_field.#member_index
                )
            }
        });

        let state_var = &self.state_var;

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::State
            >
            #clone for #builder_ident<
                #(#generic_args,)*
                #state_var
            >
            #where_clause
            {
                fn clone(&self) -> Self {
                    Self {
                        #phantom_field: ::core::marker::PhantomData,
                        #clone_receiver
                        #clone_start_fn_args

                        // We clone named members individually instead of cloning
                        // the entire tuple to improve error messages in case if
                        // one of the members doesn't implement `Clone`. This avoids
                        // a sentence that say smth like
                        // ```
                        // required for `(...big type...)` to implement `Clone`
                        // ```
                        #named_members_field: ( #( #clone_named_members, )* ),
                    }
                }
            }
        }
    }

    fn derive_debug(&self, derive: &DeriveConfig) -> TokenStream {
        let receiver_field = &self.ident_pool.receiver;
        let start_fn_args_field = &self.ident_pool.start_fn_args;
        let named_members_field = &self.ident_pool.named_members;
        let bon = &self.bon;

        let format_members = self.members.iter().filter_map(|member| {
            match member {
                Member::Named(member) => {
                    let member_index = &member.index;
                    let member_ident_str = &member.name.snake_raw_str;
                    let member_ty = member.underlying_norm_ty();
                    Some(quote! {
                        if let Some(value) = &self.#named_members_field.#member_index {
                            output.field(
                                #member_ident_str,
                                #bon::__::derives::as_dyn_debug::<#member_ty>(value)
                            );
                        }
                    })
                }
                Member::StartFnArg(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.base.ident.to_string();
                    let member_ty = &member.base.ty.norm;
                    Some(quote! {
                        output.field(
                            #member_ident_str,
                            #bon::__::derives::as_dyn_debug::<#member_ty>(
                                &self.#start_fn_args_field.#member_index
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
                    #bon::__::derives::as_dyn_debug::<#ty>(
                        &self.#receiver_field
                    )
                );
            }
        });

        let debug = quote!(::core::fmt::Debug);
        let where_clause = self.where_clause_for_derive(&debug, derive);
        let state_mod = &self.state_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let state_var = &self.state_var;
        let builder_ident_str = builder_ident.to_string();

        quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::State
            >
            #debug for #builder_ident<
                #(#generic_args,)*
                #state_var
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
