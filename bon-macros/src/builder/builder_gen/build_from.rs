use crate::builder::builder_gen::{BuilderGenCtx, member::Member};
use crate::util::prelude::*;
use quote::quote;
use syn::Type;

pub(super) fn emit(ctx: &BuilderGenCtx, target_ty: &Type) -> TokenStream {
    let mut tokens = TokenStream::new();

    let field_vars: Vec<_> = ctx
        .members
        .iter()
        .map(|member| {
            let ident = member.orig_ident();
            let ty = member.norm_ty();
            let default_expr = quote! { ::core::default::Default::default() };

            match member {
                Member::Field(_) | Member::StartFn(_) => quote! {
                    let #ident: #ty = self.#ident;
                },
                Member::Named(member) => {
                    let index = &member.index;
                    quote! {
                        let #ident: #ty = match self.__unsafe_private_named.#index {
                            Some(value) => value,
                            None => from.#ident.clone(),
                        };
                    }
                }
                Member::FinishFn(_) => quote! {
                    let #ident: #ty = from.#ident.clone();
                },
                Member::Skip(_) => quote! {
                    let #ident: #ty = #default_expr;
                },
            }
        })
        .collect();

    let ctor_args: Vec<_> = ctx
        .members
        .iter()
        .map(|m| {
            let ident = m.orig_ident();
            quote! { #ident }
        })
        .collect();

    if ctx.build_from {
        tokens.extend(emit_build_from_method(
            false,
            target_ty,
            &field_vars,
            &ctor_args,
        ));
    }

    if ctx.build_from_clone {
        tokens.extend(emit_build_from_method(
            true,
            target_ty,
            &field_vars,
            &ctor_args,
        ));
    }

    tokens
}

fn emit_build_from_method(
    clone: bool,
    target_ty: &Type,
    field_vars: &[TokenStream],
    ctor_args: &[TokenStream],
) -> TokenStream {
    let doc = if clone {
        "Fills unset builder fields from an owned value of the target type and builds it."
    } else {
        "Fills unset builder fields from a reference to the target type and builds it."
    };

    let method_name = if clone {
        quote!(build_from_clone)
    } else {
        quote!(build_from)
    };

    let arg_type = if clone {
        quote!(#target_ty)
    } else {
        quote!(&#target_ty)
    };

    let arg_pat = if clone {
        quote!(mut from)
    } else {
        quote!(from)
    };

    // Convert `target_ty` to a path segment with no generics
    let ctor_path = if let Type::Path(type_path) = target_ty {
        let ident = &type_path.path.segments.last().unwrap().ident;
        quote!(#ident)
    } else {
        quote!(#target_ty)
    };

    quote! {
        #[inline(always)]
        #[doc = #doc]
        pub fn #method_name(self, #arg_pat: #arg_type) -> #target_ty {
            #( #field_vars )*
            #ctor_path {
                #( #ctor_args, )*
            }
        }
    }
}
