use super::{
    generic_param_to_arg, AssocFreeMethodCtx, AssocMethodCtx, AssocMethodReceiverCtx,
    BuilderGenCtx, FinishFunc, FinishFuncBody, Generics, Member, MemberExpr, MemberOrigin,
    StartFunc,
};
use crate::builder::params::BuilderParams;
use crate::normalization::NormalizeSelfTy;
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromMeta;
use itertools::Itertools;
use proc_macro2::Span;
use quote::quote;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

#[derive(Debug, FromMeta)]
pub(crate) struct FuncInputParams {
    expose_positional_fn: Option<SpannedValue<ExposePositionalFnParams>>,

    #[darling(flatten)]
    base: BuilderParams,
}

#[derive(Debug, Default)]
struct ExposePositionalFnParams {
    name: Option<syn::Ident>,
    vis: Option<syn::Visibility>,
}

impl FromMeta for ExposePositionalFnParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        match meta {
            syn::Meta::Path(_) => {
                return Ok(Self::default());
            }
            syn::Meta::NameValue(meta) => {
                let val = &meta.value;
                let name = syn::parse2(quote!(#val))?;

                return Ok(Self { name, vis: None });
            }
            syn::Meta::List(_) => {}
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
        }

        let full = Full::from_meta(meta)?;

        let me = Self {
            name: full.name,
            vis: full.vis,
        };

        Ok(me)
    }
}

pub(crate) struct FuncInputCtx {
    pub(crate) orig_func: syn::ItemFn,
    pub(crate) norm_func: syn::ItemFn,
    pub(crate) impl_ctx: Option<Rc<ImplCtx>>,
    pub(crate) params: FuncInputParams,
}

pub(crate) struct ImplCtx {
    pub(crate) self_ty: Box<syn::Type>,
    pub(crate) generics: syn::Generics,
}

impl FuncInputCtx {
    fn self_ty_prefix(&self) -> Option<String> {
        let prefix = self
            .impl_ctx
            .as_deref()?
            .self_ty
            .as_path()?
            .path
            .segments
            .last()?
            .ident
            .to_string();

        Some(prefix)
    }

    fn assoc_method_ctx(&self) -> Option<AssocMethodCtx> {
        let self_ty = &self.impl_ctx.as_deref()?.self_ty;

        let Some(receiver) = self.norm_func.sig.receiver() else {
            return Some(AssocMethodCtx::Free(AssocFreeMethodCtx {
                self_ty: self_ty.clone(),
            }));
        };

        let mut without_self_ty = receiver.ty.clone();

        NormalizeSelfTy { self_ty }.visit_type_mut(&mut without_self_ty);

        Some(AssocMethodCtx::Receiver(AssocMethodReceiverCtx {
            with_self_keyword: receiver.clone(),
            without_self_keyword: without_self_ty,
        }))
    }

    fn generics(&self) -> Generics {
        let impl_ctx = self.impl_ctx.as_ref();
        let norm_func_params = &self.norm_func.sig.generics.params;
        let params = impl_ctx
            .map(|impl_ctx| merge_generic_params(&impl_ctx.generics.params, norm_func_params))
            .unwrap_or_else(|| norm_func_params.iter().cloned().collect());

        let where_clauses = [
            self.norm_func.sig.generics.where_clause.clone(),
            impl_ctx.and_then(|impl_ctx| impl_ctx.generics.where_clause.clone()),
        ];

        let where_clause = where_clauses
            .into_iter()
            .flatten()
            .reduce(|mut combined, clause| {
                combined.predicates.extend(clause.predicates);
                combined
            });

        Generics {
            params,
            where_clause,
        }
    }

    fn builder_ident(&self) -> syn::Ident {
        if let Some(builder_type) = &self.params.base.builder_type {
            return builder_type.clone();
        }

        if self.is_method_new() {
            return quote::format_ident!("{}Builder", self.self_ty_prefix().unwrap_or_default());
        }

        let pascal_case_func = self.norm_func.sig.ident.to_pascal_case();

        quote::format_ident!(
            "{}{pascal_case_func}Builder",
            self.self_ty_prefix().unwrap_or_default()
        )
    }

