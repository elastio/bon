use super::BuilderGenCtx;
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

    /// These bounds are required to ensure that all members of the,
    /// builder (including the receiver) implement the target trait,
    /// so that there is no possible state of the builder that cannot
    /// implement the target trait.
    fn builder_components_trait_bounds<'a>(
        &'a self,
        trait_path: &'a TokenStream2,
    ) -> impl Iterator<Item = TokenStream2> + 'a {
        let receiver_ty = self
            .receiver()
            .map(|receiver| &receiver.without_self_keyword);

        let member_types = self.named_members().map(|member| &member.norm_ty);

        std::iter::empty()
            .chain(receiver_ty)
            .chain(member_types)
            .map(move |ty| {
                quote! {
                    #ty: #trait_path
                }
            })
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

        let builder_where_clause_predicates = self.generics.where_clause_predicates();
        let components_where_clause_predicates = self.builder_components_trait_bounds(&clone);

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
                #(#components_where_clause_predicates,)*
            {
                fn clone(&self) -> Self {
                    Self {
                        __private_phantom: ::core::marker::PhantomData,
                        #clone_receiver
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
        let components_where_clause_predicates = self.builder_components_trait_bounds(&debug);

        let builder_ident_str = builder_ident.to_string();

        let state_type_vars = self
            .named_members()
            .map(|member| &member.generic_var_ident)
            .collect::<Vec<_>>();

        let format_members = self.named_members().map(|member| {
            let member_index = &member.index;
            let member_ident_str = member.orig_ident.to_string();

            quote! {
                // Skip members that are not set to reduce noise
                if self.__private_named_members.#member_index.is_set() {
                    output.field(#member_ident_str, &self.__private_named_members.#member_index);
                }
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
                #(#components_where_clause_predicates,)*
                #(#state_type_vars: ::bon::private::MemberState + ::core::fmt::Debug,)*
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
