use crate::util::prelude::*;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser};
use syn::{token, Token};

pub(crate) fn parse_comma_separated_meta(input: ParseStream<'_>) -> syn::Result<Vec<Meta>> {
    let mut output = vec![];

    while !input.is_empty() {
        let value = input.parse::<Meta>()?;

        output.push(value);

        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                break;
            }
            input.parse::<TokenTree>()?;
        }
    }

    Ok(output)
}

#[derive(Clone, Debug)]
pub(crate) enum Meta {
    None,
    Path(syn::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

impl Parse for Meta {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = loop {
            let path = input.call(syn::Path::parse_mod_style).ok();

            if let Some(path) = path {
                break path;
            }

            if input.parse::<TokenTree>().is_err() {
                return Ok(Self::None);
            }
        };

        let meta = if input.peek(token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(token::Bracket) {
            let content;
            syn::bracketed!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(Token![=]) {
            Self::NameValue(MetaNameValue {
                path,
                eq_token: input.parse()?,
                value: input.parse().ok(),
            })
        } else {
            Meta::Path(path)
        };

        Ok(meta)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MetaList {
    pub(crate) path: syn::Path,
    pub(crate) tokens: TokenStream2,
}

#[derive(Clone, Debug)]
pub(crate) struct MetaNameValue {
    pub(crate) path: syn::Path,

    #[allow(dead_code)]
    pub(crate) eq_token: syn::Token![=],

    #[allow(dead_code)]
    pub(crate) value: Option<syn::Expr>,
}

fn paths_from_meta(meta: Vec<Meta>) -> Vec<syn::Path> {
    meta.into_iter()
        .filter_map(|meta| match meta {
            Meta::Path(path) => Some(path),
            Meta::NameValue(meta) => Some(meta.path),
            Meta::List(meta) => Some(meta.path),
            _ => None,
        })
        .collect()
}

pub(crate) fn generate_completions(meta: Vec<Meta>) -> TokenStream2 {
    let completions = CompletionsSchema::with_children(
        "builder_top_level",
        vec![
            CompletionsSchema::leaf("expose_positional_fn"),
            CompletionsSchema::leaf("start_fn"),
        ],
    );

    let completion_triggers = completions.generate_completion_triggers(meta, &[]);

    quote! {
        #[cfg(rust_analyzer)]
        const _: () = {
            #completion_triggers
        };
    }
}

struct CompletionsSchema {
    key: &'static str,
    children: Vec<CompletionsSchema>,
}

impl CompletionsSchema {
    fn leaf(key: &'static str) -> Self {
        Self {
            key,
            children: vec![],
        }
    }

    fn with_children(key: &'static str, children: Vec<CompletionsSchema>) -> Self {
        Self { key, children }
    }

    fn generate_completion_triggers(
        &self,
        meta: Vec<Meta>,
        module_prefix: &[&syn::Ident],
    ) -> TokenStream2 {
        let module_suffix = syn::Ident::new(self.key, Span::call_site());
        let module_name = module_prefix
            .iter()
            .copied()
            .chain([&module_suffix])
            .collect::<Vec<_>>();

        let child_completion_triggers = self
            .children
            .iter()
            .map(|child| {
                let child_metas = meta
                    .iter()
                    .filter_map(|meta| {
                        let meta = match meta {
                            Meta::List(meta) => meta,
                            _ => return None,
                        };

                        if !meta.path.is_ident(&child.key) {
                            return None;
                        }

                        parse_comma_separated_meta.parse2(meta.tokens.clone()).ok()
                    })
                    .concat();

                child.generate_completion_triggers(child_metas, &module_name)
            })
            .collect::<Vec<_>>();

        let paths = paths_from_meta(meta);
        let module_name_snake_case =
            syn::Ident::new(&module_name.iter().join("_"), Span::call_site());

        quote! {
            mod #module_name_snake_case {
                // We separately import
                use ::bon::private::ide #(::#module_name)* ::*;
                use self::{ #( #paths as _, )* };
            }

            #(#child_completion_triggers)*
        }
    }
}
