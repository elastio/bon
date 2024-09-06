use crate::util::prelude::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    syn::custom_keyword!(__cfgs);
}

pub(crate) fn parse_predicate_results(tokens: &mut TokenStream2) -> Result<Option<Vec<bool>>> {
    let results: WrapOption<PredicateResults> = syn::parse2(tokens.clone())?;

    let results = results.0.map(|results| {
        // Update the parameters to remove the `@cfgs(...)` prefix from them
        *tokens = results.rest;

        results.results
    });

    Ok(results)
}

// Newtypes over an `Option` to be able to implement trait on it
#[derive(Debug)]
struct WrapOption<T>(Option<T>);

/// Represents a special directive inserted at the beginning of the macro parameters
/// that has the syntax `@cfgs(true, false, true)`. It delivers the results of cfg
/// evaluations to the macro.
#[derive(Debug)]
pub(crate) struct PredicateResults {
    pub(crate) results: Vec<bool>,
    pub(crate) rest: TokenStream2,
}

impl Parse for WrapOption<PredicateResults> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if !input.peek(kw::__cfgs) {
            // We need to exhaust the input stream to avoid a "unexpected token" error
            input.parse::<TokenStream2>()?;

            return Ok(Self(None));
        }

        input.parse::<kw::__cfgs>()?;

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

pub(crate) enum CfgSyntax {
    Cfg(TokenStream2),
    CfgAttr(CfgAttr),
}

impl CfgSyntax {
    pub(crate) fn from_meta(meta: &syn::Meta) -> Result<Option<Self>> {
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

pub(crate) struct CfgAttr {
    pub(crate) predicate: syn::Meta,
    pub(crate) then_branch: Punctuated<syn::Meta, syn::Token![,]>,
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
