mod parse;
mod visit;

use crate::util::prelude::*;
use parse::CfgSyntax;
use quote::{quote, ToTokens};
use std::collections::BTreeSet;

pub(crate) enum ExpansionOutput {
    Expanded {
        params: TokenStream2,
        item: syn::Item,
    },
    Recurse(TokenStream2),
}

pub(crate) struct ExpandCfg {
    pub(crate) macro_path: syn::Path,
    pub(crate) params: TokenStream2,
    pub(crate) item: syn::Item,
}

impl ExpandCfg {
    pub(crate) fn expand_cfg(mut self) -> Result<ExpansionOutput> {
        let predicates = self.collect_predicates()?;

        if predicates.is_empty() {
            return Ok(ExpansionOutput::Expanded {
                params: self.params,
                item: self.item,
            });
        }

        eprintln!("{}", self.params);

        let predicate_results = match parse::parse_predicate_results(&mut self.params)? {
            Some(predicate_results) => predicate_results,
            None => return Ok(self.into_recursion(&predicates)),
        };

        let true_predicates: BTreeSet<_> = predicates
            .iter()
            .map(ToString::to_string)
            .zip(predicate_results)
            .filter(|(_, result)| *result)
            .map(|(predicate, _)| predicate)
            .collect();

        visit::visit_attrs(&mut self.item, |attrs| eval_cfgs(&true_predicates, attrs))?;

        // Collect predicates again after the cfgs were evaluated. This is needed
        // because cfgs may create new cfgs e.g.: `#[cfg_attr(foo, cfg_attr(bar, ...))]`.
        let predicates = self.collect_predicates()?;

        if predicates.is_empty() {
            return Ok(ExpansionOutput::Expanded {
                params: self.params,
                item: self.item,
            });
        }

        Ok(self.into_recursion(&predicates))
    }

    /// There is no mutation happening here, but we just reuse the same
    /// visitor implementation that works with mutable references.
    fn collect_predicates(&mut self) -> Result<Vec<TokenStream2>> {
        let mut predicates = vec![];
        let mut visited = BTreeSet::new();

        visit::visit_attrs(&mut self.item, |attrs| {
            for attr in attrs {
                let cfg_syntax = match CfgSyntax::from_meta(&attr.meta)? {
                    Some(cfg_syntax) => cfg_syntax,
                    None => return Ok(true),
                };

                let predicate = match cfg_syntax {
                    CfgSyntax::Cfg(predicate) => predicate,
                    CfgSyntax::CfgAttr(cfg_attr) => cfg_attr.predicate.to_token_stream(),
                };

                if visited.insert(predicate.to_string()) {
                    predicates.push(predicate);
                }
            }

            Ok(true)
        })?;

        Ok(predicates)
    }

    fn into_recursion(self, predicates: &[TokenStream2]) -> ExpansionOutput {
        let Self {
            params,
            item,
            macro_path,
        } = &self;

        let recursive_expansion = quote! {
            ::bon::expand_cfg_callback! {
                (
                    #((#predicates),)*
                )
                ()
                #macro_path,
                (
                    #params
                )
                #item
            }
        };

        ExpansionOutput::Recurse(recursive_expansion)
    }
}

fn eval_cfgs(true_predicates: &BTreeSet<String>, attrs: &mut Vec<syn::Attribute>) -> Result<bool> {
    let mut cfg_attr_expansions = vec![];

    for (i, attr) in attrs.iter().enumerate() {
        let Some(syntax) = CfgSyntax::from_meta(&attr.meta)? else {
            continue;
        };

        let expansion = match syntax {
            CfgSyntax::Cfg(predicate) => {
                if !true_predicates.contains(&predicate.to_string()) {
                    // The cfg predicate is false. No need to keep iterating
                    // because the entire syntax this attribute is attached to
                    // should be removed. Signal the caller to remove it via `false`.
                    return Ok(false);
                }

                // Just remove the attribute. It evaluated to `true`
                None
            }
            CfgSyntax::CfgAttr(cfg_attr) => {
                let predicate = cfg_attr.predicate.to_token_stream().to_string();

                // We can't both iterate over the attributes and mutate them,
                // so collect the planned actions in a separate vector, and
                // do the mutations after the iteration.

                true_predicates
                    .contains(&predicate)
                    .then_some(cfg_attr.then_branch)
            }
        };

        cfg_attr_expansions.push((i, expansion));
    }

    // It's important to iterate in reverse order to avoid index invalidation
    for (i, metas) in cfg_attr_expansions.iter().rev() {
        let metas = if let Some(metas) = metas {
            metas
        } else {
            attrs.remove(*i);
            continue;
        };

        let replacement = metas.iter().map(|meta| syn::parse_quote!(#[#meta]));

        attrs.splice(i..=i, replacement);
    }

    Ok(true)
}
