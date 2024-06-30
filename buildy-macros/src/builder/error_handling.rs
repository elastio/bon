use super::normalization;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::Parse;

pub(crate) fn error_into_token_stream(err: prox::Error, item: TokenStream2) -> TokenStream2 {
    let compile_error = err.write_errors();

    syn::parse2::<Fallback>(item)
        .map(|fallback| quote!(#compile_error #fallback))
        .unwrap_or_else(|_| compile_error)
}

/// This is used in error handling for better IDE experience. For example, while
/// the developer is writing the function body they'll have a bunch of syntax
/// errors in the process. While that happens the proc macro should just output
/// the same code that the developer wrote with a separate compile error entry.
/// This keeps the syntax highlighting and IDE type analysis, completions and
/// other hints features working.
///
/// This utility parses the header of the function keeping the original function
/// body the same. It strips doc comments on function arguments when tokenized
/// to avoid the IDE from showing errors that "doc comments aren't allowed on
/// function arguments".
struct Fallback {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    sig: syn::Signature,
    rest: TokenStream2,
}

impl Parse for Fallback {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            sig: {
                let mut sig: syn::Signature = input.parse()?;
                normalization::strip_doc_comments_from_args(&mut sig);
                sig
            },
            rest: input.parse()?,
        })
    }
}

impl ToTokens for Fallback {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.rest.to_tokens(tokens);
    }
}