    pub(crate) fn adapted_func(&self) -> Result<syn::ItemFn> {
        let mut orig = self.orig_func.clone();

        let params = self.params.expose_positional_fn.as_ref();

        orig.vis = params
            .map(|params| {
                params
                    .vis
                    .clone()
                    // If exposing of positional fn is enabled without an explicit
                    // visibility, then just use the visibility of the original function.
                    .unwrap_or_else(|| self.norm_func.vis.clone())
            })
            // By default we change the positional function's visibility to private
            // to avoid exposing it to the surrounding code. The surrounding code is
            // supposed to use this function through the builder only.
            //
            // Not that this doesn't guarantee that adjacent code in this module can't
            // access the function, therefore we rename it below.
            .unwrap_or(syn::Visibility::Inherited);

        let orig_ident = orig.sig.ident.clone();

        if let Some(params) = params {
            let has_no_value = matches!(
                params.as_ref(),
                ExposePositionalFnParams {
                    name: None,
                    vis: None,
                }
            );

            if has_no_value && !self.is_method_new() {
                bail!(
                    &params.span(),
                    "Positional function identifier is required. It must be \
                    specified with `#[builder(expose_positional_fn = function_name_here)]`"
                )
            }
        }

        orig.sig.ident = params
            .and_then(|params| {
                params
                    .name
                    .clone()
                    // We treat `new` method specially. In this case we already know the best
                    // default name for the positional function, which is `new` itself.
                    .or_else(|| self.is_method_new().then_some(orig.sig.ident))
            })
            // By default we don't want to expose the positional function, so we
            // hide it under a generated name to avoid name conflicts.
            .unwrap_or_else(|| quote::format_ident!("__orig_{}", orig_ident.raw_name()));

        strip_known_attrs_from_args(&mut orig.sig);

        // Remove all doc comments from the function itself to avoid docs duplication
        // which may lead to duplicating doc tests, which in turn implies repeated doc
        // tests execution, which means worse tests performance.
        //
        // Also remove any `#[builder]` attributes that were meant for this proc macro.
        orig.attrs
            .retain(|attr| !attr.is_doc() && !attr.path().is_ident("builder"));

        let prefix = self
            .self_ty_prefix()
            .map(|self_ty_prefix| format!("{self_ty_prefix}::"))
            .unwrap_or_default();

        let builder_entry_fn_link = format!("{prefix}{orig_ident}",);

        let doc = format!(
            "Positional function equivalent of [`{builder_entry_fn_link}()`].\n\
            See its docs for details.",
        );

        orig.attrs.push(syn::parse_quote!(#[doc = #doc]));

        if self.params.expose_positional_fn.is_none() {
            orig.attrs.push(syn::parse_quote!(#[doc(hidden)]));
        }

        // It's fine if there are too many positional arguments in the function
        // because the whole purpose of this macro is to fight with this problem
        // at the call site by generating a builder, while keeping the fn definition
        // site the same with tons of positional arguments which don't harm readability
        // there because their names are explicitly specified at the definition site.
        orig.attrs
            .push(syn::parse_quote!(#[allow(clippy::too_many_arguments)]));

        Ok(orig)
    }

    fn is_method_new(&self) -> bool {
        self.impl_ctx.is_some() && self.norm_func.sig.ident == "new"
    }

    pub(crate) fn into_builder_gen_ctx(self) -> Result<BuilderGenCtx> {
        let receiver = self.assoc_method_ctx();

        if self.impl_ctx.is_none() {
            let explanation = "\
                but #[bon] attribute \
                is absent on top of the impl block. This additional #[bon] \
                attribute on the impl block is required for the macro to see \
                the type of `Self` and properly generate the builder struct \
                definition adjacently to the impl block.";

            if let Some(receiver) = &self.orig_func.sig.receiver() {
                bail!(
                    &receiver.self_token,
                    "Function contains a `self` parameter {explanation}"
                );
            }

            let mut ctx = FindSelfReference::default();
            ctx.visit_item_fn(&self.orig_func);
            if let Some(self_span) = ctx.self_span {
                bail!(
                    &self_span,
                    "Function contains a `Self` type reference {explanation}"
                );
            }
        }

        let builder_ident = self.builder_ident();
        let builder_private_impl_ident =
            quote::format_ident!("__{}PrivateImpl", builder_ident.raw_name());
        let builder_state_trait_ident = quote::format_ident!("__{}State", builder_ident.raw_name());

        let members: Vec<_> = self
            .norm_func
            .sig
            .inputs
            .iter()
            .filter_map(syn::FnArg::as_typed)
            .map(Member::from_typed_fn_arg)
            .try_collect()?;

        let generics = self.generics();

        let finish_func_body = FnCallBody {
            func: self.adapted_func()?,
            impl_ctx: self.impl_ctx.clone(),
        };

        let is_method_new = self.is_method_new();

        // Special case for `new` methods. We rename them to `builder`
        // since this is the name that is used in the builder pattern
        let start_func_ident = if is_method_new {
            syn::Ident::new("builder", self.norm_func.sig.ident.span())
        } else {
            self.norm_func.sig.ident.clone()
        };

        let finish_func_ident = self.params.base.finish_fn.unwrap_or_else(|| {
            // For `new` methods the `build` finisher is more conventional
            let name = if is_method_new { "build" } else { "call" };

            syn::Ident::new(name, start_func_ident.span())
        });

        let finish_func = FinishFunc {
            ident: finish_func_ident,
            unsafety: self.norm_func.sig.unsafety,
            asyncness: self.norm_func.sig.asyncness,
            body: Box::new(finish_func_body),
            output: self.norm_func.sig.output,
        };

        let start_func = StartFunc {
            ident: start_func_ident,

            // No override for visibility for the start fn is provided here.
            // It's supposed to be the same as the original function's visibility.
            vis: None,

            attrs: self
                .norm_func
                .attrs
                .into_iter()
                .filter(|attr| attr.is_doc())
                .collect(),

            generics: Some(Generics {
                params: Vec::from_iter(self.norm_func.sig.generics.params),
                where_clause: self.norm_func.sig.generics.where_clause,
            }),
        };

        let ctx = BuilderGenCtx {
            members,
            builder_ident,
            builder_private_impl_ident,
            builder_state_trait_ident,

            assoc_method_ctx: receiver,
            generics,
            vis: self.norm_func.vis,

            start_func,
            finish_func,
        };

        Ok(ctx)
    }
}

struct FnCallBody {
    func: syn::ItemFn,
    impl_ctx: Option<Rc<ImplCtx>>,
}

impl FinishFuncBody for FnCallBody {
    fn gen(&self, member_exprs: &[MemberExpr<'_>]) -> TokenStream2 {
        let asyncness = &self.func.sig.asyncness;
        let maybe_await = asyncness.is_some().then(|| quote!(.await));

        // Filter out lifetime generic arguments, because they are not needed
        // to be specified explicitly when calling the function. This also avoids
        // the problem that it's not always possible to specify lifetimes in
        // the turbofish syntax. See the problem of late-bound lifetimes specification
        // in the issue https://github.com/rust-lang/rust/issues/42868
        let generic_args = self
            .func
            .sig
            .generics
            .params
            .iter()
            .filter(|arg| !matches!(arg, syn::GenericParam::Lifetime(_)))
            .map(generic_param_to_arg);

        let prefix = self
            .func
            .sig
            .receiver()
            .map(|receiver| {
                let self_token = &receiver.self_token;
                quote!(#self_token.__private_impl.receiver.)
            })
            .or_else(|| {
                let self_ty = &self.impl_ctx.as_deref()?.self_ty;
                Some(quote!(<#self_ty>::))
            });

        let func_ident = &self.func.sig.ident;

        let member_exprs = member_exprs.iter().map(|member| &member.expr);

        quote! {
            #prefix #func_ident::<#(#generic_args,)*>(
                #( #member_exprs ),*
            )
            #maybe_await
        }
    }
}

/// Remove all doc comments attributes from function arguments, because they are
/// not valid in that position in regular Rust code. The cool trick is that they
/// are still valid syntactically when a proc macro like this one pre-processes
/// them and removes them from the expanded code. We use the doc comments to put
/// them on the generated setter methods.
///
/// We also strip all `builder(...)` attributes because this macro processes them
/// and they aren't needed in the output.
fn strip_known_attrs_from_args(sig: &mut syn::Signature) {
    for arg in &mut sig.inputs {
        arg.attrs_mut()
            .retain(|attr| !attr.is_doc() && !attr.path().is_ident("builder"));
    }
}

/// To merge generic params we need to make sure lifetimes are always the first
/// in the resulting list according to Rust syntax restrictions.
fn merge_generic_params(
    left: &Punctuated<syn::GenericParam, syn::Token![,]>,
    right: &Punctuated<syn::GenericParam, syn::Token![,]>,
) -> Vec<syn::GenericParam> {
    // False-positive. Peek is used inside of `peeking_take_while`
    #[allow(clippy::unused_peekable)]
    let (mut left, mut right) = (left.iter().peekable(), right.iter().peekable());

    let is_lifetime = |param: &&_| matches!(param, &&syn::GenericParam::Lifetime(_));

    let left_lifetimes = left.peeking_take_while(is_lifetime);
    let right_lifetimes = right.peeking_take_while(is_lifetime);

    let mut generic_params = left_lifetimes.chain(right_lifetimes).cloned().collect_vec();
    generic_params.extend(left.chain(right).cloned());
    generic_params
}

impl Member {
    pub(crate) fn from_typed_fn_arg(arg: &syn::PatType) -> Result<Self> {
        let ident = match arg.pat.as_ref() {
            syn::Pat::Ident(pat) => Some(&pat.ident),
            _ => None,
        };

        Member::new(
            MemberOrigin::FnArg,
            &arg.attrs,
            ident.cloned(),
            arg.ty.clone(),
        )
    }
}

#[derive(Default)]
struct FindSelfReference {
    self_span: Option<Span>,
}

impl Visit<'_> for FindSelfReference {
    fn visit_item(&mut self, _: &syn::Item) {
        // Don't recurse into nested items. We are interested in the reference
        // to `Self` on the current item level
    }

    fn visit_path(&mut self, path: &syn::Path) {
        if self.self_span.is_some() {
            return;
        }
        syn::visit::visit_path(self, path);

        let Some(first_segment) = path.segments.first() else {
            return;
        };

        if first_segment.ident == "Self" {
            self.self_span = Some(first_segment.ident.span());
        }
    }
}
