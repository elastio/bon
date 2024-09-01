use super::{
    AssocMethodCtx, BuilderGenCtx, FinishFunc, FinishFuncBody, Generics, Member, MemberOrigin,
    RawMember, StartFunc,
};
use crate::builder::params::{BuilderParams, ItemParams};
use crate::util::prelude::*;
use darling::FromMeta;
use quote::quote;
use syn::visit_mut::VisitMut;

#[derive(Debug, FromMeta)]
pub(crate) struct StructInputParams {
    #[darling(flatten)]
    base: BuilderParams,
    start_fn: Option<ItemParams>,
}

pub(crate) struct StructInputCtx {
    orig_struct: syn::ItemStruct,
    norm_struct: syn::ItemStruct,
    params: StructInputParams,
    struct_ty: syn::Type,
}

impl StructInputCtx {
    pub(crate) fn new(params: StructInputParams, orig_struct: syn::ItemStruct) -> Self {
        let generic_args = orig_struct
            .generics
            .params
            .iter()
            .map(super::generic_param_to_arg);
        let struct_ident = &orig_struct.ident;
        let struct_ty = syn::parse_quote!(#struct_ident<#(#generic_args),*>);

        let mut norm_struct = orig_struct.clone();

        // Structs are free to use `Self` inside of their trait bounds and any
        // internal type contexts.
        crate::normalization::NormalizeSelfTy {
            self_ty: &struct_ty,
        }
        .visit_item_struct_mut(&mut norm_struct);

        Self {
            orig_struct,
            norm_struct,
            params,
            struct_ty,
        }
    }

    fn builder_ident(&self) -> syn::Ident {
        if let Some(builder_type) = &self.params.base.builder_type {
            return builder_type.clone();
        }

        quote::format_ident!("{}Builder", self.norm_struct.ident.raw_name())
    }

    pub(crate) fn adapted_struct(&self) -> syn::ItemStruct {
        let mut orig = self.orig_struct.clone();

        // Remove all `#[builder]` attributes from the struct since
        // we used them just to configure this macro, and are they
        // no longer needed in the output code
        orig.attrs.retain(|attr| !attr.path().is_ident("builder"));

        for field in &mut orig.fields {
            field.attrs.retain(|attr| !attr.path().is_ident("builder"));
        }

        orig
    }

    pub(crate) fn into_builder_gen_ctx(self) -> Result<BuilderGenCtx> {
        let builder_ident = self.builder_ident();

        fn fields(struct_item: &syn::ItemStruct) -> Result<&syn::FieldsNamed> {
            match &struct_item.fields {
                syn::Fields::Named(fields) => Ok(fields),
                _ => {
                    bail!(&struct_item, "Only structs with named fields are supported")
                }
            }
        }

        let norm_fields = fields(&self.norm_struct)?;
        let orig_fields = fields(&self.orig_struct)?;

        let members = norm_fields
            .named
            .iter()
            .zip(&orig_fields.named)
            .map(|(norm_field, orig_field)| {
                let ident = norm_field.ident.clone().ok_or_else(|| {
                    err!(norm_field, "only structs with named fields are supported")
                })?;

                Ok(RawMember {
                    attrs: &norm_field.attrs,
                    ident,
                    norm_ty: Box::new(norm_field.ty.clone()),
                    orig_ty: Box::new(orig_field.ty.clone()),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let members = Member::from_raw(MemberOrigin::StructField, members)?;

        let generics = Generics {
            params: Vec::from_iter(self.norm_struct.generics.params.iter().cloned()),
            where_clause: self.norm_struct.generics.where_clause.clone(),
        };

        let finish_func_body = StructLiteralBody {
            struct_ident: self.norm_struct.ident.clone(),
        };

        let ItemParams {
            name: start_func_ident,
            vis: start_func_vis,
        } = self.params.start_fn.unwrap_or_default();

        let start_func_ident = start_func_ident
            .unwrap_or_else(|| syn::Ident::new("builder", self.norm_struct.ident.span()));

        let finish_func_ident = self
            .params
            .base
            .finish_fn
            .unwrap_or_else(|| syn::Ident::new("build", start_func_ident.span()));

        let struct_ty = &self.struct_ty;
        let finish_func = FinishFunc {
            ident: finish_func_ident,
            unsafety: None,
            asyncness: None,
            must_use: Some(syn::parse_quote! {
                #[must_use = "building a struct without using it is likely a bug"]
            }),
            body: Box::new(finish_func_body),
            output: syn::parse_quote!(-> #struct_ty),
            docs: "Finishes building and returns the requested object.".to_owned(),
        };

        let start_func_docs = format!(
            "Create an instance of [`{}`] using the builder syntax",
            self.norm_struct.ident
        );

        let start_func = StartFunc {
            ident: start_func_ident,
            vis: start_func_vis,
            attrs: vec![syn::parse_quote!(#[doc = #start_func_docs])],
            generics: None,
        };

        let assoc_method_ctx = Some(AssocMethodCtx {
            self_ty: self.struct_ty.into(),
            receiver: None,
        });

        let ctx = BuilderGenCtx {
            members,

            conditional_params: self.params.base.on,

            builder_ident,

            assoc_method_ctx,
            generics,
            vis: self.norm_struct.vis,

            start_func,
            finish_func,
        };

        Ok(ctx)
    }
}

struct StructLiteralBody {
    struct_ident: syn::Ident,
}

impl FinishFuncBody for StructLiteralBody {
    fn generate(&self, member_exprs: &[Member]) -> TokenStream2 {
        let Self { struct_ident } = self;

        // The variables with values of members are in scope for this expression.
        let member_vars = member_exprs.iter().map(Member::ident);

        quote! {
            #struct_ident {
                #(#member_vars,)*
            }
        }
    }
}
