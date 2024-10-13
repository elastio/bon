use super::builder_params::BuilderParams;
use super::{
    AssocMethodCtx, AssocMethodReceiverCtx, BuilderGenCtx, FinishFn, FinishFnBody, Generics,
    Member, MemberOrigin, RawMember,
};
use crate::builder::builder_gen::models::{BuilderGenCtxParams, BuilderTypeParams, StartFnParams};
use crate::normalization::{GenericsNamespace, NormalizeSelfTy, SyntaxVariant};
use crate::parsing::{ItemParams, SpannedKey};
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromMeta;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use std::borrow::Cow;

#[derive(Debug, FromMeta)]
pub(crate) struct FnInputParams {
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

pub(crate) struct FnInputCtx<'a> {
    pub(crate) namespace: &'a GenericsNamespace,
    pub(crate) fn_item: SyntaxVariant<syn::ItemFn>,
    pub(crate) impl_ctx: Option<Rc<ImplCtx>>,
    pub(crate) params: FnInputParams,
}

pub(crate) struct ImplCtx {
    pub(crate) self_ty: Box<syn::Type>,
    pub(crate) generics: syn::Generics,

    /// Lint suppressions from the original item that will be inherited by all items
    /// generated by the macro. If the original syntax used `#[expect(...)]`,
    /// then it must be represented as `#[allow(...)]` here.
    pub(crate) allow_attrs: Vec<syn::Attribute>,
}

