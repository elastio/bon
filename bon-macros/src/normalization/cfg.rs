use crate::util::prelude::*;
use quote::{quote, ToTokens};
use std::collections::{BTreeMap, BTreeSet};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

pub(crate) enum ExpansionOutput {
    Expanded,
    Recurse(TokenStream2),
}

#[cfg_attr(all(), must_use, doc = "Expands the `cfg` attribute")]
pub(crate) fn expand_cfg(
    macro_path: syn::Path,
    params: &mut TokenStream2,
    item: &mut syn::Item,
) -> Result<ExpansionOutput> {
    let mut predicates = WrapResult(Ok(CollectPredicates::default()));
    predicates.visit_item(item);
    let predicates = predicates.0?.predicates;

    if predicates.is_empty() {
        return Ok(ExpansionOutput::Expanded);
    }

    let predicate_results: WrapOption<PredicateResults> = syn::parse2(params.clone())?;

    let Some(predicate_results) = predicate_results.0 else {
        let recursive_expansion = quote! {
            ::bon::expand_cfg_callback! {
                (
                    #((#predicates),)*
                )
                ()
                macro_path,
                (
                    #params
                )
                #item
            }
        };
        return Ok(ExpansionOutput::Recurse(recursive_expansion));
    };

    let true_predicates: BTreeSet<_> = predicates
        .iter()
        .map(ToString::to_string)
        .zip(predicate_results.results)
        .filter(|(_, result)| *result)
        .collect();

    Ok(ExpansionOutput::Expanded)
}

// Newtypes over `Result` and `Option` to be able to implement traits for them
struct WrapResult<T>(Result<T>);
struct WrapOption<T>(Option<T>);

/// Represents a special directive inserted at the beginning of the macro parameters
/// that has the syntax `@cfgs(true, false, true)`. It delivers the results of cfg
/// evaluations to the macro.
struct PredicateResults {
    results: Vec<bool>,
    rest: TokenStream2,
}

impl Parse for WrapOption<PredicateResults> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if !input.peek(syn::Token![@]) {
            return Ok(Self(None));
        }

        input.parse::<syn::Token![@]>()?;
        input.parse::<kw::cfgs>()?;

        let results;
        syn::parenthesized!(results in input);

        let cfgs: Vec<bool> =
            Punctuated::<syn::LitBool, syn::Token![,]>::parse_terminated(&results)?
                .into_iter()
                .map(|bool| bool.value)
                .collect();

        let results = PredicateResults {
            results: cfgs,
            rest: input.parse()?,
        };

        Ok(Self(Some(results)))
    }
}

mod kw {
    syn::custom_keyword!(cfgs);
}

#[derive(Default)]
struct CollectPredicates {
    predicates: Vec<TokenStream2>,
    visited: BTreeSet<String>,
}

impl Visit<'_> for WrapResult<CollectPredicates> {
    fn visit_meta(&mut self, meta: &syn::Meta) {
        let Ok(ctx) = &mut self.0 else { return };

        let cfg_syntax = match CfgSyntax::from_meta(&meta) {
            Ok(Some(cfg_syntax)) => cfg_syntax,
            Ok(None) => return,
            Err(err) => {
                self.0 = Err(err.into());
                return;
            }
        };

        let predicate = match cfg_syntax {
            CfgSyntax::Cfg(predicate) => predicate,
            CfgSyntax::CfgAttr(cfg_attr) => cfg_attr.predicate.to_token_stream(),
        };

        if ctx.visited.insert(predicate.to_string()) {
            ctx.predicates.push(predicate);
        }
    }

    // We do expansion only for the function signatures and struct declarations.
    // So we can skip traversing the rest of the tree.
    fn visit_block(&mut self, _: &syn::Block) {
        return;
    }
    fn visit_expr(&mut self, _: &syn::Expr) {
        return;
    }
    fn visit_type(&mut self, _: &syn::Type) {
        return;
    }
}

