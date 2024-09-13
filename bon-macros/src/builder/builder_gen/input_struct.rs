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

impl StructInputParams {
    fn parse(item_struct: &syn::ItemStruct) -> Result<Self> {
        let meta = item_struct
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("builder"))
            .map(|attr| {
                let meta = match &attr.meta {
                    syn::Meta::List(meta) => meta,
                    _ => bail!(attr, "expected `#[builder(...)]` syntax"),
                };

                if !matches!(meta.delimiter, syn::MacroDelimiter::Paren(_)) {
                    bail!(
                        &meta,
                        "wrong delimiter {:?}, expected `#[builder(...)]` syntax",
                        meta.delimiter
                    );
                }

                let meta = darling::ast::NestedMeta::parse_meta_list(meta.tokens.clone())?;

                Ok(meta)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .concat();

        Self::from_list(&meta)
    }
}

pub(crate) struct StructInputCtx {
    orig_struct: syn::ItemStruct,
    norm_struct: syn::ItemStruct,
    params: StructInputParams,
    struct_ty: syn::Type,
}

impl StructInputCtx {
    pub(crate) fn new(orig_struct: syn::ItemStruct) -> Result<Self> {
        let params = StructInputParams::parse(&orig_struct)?;

        let generic_args = orig_struct
            .generics
            .params
            .iter()
            .map(super::generic_param_to_arg);
        let struct_ident = &orig_struct.ident;
        let struct_ty = syn::parse_quote!(#struct_ident<#(#generic_args),*>);

        let mut norm_struct = orig_struct.clone();

        // Structs are free to use `Self` inside of their trait bounds and any
        // internal type contexts. However, when copying these bounds to the
        // builder struct and its impl blocks we need to get rid of `Self`
        // references and replace them with the actual struct type.
        crate::normalization::NormalizeSelfTy {
            self_ty: &struct_ty,
        }
        .visit_item_struct_mut(&mut norm_struct);

        Ok(Self {
            orig_struct,
            norm_struct,
            params,
            struct_ty,
        })
    }

    fn builder_ident(&self) -> syn::Ident {
        if let Some(builder_type) = &self.params.base.builder_type {
            return builder_type.clone();
        }

        quote::format_ident!("{}Builder", self.norm_struct.ident.raw_name())
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

        let generics = Generics::new(
            self.norm_struct.generics.params.iter().cloned().collect(),
            self.norm_struct.generics.where_clause.clone(),
        );

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

        let allow_attrs = self
            .norm_struct
            .attrs
            .iter()
            .filter_map(syn::Attribute::to_allow)
            .collect();

        let ctx = BuilderGenCtx {
            members,

            allow_attrs,

            conditional_params: self.params.base.on,
            builder_derives: self.params.base.derive,

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
        let member_vars = member_exprs.iter().map(Member::orig_ident);

        quote! {
            #struct_ident {
                #(#member_vars,)*
            }
        }
    }
}