impl FnInputCtx<'_> {
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

    fn assoc_method_ctx(&self) -> Result<Option<AssocMethodCtx>> {
        let self_ty = match self.impl_ctx.as_deref() {
            Some(impl_ctx) => impl_ctx.self_ty.clone(),
            None => return Ok(None),
        };

        Ok(Some(AssocMethodCtx {
            self_ty,
            receiver: self.assoc_method_receiver_ctx()?,
        }))
    }

    fn assoc_method_receiver_ctx(&self) -> Result<Option<AssocMethodReceiverCtx>> {
        let receiver = match self.fn_item.norm.sig.receiver() {
            Some(receiver) => receiver,
            None => return Ok(None),
        };

        if let [attr, ..] = receiver.attrs.as_slice() {
            bail!(
                attr,
                "attributes on the receiver are not supported in the #[builder] macro"
            );
        }

        let self_ty = match self.impl_ctx.as_deref() {
            Some(impl_ctx) => &impl_ctx.self_ty,
            None => return Ok(None),
        };

        let mut without_self_keyword = receiver.ty.clone();

        NormalizeSelfTy { self_ty }.visit_type_mut(&mut without_self_keyword);

        Ok(Some(AssocMethodReceiverCtx {
            with_self_keyword: receiver.clone(),
            without_self_keyword,
        }))
    }

    fn generics(&self) -> Generics {
        let impl_ctx = self.impl_ctx.as_ref();
        let norm_fn_params = &self.fn_item.norm.sig.generics.params;
        let params = impl_ctx
            .map(|impl_ctx| merge_generic_params(&impl_ctx.generics.params, norm_fn_params))
            .unwrap_or_else(|| norm_fn_params.iter().cloned().collect());

        let where_clauses = [
            self.fn_item.norm.sig.generics.where_clause.clone(),
            impl_ctx.and_then(|impl_ctx| impl_ctx.generics.where_clause.clone()),
        ];

        let where_clause = where_clauses
            .into_iter()
            .flatten()
            .reduce(|mut combined, clause| {
                combined.predicates.extend(clause.predicates);
                combined
            });

        Generics::new(params, where_clause)
    }

    fn builder_ident(&self) -> syn::Ident {
        let user_override = self.params.base.builder_type.name.as_deref();

        if let Some(user_override) = user_override {
            return user_override.clone();
        }

        if self.is_method_new() {
            return format_ident!("{}Builder", self.self_ty_prefix().unwrap_or_default());
        }

        let pascal_case_fn = self.fn_item.norm.sig.ident.snake_to_pascal_case();

        format_ident!(
            "{}{pascal_case_fn}Builder",
            self.self_ty_prefix().unwrap_or_default(),
        )
    }

    pub(crate) fn adapted_fn(&self) -> Result<syn::ItemFn> {
        let mut orig = self.fn_item.orig.clone();

        let params = self.params.expose_positional_fn.as_ref();

        orig.vis = params
            .map(|params| {
                params
                    .vis
                    .clone()
                    // If exposing of positional fn is enabled without an explicit
                    // visibility, then just use the visibility of the original function.
                    .unwrap_or_else(|| self.fn_item.norm.vis.clone())
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
                    .or_else(|| self.is_method_new().then(|| orig.sig.ident))
            })
            // By default we don't want to expose the positional function, so we
            // hide it under a generated name to avoid name conflicts.
            .unwrap_or_else(|| format_ident!("__orig_{}", orig_ident.raw_name()));

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
            orig.attrs.extend([syn::parse_quote!(#[doc(hidden)])]);
        }
        orig.attrs.push(syn::parse_quote!(#[allow(
            // It's fine if there are too many positional arguments in the function
            // because the whole purpose of this macro is to fight with this problem
            // at the call site by generating a builder, while keeping the fn definition
            // site the same with tons of positional arguments which don't harm readability
            // there because their names are explicitly specified at the definition site.
            clippy::too_many_arguments,

            // It's fine to use many bool arguments in the function signature because
            // all of the will be named at the call site
            clippy::fn_params_excessive_bools,
        )]));

        Ok(orig)
    }

    fn is_method_new(&self) -> bool {
        self.impl_ctx.is_some() && self.fn_item.norm.sig.ident == "new"
    }

    pub(crate) fn into_builder_gen_ctx(self) -> Result<BuilderGenCtx> {
        let assoc_method_ctx = self.assoc_method_ctx()?;

        if self.impl_ctx.is_none() {
            let explanation = "\
                but #[bon] attribute is absent on top of the impl block; this \
                additional #[bon] attribute on the impl block is required for \
                the macro to see the type of `Self` and properly generate
                the builder struct definition adjacently to the impl block.";

            if let Some(receiver) = &self.fn_item.orig.sig.receiver() {
                bail!(
                    &receiver.self_token,
                    "function contains a `self` parameter {explanation}"
                );
            }

            let mut ctx = FindSelfReference::default();
            ctx.visit_item_fn(&self.fn_item.orig);
            if let Some(self_span) = ctx.self_span {
                bail!(
                    &self_span,
                    "function contains a `Self` type reference {explanation}"
                );
            }
        }

        let builder_ident = self.builder_ident();

        let typed_args = self
            .fn_item
            .apply_ref(|fn_item| fn_item.sig.inputs.iter().filter_map(syn::FnArg::as_typed));

        let members = typed_args
            .norm
            .zip(typed_args.orig)
            .map(|(norm_arg, orig_arg)| {
                let pat = match norm_arg.pat.as_ref() {
                    syn::Pat::Ident(pat) => pat,
                    _ => bail!(
                        &orig_arg.pat,
                        "use a simple `identifier: type` syntax for the function argument; \
                        destructuring patterns in arguments aren't supported by the `#[builder]`",
                    ),
                };

                let ty = SyntaxVariant {
                    norm: norm_arg.ty.clone(),
                    orig: orig_arg.ty.clone(),
                };

                Ok(RawMember {
                    attrs: &norm_arg.attrs,
                    ident: pat.ident.clone(),
                    ty,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let members = Member::from_raw(&self.params.base.on, MemberOrigin::FnArg, members)?;

        let generics = self.generics();

        let finish_fn_body = FnCallBody {
            sig: self.adapted_fn()?.sig,
            impl_ctx: self.impl_ctx.clone(),
        };

        let is_method_new = self.is_method_new();

        // Special case for `new` methods. We rename them to `builder`
        // since this is the name that is used in the builder pattern
        let start_fn_ident = if is_method_new {
            format_ident!("builder")
        } else {
            self.fn_item.norm.sig.ident.clone()
        };

        let ItemParams {
            name: finish_fn_ident,
            vis: finish_fn_vis,
            docs: finish_fn_docs,
        } = self.params.base.finish_fn;

        let finish_fn_ident = finish_fn_ident
            .map(SpannedKey::into_value)
            .unwrap_or_else(|| {
                // For `new` methods the `build` finisher is more conventional
                if is_method_new {
                    format_ident!("build")
                } else {
                    format_ident!("call")
                }
            });

        let finish_fn_docs = finish_fn_docs
            .map(SpannedKey::into_value)
            .unwrap_or_else(|| {
                vec![syn::parse_quote! {
                    /// Finishes building and performs the requested action.
                }]
            });

        let finish_fn = FinishFn {
            ident: finish_fn_ident,
            vis: finish_fn_vis.map(SpannedKey::into_value),
            unsafety: self.fn_item.norm.sig.unsafety,
            asyncness: self.fn_item.norm.sig.asyncness,
            must_use: get_must_use_attribute(&self.fn_item.norm.attrs)?,
            body: Box::new(finish_fn_body),
            output: self.fn_item.norm.sig.output,
            attrs: finish_fn_docs,
        };

        let fn_allows = self
            .fn_item
            .norm
            .attrs
            .iter()
            .filter_map(syn::Attribute::to_allow);

        let allow_attrs = self
            .impl_ctx
            .as_ref()
            .into_iter()
            .flat_map(|impl_ctx| impl_ctx.allow_attrs.iter().cloned())
            .chain(fn_allows)
            .collect();

        let start_fn = StartFnParams {
            ident: start_fn_ident,

            // No override for visibility for the start fn is provided here.
            // It's supposed to be the same as the original function's visibility.
            vis: None,

            attrs: self
                .fn_item
                .norm
                .attrs
                .into_iter()
                .filter(<_>::is_doc)
                .collect(),

            // Override on the start fn to use the the generics from the
            // target function itself. We don't need to duplicate the generics
            // from the impl block here.
            generics: Some(Generics::new(
                Vec::from_iter(self.fn_item.norm.sig.generics.params),
                self.fn_item.norm.sig.generics.where_clause,
            )),
        };

        let builder_type = self.params.base.builder_type;
        let builder_type = BuilderTypeParams {
            ident: builder_ident,
            derives: self.params.base.derive,
            docs: builder_type.docs.map(SpannedKey::into_value),
            vis: builder_type.vis.map(SpannedKey::into_value),
        };

        BuilderGenCtx::new(BuilderGenCtxParams {
            namespace: Cow::Borrowed(self.namespace),
            members,

            allow_attrs,

            on_params: self.params.base.on,

            assoc_method_ctx,
            generics,
            orig_item_vis: self.fn_item.norm.vis,

            builder_type,
            state_mod: self.params.base.state_mod,
            start_fn,
            finish_fn,
        })
    }
}

struct FnCallBody {
    sig: syn::Signature,
    impl_ctx: Option<Rc<ImplCtx>>,
}

impl FinishFnBody for FnCallBody {
    fn generate(&self, ctx: &BuilderGenCtx) -> TokenStream {
        let asyncness = &self.sig.asyncness;
        let maybe_await = asyncness.is_some().then(|| quote!(.await));

        // Filter out lifetime generic arguments, because they are not needed
        // to be specified explicitly when calling the function. This also avoids
        // the problem that it's not always possible to specify lifetimes in
        // the turbofish syntax. See the problem of late-bound lifetimes specification
        // in the issue https://github.com/rust-lang/rust/issues/42868
        let generic_args = self
            .sig
            .generics
            .params
            .iter()
            .filter(|arg| !matches!(arg, syn::GenericParam::Lifetime(_)))
            .map(syn::GenericParam::to_generic_argument);

        let prefix = self
            .sig
            .receiver()
            .map(|_| {
                let receiver_field = &ctx.idents_pool.receiver;
                quote!(self.#receiver_field.)
            })
            .or_else(|| {
                let self_ty = &self.impl_ctx.as_deref()?.self_ty;
                Some(quote!(<#self_ty>::))
            });

        let fn_ident = &self.sig.ident;

        // The variables with values of members are in scope for this expression.
        let member_vars = ctx.members.iter().map(Member::orig_ident);

        quote! {
            #prefix #fn_ident::<#(#generic_args,)*>(
                #( #member_vars ),*
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
    let is_lifetime = |param: &&_| matches!(param, &&syn::GenericParam::Lifetime(_));

    let (left_lifetimes, left_rest): (Vec<_>, Vec<_>) = left.iter().partition(is_lifetime);
    let (right_lifetimes, right_rest): (Vec<_>, Vec<_>) = right.iter().partition(is_lifetime);

    left_lifetimes
        .into_iter()
        .chain(right_lifetimes)
        .chain(left_rest)
        .chain(right_rest)
        .cloned()
        .collect()
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

        let first_segment = match path.segments.first() {
            Some(first_segment) => first_segment,
            _ => return,
        };

        if first_segment.ident == "Self" {
            self.self_span = Some(first_segment.ident.span());
        }
    }
}

fn get_must_use_attribute(attrs: &[syn::Attribute]) -> Result<Option<syn::Attribute>> {
    let mut iter = attrs
        .iter()
        .filter(|attr| attr.meta.path().is_ident("must_use"));

    let result = iter.next();

    if let Some(second) = iter.next() {
        bail!(
            second,
            "Found multiple #[must_use], but bon only works with exactly one (or less)."
        );
    }

    if let Some(attr) = result {
        if let syn::AttrStyle::Inner(_) = attr.style {
            bail!(
                attr,
                "The #[must_use] attribute must be placed on the function itself, \
                not inside it."
            );
        }
    }

    Ok(result.cloned())
}
