use super::ReceiverCtx;
use crate::builder::builder_gen::{
    generic_param_to_arg, BuilderGenCtx, Field, FieldExpr, FinishFunc, FinishFuncBody, Generics,
    StartFunc,
};
use crate::builder::params::BuilderParams;
use crate::normalization::NormalizeSelfTy;
use darling::FromMeta;
use heck::AsPascalCase;
use itertools::Itertools;
use prox::prelude::*;
use quote::quote;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

#[derive(Debug, FromMeta)]
pub(crate) struct FuncInputParams {
    expose_positional_fn: Option<ExposePositionalFnParams>,

    #[darling(flatten)]
    base: BuilderParams,
}

#[derive(Debug)]
pub(crate) struct ExposePositionalFnParams {
    pub(crate) name: syn::Ident,
    pub(crate) vis: Option<syn::Visibility>,
}

impl FromMeta for ExposePositionalFnParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(quote!(#val))?;

            return Ok(Self { name, vis: None });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: syn::Ident,
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

    fn receiver_ctx(&self) -> Option<ReceiverCtx> {
        let receiver = self.norm_func.sig.receiver()?;
        let mut without_self_ty = receiver.ty.clone();
        let self_ty = &self.impl_ctx.as_deref()?.self_ty;

        NormalizeSelfTy { self_ty }.visit_type_mut(&mut without_self_ty);

        Some(ReceiverCtx {
            with_self_ty: receiver.clone(),
            without_self_ty,
        })
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
            })
            .map(|clause| syn::WhereClause {
                where_token: clause.where_token,
                predicates: clause.predicates,
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

        let pascal_case_func = AsPascalCase(self.norm_func.sig.ident.to_string());
        quote::format_ident!(
            "{}{pascal_case_func}Builder",
            self.self_ty_prefix().unwrap_or_default()
        )
    }

    pub(crate) fn adapted_func(&self) -> syn::ItemFn {
        let mut orig = self.orig_func.clone();

        let params = self.params.expose_positional_fn.as_ref();

        orig.vis = params
            .and_then(|params| params.vis.clone())
            // By default we change the positional function's visibility to private
            // to avoid exposing it to the surrounding code. The surrounding code is
            // supposed to use this function through the builder only.
            //
            // Not that this doesn't guarantee that adjacent code in this module can't
            // access the function, therefore we rename it below.
            .unwrap_or(syn::Visibility::Inherited);

        let orig_ident = orig.sig.ident.clone();
        orig.sig.ident = params
            .map(|params| params.name.clone())
            // By default we don't want to expose the positional function, so we
            // hide it under a generated name to avoid name conflicts.
            .unwrap_or_else(|| quote::format_ident!("__orig_{}", orig_ident.to_string()));

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

        // It's fine if there are too many positional arguments in the function
        // because the whole purpose of this macro is to fight with this problem
        // at the call site by generating a builder, while keeping the fn definition
        // site the same with tons of positional arguments which don't harm readability
        // there because their names are explicitly specified at the definition site.
        orig.attrs
            .push(syn::parse_quote!(#[allow(clippy::too_many_arguments)]));

        orig
    }

    pub(crate) fn into_builder_gen_ctx(self) -> Result<BuilderGenCtx> {
        let receiver = self.receiver_ctx();
        if let Some(receiver) = &receiver {
            if self.impl_ctx.is_none() {
                prox::bail!(
                    &receiver.with_self_ty.self_token,
                    "Function contains a `self` parameter, but #[bon] attribute \
                    is absent on top of the impl block. This additional #[bon] \
                    attribute on the impl block is required for the macro to see \
                    the type of `Self` and properly generate the builder struct \
                    definition adjacently to the impl block."
                );
            }
        }

        let builder_ident = self.builder_ident();
        let builder_private_impl_ident = quote::format_ident!("__{builder_ident}PrivateImpl");
        let builder_state_trait_ident = quote::format_ident!("__{builder_ident}State");

        let fields: Vec<_> = self
            .norm_func
            .sig
            .inputs
            .iter()
            .filter_map(syn::FnArg::as_typed)
            .map(Field::from_typed_fn_arg)
            .try_collect()?;

        let generics = self.generics();

        let finish_func_body = FnCallBody {
            func: self.adapted_func(),
            impl_ctx: self.impl_ctx.clone(),
        };

        let start_func_ident = self.norm_func.sig.ident;

        let finish_func_ident = self.params.base.finish_fn.unwrap_or_else(|| {
            let name = if self.impl_ctx.is_some() && receiver.is_none() {
                // Associated methods of an impl block without the receiver likely create an instance of
                // `Self` so we have a bit different convention for default exit function name in this case.
                "build"
            } else {
                "call"
            };

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
            fields,
            builder_ident,
            builder_private_impl_ident,
            builder_state_trait_ident,

            receiver,
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
    fn gen(&self, field_exprs: &[FieldExpr<'_>]) -> TokenStream2 {
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

        let field_exprs = field_exprs.iter().map(|field| &field.expr);

        quote! {
            #prefix #func_ident::<#(#generic_args,)*>(
                #( #field_exprs ),*
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

impl Field {
    pub(crate) fn from_typed_fn_arg(arg: &syn::PatType) -> Result<Self> {
        let syn::Pat::Ident(pat) = arg.pat.as_ref() else {
            // We may allow setting a name for the builder method in parameter
            // attributes and relax this requirement
            prox::bail!(
                &arg.pat,
                "Only simple identifiers in function arguments supported \
                to infer the name of builder methods"
            );
        };

        Field::new(&arg.attrs, pat.ident.clone(), arg.ty.clone())
    }
}
