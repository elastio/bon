use super::{Field, MacroCtx};
use itertools::Itertools;
use prox::prelude::*;
use quote::{quote, ToTokens};

impl MacroCtx<'_> {
    pub(crate) fn setter_methods_impls_for_field(&self, field: &Field) -> TokenStream2 {
        let output_fields_states = self.fields.iter().map(|other_field| {
            if other_field.fn_arg_ident == field.fn_arg_ident {
                return field.set_state_type().to_token_stream();
            }

            let state_assoc_type_ident = &other_field.state_assoc_type_ident;
            quote!(__State::#state_assoc_type_ident)
        });

        let generic_args = self.impl_and_norm_func_generic_args().collect_vec();
        let state_assoc_type_ident = &field.state_assoc_type_ident;
        let builder_ident = &self.builder_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let generics_decl = self.impl_and_norm_func_generics_decl();
        let where_clause = self.impl_and_norm_func_where_clause();
        let unset_state_type = field.unset_state_type();
        let output_builder_alias_ident =
            quote::format_ident!("__{builder_ident}Set{state_assoc_type_ident}");

        // A case where there is just one field is special, because the type alias would
        // receive a generic `__State` parameter that it wouldn't use, so we create it
        // only if there are 2 or more fields.
        let (output_builder_alias_state_var_decl, output_builder_alias_state_arg) =
            (self.fields.len() > 1)
                .then(|| (quote!(__State: #builder_state_trait_ident), quote!(__State)))
                .unzip();

        let setter_methods = FieldSettersCtx {
            macro_ctx: self,
            return_type: quote! {
                #output_builder_alias_ident<
                    #(#generic_args,)*
                    #output_builder_alias_state_arg
                >
            },
        }
        .setter_methods(field);

        quote! {
            // This lint is ignored, because bounds in type aliases are still useful
            // to make the following example usage compile:
            // ```
            // type Bar<T: IntoIterator> = T::Item;
            // ```
            // In this case the bound is necessary for `T::Item` access to be valid.
            // The compiler proposes this:
            //
            // > use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer
            // > to associated types in type aliases
            //
            // But, come on... Why would you want to write `T::Item` as `<T as IntoIterator>::Item`
            // especially if that `T::Item` access is used in multiple places? It's a waste of time
            // to implement logic that rewrites the user's type expressions to that syntax when just
            // having bounds on the type alias is enough already.
            #[allow(type_alias_bounds)]
            type #output_builder_alias_ident<
                #(#generics_decl,)*
                #output_builder_alias_state_var_decl
            >
            // The where clause in this position will be deprecated. The preferred
            // position will be at the end of the entire type alias syntax construct.
            // See details at https://github.com/rust-lang/rust/issues/112792.
            //
            // However, at the time of this writing the only way to put the where
            // clause on a type alias is here.
            #where_clause
            = #builder_ident<
                #(#generic_args,)*
                ( #(#output_fields_states,)* )
            >;

            impl<
                #(#generics_decl,)*
                __State: #builder_state_trait_ident<
                    #state_assoc_type_ident = #unset_state_type
                >
            >
            #builder_ident<
                #(#generic_args,)*
                __State
            >
            #where_clause
            {
                #setter_methods
            }
        }
    }
}

struct FieldSettersCtx<'a> {
    macro_ctx: &'a MacroCtx<'a>,
    return_type: TokenStream2,
}

impl FieldSettersCtx<'_> {
    fn setter_methods(&self, field: &Field) -> TokenStream2 {
        let field_type = field.fn_arg_type.as_ref();
        let field_ident = &field.fn_arg_ident;

        if let Some(inner_type) = field_type.option_type_param() {
            let syn::Type::Path(mut option_path) = field_type.clone() else {
                unreachable!(
                    "BUG: option_type_param returned Some for a non-Path type: \
                    {field_type:#?}",
                )
            };
            if let Some(segment) = option_path.path.segments.last_mut() {
                segment.arguments = syn::PathArguments::None;
            }

            let optionless_method_ident = field_ident.clone();

            let methods = [
                FieldSetterMethod {
                    method_name: quote::format_ident!("maybe_{field_ident}"),
                    field,
                    fn_params: quote!(value: #option_path<#inner_type>),
                    field_init: quote!(bon::Set::new(value)),
                    overwrite_docs: Some(format!(
                        "Same as [`Self::{optionless_method_ident}`], but accepts \
                        an `Option` as input. See that method's documentation for \
                        more details.",
                    )),
                },
                // We intentionally keep the name and signature of the setter method
                // for an optional field that accepts the value under the option the
                // same as the setter method for the required field to keep the API
                // of the builder compatible when a required input field becomes
                // optional. To be able to explicitly pass an `Option` value to the
                // setter method users need to use the `maybe_{field_ident}` method.
                FieldSetterMethod {
                    method_name: optionless_method_ident,
                    field,
                    fn_params: quote!(value: #inner_type),
                    field_init: quote!(bon::Set::new(Some(value))),
                    overwrite_docs: None,
                },
            ];

            return methods
                .into_iter()
                .map(|method| self.setter_method(method))
                .collect();
        }

        self.setter_method(FieldSetterMethod {
            method_name: field_ident.clone(),
            field,
            fn_params: quote!(value: #field_type),
            field_init: quote!(bon::Set::new(value)),
            overwrite_docs: None,
        })
    }

    fn setter_method(&self, method: FieldSetterMethod<'_>) -> TokenStream2 {
        let return_type = &self.return_type;
        let FieldSetterMethod {
            method_name,
            field,
            fn_params,
            field_init,
            overwrite_docs,
        } = method;

        let docs = match overwrite_docs {
            Some(docs) => &[syn::parse_quote!(#[doc = #docs])],
            None => field.docs.as_slice(),
        };

        let vis = &self.macro_ctx.norm_func.vis;

        let builder_ident = &self.macro_ctx.builder_ident;
        let builder_private_impl_ident = &self.macro_ctx.builder_private_impl_ident;
        let maybe_phantom_field = self.macro_ctx.phantom_field_init();
        let field_idents = self.macro_ctx.field_idents();
        let maybe_receiver_field = self
            .macro_ctx
            .norm_func
            .sig
            .receiver()
            .map(|_| quote!(receiver: self.__private_impl.receiver,));

        let field_exprs = self.macro_ctx.fields.iter().map(|other_field| {
            if other_field.fn_arg_ident == field.fn_arg_ident {
                return field_init.clone();
            }

            let ident = &other_field.fn_arg_ident;
            quote!(self.__private_impl.#ident)
        });

        quote! {
            #( #docs )*
            #vis fn #method_name(self, #fn_params) -> #return_type {
                #builder_ident {
                    __private_impl: #builder_private_impl_ident {
                        #maybe_phantom_field
                        #maybe_receiver_field
                        #( #field_idents: #field_exprs, )*
                    }
                }
            }
        }
    }
}

struct FieldSetterMethod<'a> {
    method_name: syn::Ident,
    field: &'a Field,
    fn_params: TokenStream2,
    field_init: TokenStream2,
    overwrite_docs: Option<String>,
}
