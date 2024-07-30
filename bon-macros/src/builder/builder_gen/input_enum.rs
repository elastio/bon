use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::{Ident,Span};
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned, Fields, FieldsNamed, ItemStruct, PathSegment};
use convert_case::{Case, Casing};
use crate::{builder::{builder_gen::{input_struct::StructInputParams, FinishFuncBody, MemberExpr}, item_struct, params::ItemParams, shared}, util::prelude::*};

use super::input_struct::StructLiteralBody; 

pub(crate) struct NamedVariantInputCtx<'a> {
    generated_struct : ItemStruct,
    orig_enum : &'a syn::ItemEnum,
    variant : &'a syn::Variant
}

struct NamedVariantFinishFuncBody { 
    orig_enum_ident : Ident,
    variant_ident : Ident,

    // Prevents breaking if a `rename` method is added to the struct builder in the future.
    named_fields : FieldsNamed,

    literal: StructLiteralBody
}

impl<'a> NamedVariantInputCtx<'a> {
    pub(crate) fn new(orig_enum : &'a syn::ItemEnum,variant : &'a syn::Variant,fields : FieldsNamed) -> Self {
        debug_assert!(matches!(variant.fields,Fields::Named(_)));

        let orig_ident = &orig_enum.ident;
        let variant_ident = &variant.ident;
        let generated_struct_ident = format_ident!("{orig_ident}{variant_ident}");

        let orig_vis = &orig_enum.vis;
        let generated_struct: ItemStruct = syn::parse_quote! {
            // TODO: Generate variant generics as well
            #orig_vis struct #generated_struct_ident #fields
        };

        Self {
            generated_struct : generated_struct,
            orig_enum : orig_enum,
            variant : variant
        }
    }

    pub(crate) fn generate(self) -> Result<TokenStream2> {
        let mut token_stream = self.generated_struct.to_token_stream();

        let params = &NestedMeta::parse_meta_list(self.variant.to_token_stream())?;
        let input_struct: StructInputParams = FromMeta::from_list(params)?;

        // Generate builder and alter the finsih_fn to return enum 
        token_stream.extend(self.generate_builder(input_struct.clone())?);

        // Generate the start_fn for the original enum to access the builder method
        token_stream.extend(self.variant_start_fn(input_struct));

        Ok(token_stream)
    }

    fn generate_builder(&self,input_struct : StructInputParams) -> Result<TokenStream2> {
        // Generate Builder for the VariantStruct
        // TODO: try to limit the visibility of the VariantStruct#start_fn pub(super) somehow 
        shared::generate(
            input_struct,
            self.generated_struct.clone(),
            |ctx| {
                let segment = PathSegment {
                    ident : self.orig_enum.ident.clone(),
                    arguments : syn::PathArguments::None
                };

                let mut segments = Punctuated::new();
                segments.push(segment);

                let path = syn::Path {
                    segments : segments,
                    leading_colon : None
                };

                let type_path = syn::TypePath {
                    path : path,
                    qself : None
                };

                let Fields::Named(ref named_fields) = self.variant.fields else {
                    // Due to asseration in new method
                    unreachable!()
                };

                let finish_fn_body = NamedVariantFinishFuncBody {
                    orig_enum_ident : self.orig_enum.ident.clone(),
                    variant_ident : self.variant.ident.clone(),
                    named_fields : named_fields.clone(),
                    literal : StructLiteralBody::new(ctx.norm_struct.ident.clone())
                };
                
                ctx.into_builder_gen_ctx(
                    Some((
                        &syn::Type::Path(type_path),
                        finish_fn_body
                    ))
                )
            }
        )
    }

    fn variant_start_fn(&self,input_struct : StructInputParams) -> TokenStream2 {
        let orig_ident = &self.orig_enum.ident;
        let ItemParams { name , vis } = input_struct.start_fn.unwrap_or_default();
        let vis = vis.unwrap_or_else(|| self.orig_enum.vis.clone());
        let ident = name.clone().unwrap_or_else(|| {
            let ident = &self.variant.ident;
            let variant_span = ident.span();
            let variant_ident = ident.raw_name().to_case(Case::Snake);

            eprintln!("variant_ident={variant_ident}");
            
            Ident::new_maybe_raw(&variant_ident,variant_span.span())
        });

        let return_type = &self.generated_struct.ident;
        let inner_start_fn = name.unwrap_or_else(|| Ident::new("builder",Span::call_site()));

        quote! {
            // TODO: Use enum generics here but only add variant bounds
            impl #orig_ident {
                #vis fn #ident () -> #return_type {
                    #return_type :: #inner_start_fn ()
                }
            }
        }
    }
}

impl FinishFuncBody for NamedVariantFinishFuncBody { 
    fn gen(&self, member_exprs: &[MemberExpr<'_>]) -> TokenStream2 {
        let enum_ident = &self.orig_enum_ident;
        let variant_ident = &self.variant_ident;

        let struct_value = self.literal.r#gen(member_exprs);
        let struct_value_ident = format_ident!("value");

        let transfer_values = self.named_fields.named
            .iter()
            .zip(member_exprs.iter())
            .map(|(field,MemberExpr { member, .. })|{
                let value_ident = field.ident.as_ref();
                let struct_value_ident = &member.ident;
                quote! {
                    #value_ident : #struct_value_ident . #struct_value_ident
                }
            });

        quote! {
            let #struct_value_ident = #struct_value;
            
            #enum_ident :: #variant_ident { 
                #(#transfer_values,)*
            }
        }
    }
}
