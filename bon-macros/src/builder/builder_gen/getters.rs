use super::member::{GetterConfig, GetterKind};
use super::{BuilderGenCtx, NamedMember};
use crate::util::prelude::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

pub(crate) struct GettersCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
    config: &'a GetterConfig,
}

impl<'a> GettersCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Option<Self> {
        Some(Self {
            base,
            member,
            config: member.config.getter.as_ref()?,
        })
    }

    pub(crate) fn getter_methods(self) -> Result<TokenStream> {
        let name = self.config.name.as_deref().cloned().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("get_{}", self.member.name.snake.raw_name()),
                self.member.name.snake.span(),
            )
        });

        let vis = self
            .config
            .vis
            .as_deref()
            .unwrap_or(&self.base.builder_type.vis)
            .clone();

        let docs = self.config.docs.as_deref().cloned().unwrap_or_else(|| {
            let header = format!(
                "_**Getter.**_ Returns `{}`, which must be set before calling this method.\n\n",
                self.member.name.snake,
            );

            std::iter::once(syn::parse_quote!(#[doc = #header]))
                .chain(self.member.docs.iter().cloned())
                .collect()
        });

        let index = &self.member.index;
        let ty = self.member.underlying_norm_ty();

        let ret_ty = self.return_ty()?;
        let body = self.body();

        let state_var = &self.base.state_var;
        let member_pascal = &self.member.name.pascal;
        let state_mod = &self.base.state_mod.ident;

        Ok(quote! {
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                clippy::missing_const_for_fn,
            )]
            #[inline(always)]
            #[must_use = "this method has no side effects; it only returns a value"]
            #vis fn #name(&self) -> #ret_ty
            where
                #state_var::#member_pascal: #state_mod::IsSet,
            {
                #body
            }
        })
    }

    fn body(&self) -> TokenStream {
        let index = &self.member.index;
        let field = quote! {
            self.__unsafe_private_named.#index
        };

        if let Some(kind) = &self.config.kind {
            match &kind.value {
                GetterKind::Copy => {
                    if self.member.is_required() {
                        return quote! {
                            unsafe {
                                ::core::option::Option::unwrap_unchecked(#field)
                            }
                        };
                    }
                    return field;
                }
                GetterKind::Clone => {
                    if self.member.is_required() {
                        return quote! {
                            unsafe {
                                ::core::clone::Clone::clone(&::core::option::Option::unwrap_unchecked(
                                    ::core::option::Option::as_ref(&#field)
                                ))
                            }
                        };
                    }
                    return quote!(::core::clone::Clone::clone(&#field));
                }
                GetterKind::Deref(_) => {}
            }
        }

        if self.member.is_required() {
            return quote! {
                match &#field {
                    Some(value) => value,
                    None => unsafe {
                        ::core::hint::unreachable_unchecked()
                    },
                }
            };
        }

        quote! {
            match &#field {
                Some(value) => Some(value),
                None => None,
            }
        }
    }

    fn underlying_return_ty(&self) -> Result<TokenStream> {
        let ty = self.member.underlying_norm_ty();

        let kind = match &self.config.kind {
            Some(kind) => kind,
            None => return Ok(quote! { &#ty }),
        };

        match &kind.value {
            GetterKind::Copy | GetterKind::Clone => return Ok(quote! { #ty }),
            GetterKind::Deref(Some(deref_target)) => return Ok(quote! { &#deref_target }),
            // Go below to the code that infers the deref target type
            GetterKind::Deref(None) => {}
        }

        use quote_spanned as qs;

        let span = ty.span();

        let deref_target_inference_table: &[(_, &dyn Fn(&Punctuated<_, _>) -> _)] = &[
            ("Vec", &|args| args.first().map(|arg| qs!(span=> [#arg]))),
            ("Box", &|args| args.first().map(ToTokens::to_token_stream)),
            ("Rc", &|args| args.first().map(ToTokens::to_token_stream)),
            ("Arc", &|args| args.first().map(ToTokens::to_token_stream)),
            ("String", &|args| args.is_empty().then(|| qs!(span=> str))),
            ("CString", &|args| {
                args.is_empty().then(|| qs!(span=> ::core::ffi::CStr))
            }),
            ("OsString", &|args| {
                args.is_empty().then(|| qs!(span=> ::std::ffi::OsStr))
            }),
            ("PathBuf", &|args| {
                args.is_empty().then(|| qs!(span=> ::std::path::Path))
            }),
            ("Cow", &|args| {
                args.iter()
                    .find(|arg| matches!(arg, syn::GenericArgument::Type(_)))
                    .map(ToTokens::to_token_stream)
            }),
        ];

        let err = || {
            let inferable_types = deref_target_inference_table
                .iter()
                .map(|(name, _)| format!("- {name}"))
                .join("\n");

            err!(
                &kind.key,
                "can't infer the `Deref::Target` for the getter from the member's type; \
                please specify the return type (target of the deref coercion) explicitly \
                in parentheses without the leading `&`;\n\
                example: `#[builder(getter(deref(TargetTypeHere))]`\n\
                \n\
                automatic deref target detection is supported only for the following types:\n\
                {inferable_types}",
            )
        };

        let path = ty.as_path_no_qself().ok_or_else(err)?;

        let last_segment = path.segments.last().ok_or_else(err)?;

        let empty_punctuated = Punctuated::new();

        let args = match &last_segment.arguments {
            syn::PathArguments::AngleBracketed(args) => &args.args,
            _ => &empty_punctuated,
        };

        let last_segment_ident_str = last_segment.ident.to_string();

        let inferred = deref_target_inference_table
            .iter()
            .filter(|(name, _)| last_segment_ident_str == *name)
            .find_map(|(_, infer)| infer(args))
            .ok_or_else(err)?;

        Ok(quote!(&#inferred))
    }

    fn return_ty(&self) -> Result<TokenStream> {
        let underlying_return_ty = self.underlying_return_ty()?;

        Ok(if self.member.is_required() {
            quote! { #underlying_return_ty }
        } else {
            // We are not using the fully qualified path to `Option` here
            // to make function signature in IDE popus shorter and more
            // readable.
            quote! { Option<#underlying_return_ty> }
        })
    }
}
