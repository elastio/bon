use super::builder_params::{BuilderDerives, ItemParams, OnParams};
use super::member::Member;
use crate::util::prelude::*;

pub(super) trait FinishFnBody {
    /// Generate the `finish` function body from the ready-made variables.
    /// The generated function body may assume that there are variables
    /// named the same as the members in scope.
    fn generate(&self, members: &[Member]) -> TokenStream2;
}

pub(super) struct AssocMethodReceiverCtx {
    pub(super) with_self_keyword: syn::Receiver,
    pub(super) without_self_keyword: Box<syn::Type>,
}

pub(super) struct AssocMethodCtx {
    /// The `Self` type of the impl block. It doesn't contain any nested
    /// `Self` keywords in it. This is prohibited by Rust's syntax itself.
    pub(super) self_ty: Box<syn::Type>,

    /// Present only if the method has a receiver, i.e. `self` or `&self` or
    /// `&mut self` or `self: ExplicitType`.
    pub(super) receiver: Option<AssocMethodReceiverCtx>,
}

pub(super) struct FinishFn {
    pub(super) ident: syn::Ident,

    /// Additional attributes to apply to the item
    pub(super) attrs: Vec<syn::Attribute>,

    pub(super) unsafety: Option<syn::Token![unsafe]>,
    pub(super) asyncness: Option<syn::Token![async]>,
    /// <https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute>
    pub(super) must_use: Option<syn::Attribute>,
    pub(super) body: Box<dyn FinishFnBody>,
    pub(super) output: syn::ReturnType,
}

pub(super) struct StartFn {
    pub(super) ident: syn::Ident,

    /// Additional attributes to apply to the item
    pub(super) attrs: Vec<syn::Attribute>,

    /// Overrides the common generics
    pub(super) generics: Option<Generics>,

    /// If present overrides the automatic visibility
    pub(super) vis: Option<syn::Visibility>,
}

pub(super) struct BuilderType {
    pub(super) ident: syn::Ident,
    pub(super) derives: BuilderDerives,
    pub(super) docs: Vec<syn::Attribute>,
}

pub(super) struct BuilderMod {
    pub(super) ident: syn::Ident,
    pub(super) docs: Vec<syn::Attribute>,

    /// Visibility equivalent to the [`BuilderGenCtx::vis`], but for items
    /// generated inside the builder child module.
    pub(super) vis_child: syn::Visibility,

    /// Visibility equivalent to the [`Self::vis_child`], but for items
    /// generated inside one more level of nesting in the builder child module.
    pub(super) vis_child_child: syn::Visibility,
}

pub(super) struct Generics {
    pub(super) where_clause: Option<syn::WhereClause>,

    /// Original generics that may contain default values in them. This is only
    /// suitable for use in places where default values for generic parameters
    /// are allowed.
    pub(super) decl_with_defaults: Vec<syn::GenericParam>,

    /// Generic parameters without default values in them. This is suitable for
    /// use as generics in function signatures or impl blocks.
    pub(super) decl_without_defaults: Vec<syn::GenericParam>,

    /// Mirrors the `decl` representing how generic params should be represented
    /// when these parameters are passed through as arguments in a turbofish.
    pub(super) args: Vec<syn::GenericArgument>,
}

pub(crate) struct BuilderGenCtx {
    pub(super) members: Vec<Member>,

    /// Lint suppressions from the original item that will be inherited by all items
    /// generated by the macro. If the original syntax used `#[expect(...)]`,
    /// then it must be represented as `#[allow(...)]` here.
    pub(super) allow_attrs: Vec<syn::Attribute>,
    pub(super) on_params: Vec<OnParams>,

    pub(super) generics: Generics,

    /// Visibility of the generated items
    pub(super) vis: syn::Visibility,

    pub(super) assoc_method_ctx: Option<AssocMethodCtx>,

    pub(super) builder_type: BuilderType,
    pub(super) builder_mod: BuilderMod,
    pub(super) start_fn: StartFn,
    pub(super) finish_fn: FinishFn,
}

pub(super) struct BuilderGenCtxParams {
    pub(super) members: Vec<Member>,

    pub(super) allow_attrs: Vec<syn::Attribute>,
    pub(super) on_params: Vec<OnParams>,

    pub(super) generics: Generics,
    pub(super) vis: syn::Visibility,
    pub(super) assoc_method_ctx: Option<AssocMethodCtx>,

    pub(super) builder_type: BuilderTypeParams,
    pub(super) builder_mod: ItemParams,
    pub(super) start_fn: StartFn,
    pub(super) finish_fn: FinishFn,
}

pub(super) struct BuilderTypeParams {
    pub(super) ident: syn::Ident,
    pub(super) derives: BuilderDerives,
    pub(super) docs: Option<Vec<syn::Attribute>>,
}

impl BuilderGenCtx {
    pub(super) fn new(params: BuilderGenCtxParams) -> Result<Self> {
        let BuilderGenCtxParams {
            members,
            allow_attrs,
            on_params,
            generics,
            vis,
            assoc_method_ctx,
            builder_type,
            builder_mod,
            start_fn,
            finish_fn,
        } = params;

        let vis_child = vis.clone().into_equivalent_in_child_module()?;
        let vis_child_child = vis_child.clone().into_equivalent_in_child_module()?;

        let builder_mod = BuilderMod {
            vis_child,
            vis_child_child,

            ident: builder_mod
                .name
                .unwrap_or_else(|| builder_type.ident.pascal_to_snake_case()),

            docs: builder_mod.docs.unwrap_or_else(|| {
                let docs = format!(
                    "Contains the traits and type aliases for manipulating \
                    the type state of the {}",
                    builder_type.ident
                );

                vec![syn::parse_quote!(#[doc = #docs])]
            }),
        };

        let builder_type = BuilderType {
            docs: builder_type.docs.unwrap_or_else(|| {
                let doc = format!(
                    "Use builder syntax to set the required parameters and finish \
                    by calling the method [`Self::{}()`].",
                    finish_fn.ident
                );

                vec![syn::parse_quote! {
                    #[doc = #doc]
                }]
            }),
            derives: builder_type.derives,
            ident: builder_type.ident,
        };

        Ok(Self {
            members,
            allow_attrs,
            on_params,
            generics,
            vis,
            assoc_method_ctx,
            builder_type,
            builder_mod,
            start_fn,
            finish_fn,
        })
    }
}

impl Generics {
    pub(super) fn new(
        decl_with_defaults: Vec<syn::GenericParam>,
        where_clause: Option<syn::WhereClause>,
    ) -> Self {
        let decl_without_defaults = decl_with_defaults
            .iter()
            .cloned()
            .map(|mut param| {
                match &mut param {
                    syn::GenericParam::Type(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Const(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Lifetime(_) => {}
                }
                param
            })
            .collect();

        let args = decl_with_defaults
            .iter()
            .map(syn::GenericParam::to_generic_argument)
            .collect();

        Self {
            where_clause,
            decl_with_defaults,
            decl_without_defaults,
            args,
        }
    }

    pub(super) fn where_clause_predicates(&self) -> impl Iterator<Item = &syn::WherePredicate> {
        self.where_clause
            .as_ref()
            .into_iter()
            .flat_map(|clause| &clause.predicates)
    }
}