enum CfgSyntax {
    Cfg(TokenStream2),
    CfgAttr(CfgAttr),
}

impl CfgSyntax {
    fn from_meta(meta: &syn::Meta) -> Result<Option<Self>> {
        let syn::Meta::List(meta) = meta else {
            return Ok(None);
        };

        if meta.path.is_ident("cfg") {
            return Ok(Some(Self::Cfg(meta.tokens.clone())));
        }

        if meta.path.is_ident("cfg_attr") {
            let cfg_attr = syn::parse2(meta.tokens.clone())?;
            return Ok(Some(Self::CfgAttr(cfg_attr)));
        }

        Ok(None)
    }
}

struct CfgAttr {
    predicate: syn::Meta,
    then_branch: Punctuated<syn::Meta, syn::Token![,]>,
}

impl Parse for CfgAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let predicate = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let then_branch = Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;

        Ok(Self {
            predicate,
            then_branch,
        })
    }
}

struct ExpandCfgs {
    true_predicates: BTreeSet<String>,
}

impl ExpandCfgs {
    fn eval_cfgs(&self, attrs: &mut Vec<syn::Attribute>) -> Result<bool> {
        let mut cfg_attr_expansions = vec![];

        for (i, attr) in attrs.iter().enumerate() {
            let Some(syntax) = CfgSyntax::from_meta(&attr.meta)? else {
                continue;
            };

            match syntax {
                CfgSyntax::Cfg(predicate) => {
                    if self.true_predicates.contains(&predicate.to_string()) {
                        continue;
                    }

                    // The cfg predicate is false. No need to keep iterating
                    // because the entire syntax this attribute is attached to
                    // should be removed. Signal the caller to remove it via `false`.
                    return Ok(false);
                }
                CfgSyntax::CfgAttr(cfg_attr) => {
                    let predicate = cfg_attr.predicate.to_token_stream().to_string();

                    // We can't both iterate over the attributes and mutate them,
                    // so collect the planned actions in a separate vector, and
                    // do the mutations after the iteration.
                    cfg_attr_expansions.push((
                        i,
                        self.true_predicates
                            .contains(&predicate)
                            .then_some(cfg_attr.then_branch),
                    ));
                }
            }
        }

        // It's important to iterate in reverse order to avoid index invalidation
        for (i, metas) in cfg_attr_expansions.iter().rev() {
            let Some(metas) = metas else {
                attrs.remove(*i);
                continue;
            };

            let replacement = metas.iter().map(|meta| syn::parse_quote!(#[#meta]));

            attrs.splice(i..=i, replacement);
        }

        Ok(true)
    }
}

impl VisitMut for WrapResult<ExpandCfgs> {
    fn visit_fields_named_mut(&mut self, fields: &mut syn::FieldsNamed) {
        let Ok(ctx) = &mut self.0 else { return };

        // Unforunatelly, there is no `retain` method in `Punctuated` so we
        // do this dance of `mem::take`, and pushing the fitting items back.
        for mut field in std::mem::take(&mut fields.named).into_pairs() {
            let eval_result = ctx.eval_cfgs(&mut field.value_mut().attrs);

            match eval_result {
                Ok(true) => fields.named.extend([field]),
                Ok(false) => {
                    // #[cfg(...)] was false, so remove (don't add) the field
                }
                Err(err) => {
                    self.0 = Err(err);
                    return;
                }
            }
        }
    }

    fn visit_item_struct_mut(&mut self, struct_item: &mut syn::ItemStruct) {
        let Ok(ctx) = &mut self.0 else { return };

        let eval_result = ctx.eval_cfgs(&mut struct_item.attrs);

        match eval_result {
            Ok(true) => {
                syn::visit_mut::visit_item_struct_mut(self, struct_item);
            }
            Ok(false) => {
                // #[cfg(...)] was false. We just don't do nothing. That
                // cfg will remove the struct by itself.
            }
            Err(err) => {
                self.0 = Err(err);
                return;
            }
        }
    }
}
