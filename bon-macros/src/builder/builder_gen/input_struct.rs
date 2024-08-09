use super::{
    AssocMethodCtx, BuilderGenCtx, FinishFunc, FinishFuncBody, Generics, Member, MemberExpr,
    MemberOrigin, StartFunc,
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
        let builder_private_impl_ident =
            quote::format_ident!("__{}PrivateImpl", builder_ident.raw_name());

        let builder_state_trait_ident = quote::format_ident!("__{}State", builder_ident.raw_name());

        let fields = match self.norm_struct.fields {
            syn::Fields::Named(fields) => fields,
            _ => {
                bail!(
                    &self.norm_struct,
                    "Only structs with named fields are supported"
                )
            }
        };

        let members: Vec<_> = fields
            .named
            .iter()
            .map(Member::from_syn_field)
            .try_collect()?;

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
            body: Box::new(finish_func_body),
            output: syn::parse_quote!(-> #struct_ty),
            docs: "Finishes building an returns the requested object.".to_owned(),
        };

        let start_func_docs = format!(
            "Use builder syntax to create an instance of [`{}`]",
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
            builder_ident,
            builder_private_impl_ident,
            builder_state_trait_ident,

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
    fn gen(&self, member_exprs: &[MemberExpr<'_>]) -> TokenStream2 {
        let Self { struct_ident } = self;

        let member_exprs = member_exprs.iter().map(|MemberExpr { member, expr }| {
            let ident = &member.ident().unwrap_or_else(|| {
                panic!("All struct members must be named, but got: {member:#?}:\n{expr}")
            });
            quote! {
                #ident: #expr
            }
        });

        quote! {
            #struct_ident {
                #(#member_exprs,)*
            }
        }
    }
}

impl Member {
    pub(crate) fn from_syn_field(field: &syn::Field) -> Result<Self> {
        Member::new(
            MemberOrigin::StructField,
            &field.attrs,
            field.ident.clone(),
            Box::new(field.ty.clone()),
        )
    }
}
