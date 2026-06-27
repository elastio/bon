use crate::builder::builder_gen::{BuilderGenCtx, member::Member};
use crate::util::prelude::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Type, ext::IdentExt, spanned::Spanned};

pub(super) fn emit(ctx: &BuilderGenCtx, target_ty: &Type) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    let ctor_args: Vec<_> = ctx
        .members
        .iter()
        .map(|m| {
            let ident = m.orig_ident();
            quote! { #ident }
        })
        .collect();
    let base_name = ctx.finish_fn.ident.clone();
    if ctx.build_from.is_some() {
        tokens.extend(emit_build_from_method(
            false,
            &base_name,
            target_ty,
            &ctx.members,
            &ctor_args,
            ctx.build_from.as_ref(),
        )?);
    }
    if ctx.build_from_clone.is_some() {
        tokens.extend(emit_build_from_method(
            true,
            &base_name,
            target_ty,
            &ctx.members,
            &ctor_args,
            ctx.build_from_clone.as_ref(),
        )?);
    }
    Ok(tokens)
}

fn emit_build_from_method(
    clone: bool,
    base_name: &Ident,
    target_ty: &Type,
    members: &[Member],
    ctor_args: &[TokenStream],
    config: Option<&crate::parsing::ItemSigConfig>,
) -> Result<TokenStream> {
    let doc = if clone {
        "Fills unset builder fields from a reference to the target type and builds it."
    } else {
        "Fills unset builder fields from an owned value of the target type and builds it."
    };
    let method_name: Ident = config
        .and_then(|cfg| cfg.name.as_ref().map(|spanned_key| spanned_key.unraw()))
        .unwrap_or_else(|| {
            if clone {
                format_ident!("{}_from_clone", base_name)
            } else {
                format_ident!("{}_from", base_name)
            }
        });
    let arg_type = if clone {
        quote!(&#target_ty)
    } else {
        quote!(#target_ty)
    };
    let arg_pat = if clone {
        quote!(mut from)
    } else {
        quote!(from)
    };
    let ctor_path = extract_ctor_ident_path(target_ty, target_ty.span())?;
    let field_vars = field_vars_from_members(members, clone);
    Ok(quote! {
        #[inline(always)]
        #[doc = #doc]
        pub fn #method_name(self, #arg_pat: #arg_type) -> #target_ty {
            #( #field_vars )*
            #ctor_path {
                #( #ctor_args, )*
            }
        }
    })
}

fn field_vars_from_members(members: &[Member], clone: bool) -> Vec<TokenStream> {
    members
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
                    if clone {
                        quote! {
                            let #ident: #ty = match self.__unsafe_private_named.#index {
                                Some(value) => value,
                                None => from.#ident.clone(),
                            };
                        }
                    } else {
                        quote! {
                            let #ident: #ty = match self.__unsafe_private_named.#index {
                                Some(value) => value,
                                None => from.#ident,
                            };
                        }
                    }
                }
                Member::FinishFn(_) => {
                    if clone {
                        quote! {
                            let #ident: #ty = from.#ident.clone();
                        }
                    } else {
                        quote! {
                            let #ident: #ty = from.#ident;
                        }
                    }
                }
                Member::Skip(_) => quote! {
                    let #ident: #ty = #default_expr;
                },
            }
        })
        .collect()
}

pub(crate) fn extract_ctor_ident_path(ty: &Type, span: Span) -> Result<TokenStream> {
    let path = ty.as_path_no_qself().ok_or_else(|| {
        err!(
            &span,
            "expected a concrete type path (like `MyStruct`) for constructor"
        )
    })?;
    let mut clean_path = path.clone();
    if let Some(last_segment) = clean_path.segments.last_mut() {
        last_segment.arguments = syn::PathArguments::None;
        last_segment.ident.set_span(span);
    }
    Ok(quote! { #clean_path })
}
