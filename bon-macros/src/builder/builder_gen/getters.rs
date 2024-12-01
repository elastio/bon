use super::{BuilderGenCtx, NamedMember};
use crate::util::prelude::*;

pub(crate) struct GettersCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

struct GetterItem {
    name: syn::Ident,
    vis: syn::Visibility,
    docs: Vec<syn::Attribute>,
}

impl<'a> GettersCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self { base, member }
    }

    pub(crate) fn getter_methods(&self) -> TokenStream {
        let GetterItem { name, vis, docs } = match GetterItem::new(self) {
            Some(item) => item,
            None => return quote! {},
        };

        let index = &self.member.index;
        let ty = self.member.underlying_norm_ty();

        let (return_type, body) = if self.member.is_required() {
            (
                quote! { &#ty },
                quote! {
                    unsafe {
                        // SAFETY: this code is runs in a method that has a where
                        // bound that ensures the member was set.
                        ::std::option::Option::unwrap_unchecked(
                            self.__unsafe_private_named.#index.as_ref()
                        )
                    }
                },
            )
        } else {
            (
                // We are not using the fully qualified path to `Option` here
                // to make function signature in IDE popus shorter and more
                // readable.
                quote! { Option<&#ty> },
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
                clippy::missing_const_for_fn,
            )]
            #[inline(always)]
            #[must_use = "this method has no side effects; it only returns a value"]
            #vis fn #name(&self) -> #return_type
            where
                #state_var::#member_pascal: #state_mod::IsSet,
            {
                #body
            }
        }
    }
}

impl GetterItem {
    fn new(ctx: &GettersCtx<'_>) -> Option<Self> {
        let GettersCtx { member, base } = ctx;

        let config = member.config.getter.as_ref()?;

        Some(Self {
            name: config.name().cloned().unwrap_or_else(|| {
                syn::Ident::new(
                    &format!("get_{}", member.name.snake.raw_name()),
                    member.name.snake.span(),
                )
            }),
            vis: config.vis().unwrap_or(&base.builder_type.vis).clone(),
            docs: config.docs().map(<[_]>::to_vec).unwrap_or_else(|| {
                let header = format!(
                    "_**Getter.**_ Returns `{}`, which must be set before calling this method.\n\n",
                    member.name.snake,
                );

                std::iter::once(syn::parse_quote!(#[doc = #header]))
                    .chain(member.docs.iter().cloned())
                    .collect()
            }),
        })
    }
}
