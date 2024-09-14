use super::BuilderGenCtx;
use crate::builder::builder_gen::Member;
use crate::util::prelude::*;
use quote::quote;

impl BuilderGenCtx {
    pub(crate) fn builder_derives(&self) -> TokenStream2 {
        let derives = match &self.builder_derives {
            Some(derives) => derives,
            None => return quote!(),
        };

        let mut tokens = TokenStream2::new();

        if derives.clone.is_present() {
            tokens.extend(self.derive_clone());
        }

        if derives.debug.is_present() {
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

    fn derive_clone(&self) -> TokenStream2 {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_ident;

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

        let builder_where_clause_predicates = self.generics.where_clause_predicates();

        let builder_component_types = self.builder_component_types();

        quote! {
            #[automatically_derived]
            impl <
                #(#generics_decl,)*
                ___State
            >
            #clone for #builder_ident <
                #(#generic_args,)*
                ___State
            >
            where
                #(#builder_where_clause_predicates,)*
                ___State: #clone,
            {
                fn clone(&self) -> Self {
                    #(::bon::private::assert_clone::<#builder_component_types>();)*
                    Self {
                        __private_phantom: ::core::marker::PhantomData,
                        #clone_receiver
                        #clone_start_fn_args
                        __private_named_members: self.__private_named_members.clone(),
                    }
                }
            }
        }
    }

    fn derive_debug(&self) -> TokenStream2 {
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_ident;

        let debug = quote!(::core::fmt::Debug);

        let format_receiver = self.receiver().map(|_| {
            quote! {
                output.field("self", &self.__private_receiver);
            }
        });

        let builder_where_clause_predicates = self.generics.where_clause_predicates();
        let builder_component_types = self.builder_component_types();

        let builder_ident_str = builder_ident.to_string();

        let state_type_vars = self
            .named_members()
            .map(|member| &member.generic_var_ident)
            .collect::<Vec<_>>();

        let format_members = self.members.iter().filter_map(|member| {
            match member {
                Member::Named(member) => {
                    let member_index = &member.index;
                    let member_ident_str = member.orig_ident.to_string();
                    Some(quote! {
                        // Skip members that are not set to reduce noise
                        if self.__private_named_members.#member_index.is_set() {
                            output.field(
                                #member_ident_str,
                                &self.__private_named_members.#member_index
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

        quote! {
            #[automatically_derived]
            impl <
                #(#generics_decl,)*
                #(#state_type_vars,)*
            >
            #debug for #builder_ident <
                #(#generic_args,)*
                (#(#state_type_vars,)*)
            >
            where
                #(#builder_where_clause_predicates,)*
                #(#state_type_vars: ::bon::private::MemberState + ::core::fmt::Debug,)*
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #(::bon::private::assert_debug::<#builder_component_types>();)*

                    let mut output = f.debug_struct(#builder_ident_str);

                    #format_receiver
                    #(#format_members)*

                    output.finish()
                }
            }
        }
    }
}
