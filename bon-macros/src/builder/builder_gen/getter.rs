use proc_macro2::TokenStream;
use quote::quote;

use super::{BuilderGenCtx, IdentExt, NamedMember};

pub(crate) struct GetterCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

struct GetterItem {
    name: syn::Ident,
    vis: syn::Visibility,
    docs: Vec<syn::Attribute>,
}

impl<'a> GetterCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self { base, member }
    }

    pub(crate) fn getter_method(&self) -> TokenStream {
        let Some(GetterItem { name, vis, docs }) = GetterItem::new(self) else {
            return quote! {};
        };

        let index = &self.member.index;
        let ty = self.member.underlying_norm_ty();

        let (return_type, body) = if self.member.is_required() {
            (
                quote! { &#ty },
                quote! { unsafe { ::std::option::Option::unwrap_unchecked(self.__unsafe_private_named.#index.as_ref()) } },
            )
        } else {
            (
                quote! { ::core::option::Option<&#ty> },
                quote! { self.__unsafe_private_named.#index.as_ref() },
            )
        };

        let state_var = &self.base.state_var;
        let member_pascal = &self.member.name.pascal;
        let state_mod = &self.base.state_mod.ident;

        quote! {
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We don't want to avoid using `impl Trait` in the setter. This way
                // the setter signature is easier to read, and anyway if you want to
                // specify a type hint for the method that accepts an `impl Into`, then
                // your design of this setter already went wrong.
                clippy::impl_trait_in_params,
                clippy::missing_const_for_fn,
            )]
            #[inline(always)]
            #vis fn #name(&self) -> #return_type
            where #state_var::#member_pascal: #state_mod::IsSet,
            {
                #body
            }
        }
    }
}

impl GetterItem {
    fn new(ctx: &GetterCtx<'_>) -> Option<Self> {
        let GetterCtx { member, base } = ctx;

        let Some(spanned_keyed_config) = member.config.getter.as_ref() else {
            return None;
        };

        let common_name = spanned_keyed_config.name();
        let common_vis = spanned_keyed_config.vis();
        let common_docs = spanned_keyed_config.docs();

        let orig_item_required = member.is_required();

        Some(GetterItem {
            name: common_name.cloned().unwrap_or_else(|| {
                if orig_item_required {
                    syn::Ident::new(
                        &format!("get_{}", member.name.snake.raw_name()),
                        member.name.snake.span(),
                    )
                } else {
                    syn::Ident::new(
                        &format!("maybe_get_{}", member.name.snake.raw_name()),
                        member.name.snake.span(),
                    )
                }
            }),
            vis: common_vis.unwrap_or(&base.builder_type.vis).clone(),
            docs: common_docs.map(|d| d.to_vec()).unwrap_or_else(|| {
                const HEADER: &str = "_**Getter.**_\n\n";

                std::iter::once(syn::parse_quote!(#[doc = #HEADER]))
                    .chain(member.docs.iter().cloned())
                    .collect()
            }),
        })
    }
}
