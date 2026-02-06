use super::models::BuilderGenCtx;
use crate::parsing::ItemSigConfig;
use crate::util::prelude::*;

pub(super) struct GenericSettersCtx<'a> {
    base: &'a BuilderGenCtx,
    config: &'a ItemSigConfig<String>,
}

impl<'a> GenericSettersCtx<'a> {
    pub(super) fn new(base: &'a BuilderGenCtx, config: &'a ItemSigConfig<String>) -> Self {
        Self { base, config }
    }

    pub(super) fn generic_setter_methods(&self) -> Result<TokenStream> {
        let generics = &self.base.generics.decl_without_defaults;

        let mut methods = Vec::with_capacity(generics.len());

        for (index, param) in generics.iter().enumerate() {
            match param {
                syn::GenericParam::Type(type_param) => {
                    methods.push(self.generic_setter_method(index, &type_param.ident));
                }
                syn::GenericParam::Const(const_param) => {
                    bail!(
                        &const_param.ident,
                        "const generic parameters are not supported in `generics(setters(...))`; \
                         only type parameters can be converted"
                    );
                }
                syn::GenericParam::Lifetime(_) => {
                    // Skip lifetimes, they don't get setters
                }
            }
        }

        Ok(quote! {
            #(#methods)*
        })
    }

    fn generic_setter_method(&self, param_index: usize, param_ident: &syn::Ident) -> TokenStream {
        let builder_ident = &self.base.builder_type.ident;
        let state_var = &self.base.state_var;
        let where_clause = &self.base.generics.where_clause;

        let method_name = self.method_name(param_ident);

        let vis = self
            .config
            .vis
            .as_ref()
            .map(|v| &v.value)
            .unwrap_or(&self.base.builder_type.vis);

        let docs = self.method_docs(param_ident);

        // Build the generic arguments for the output type, where the current parameter
        // is replaced with a new type variable
        let new_type_var = self.base.namespace.unique_ident(param_ident.to_string());
        let output_generic_args = self
            .base
            .generics
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                if i == param_index {
                    quote!(#new_type_var)
                } else {
                    quote!(#arg)
                }
            })
            .collect::<Vec<_>>();

        // Check which named members use this generic parameter
        let mut runtime_asserts = Vec::new();
        let named_member_conversions = self
            .base
            .named_members()
            .enumerate()
            .map(|(idx, member)| {
                let uses_param = member_uses_generic_param(member, param_ident);
                let index = syn::Index::from(idx);
                if uses_param {
                    // Add runtime assert that this field is None
                    let field_ident = &member.name.orig;
                    let message = format!("BUG: field `{field_ident}` should be None when converting generic parameter `{param_ident}`");
                    runtime_asserts.push(quote! {
                        ::core::assert!(named.#index.is_none(), #message);
                    });
                    // Field uses the generic parameter, so create a new None
                    quote!(::core::option::Option::None)
                } else {
                    // Field doesn't use the generic parameter, so move it from the tuple
                    quote!(named.#index)
                }
            })
            .collect::<Vec<_>>();

        let receiver_field = self.base.receiver().map(|receiver| {
            let ident = &receiver.field_ident;
            quote!(#ident: self.#ident,)
        });

        let start_fn_fields = self.base.start_fn_args().map(|member| {
            let ident = &member.ident;
            quote!(#ident: self.#ident,)
        });

        let custom_fields = self.base.custom_fields().map(|field| {
            let ident = &field.ident;
            quote!(#ident: self.#ident,)
        });

        quote! {
            #(#docs)*
            #[inline(always)]
            #vis fn #method_name<#new_type_var>(
                self
            ) -> #builder_ident<#(#output_generic_args,)* #state_var>
            #where_clause
            {
                let named = self.__unsafe_private_named;

                // Runtime safety asserts to ensure fields using the converted
                // generic parameter are None
                #(#runtime_asserts)*

                #builder_ident {
                    __unsafe_private_phantom: ::core::marker::PhantomData,
                    #receiver_field
                    #(#start_fn_fields)*
                    #(#custom_fields)*
                    __unsafe_private_named: (
                        #(#named_member_conversions,)*
                    ),
                }
            }
        }
    }

    fn method_name(&self, param_ident: &syn::Ident) -> syn::Ident {
        let param_name_snake = param_ident.pascal_to_snake_case();

        // Name is guaranteed to be present due to validation in parse_setters_config
        let name_pattern = &self
            .config
            .name
            .as_ref()
            .expect("name should be validated")
            .value;

        let method_name = name_pattern.replace("{}", &param_name_snake.to_string());

        syn::Ident::new(&method_name, param_ident.span())
    }

    fn method_docs(&self, param_ident: &syn::Ident) -> Vec<syn::Attribute> {
        // If custom docs are provided, use them
        if let Some(ref docs) = self.config.docs {
            return docs.value.clone();
        }

        // Otherwise, generate default documentation
        let doc = format!(
            "Convert the `{param_ident}` generic parameter to a different type.\n\
            \n\
            This method allows changing the type of the `{param_ident}` parameter on the builder, \
            which is useful when you need to build up values with different types at \
            different stages of construction."
        );

        vec![syn::parse_quote!(#[doc = #doc])]
    }
}

/// Check if a member's type uses a specific generic parameter
fn member_uses_generic_param(member: &super::NamedMember, param_ident: &syn::Ident) -> bool {
    let member_ty = member.underlying_norm_ty();
    type_uses_generic_param(member_ty, param_ident)
}

/// Recursively check if a type uses a specific generic parameter
fn type_uses_generic_param(ty: &syn::Type, param_ident: &syn::Ident) -> bool {
    use syn::visit::Visit;

    struct GenericParamVisitor<'a> {
        param_ident: &'a syn::Ident,
        found: bool,
    }

    impl<'ast> Visit<'ast> for GenericParamVisitor<'_> {
        fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
            // Early return if already found to avoid unnecessary recursion
            if self.found {
                return;
            }

            // Check if the path is the generic parameter we're looking for
            if type_path.path.is_ident(self.param_ident) {
                self.found = true;
                return;
            }

            // Continue visiting the rest of the type path
            syn::visit::visit_type_path(self, type_path);
        }
    }

    let mut visitor = GenericParamVisitor {
        param_ident,
        found: false,
    };
    visitor.visit_type(ty);
    visitor.found
}
