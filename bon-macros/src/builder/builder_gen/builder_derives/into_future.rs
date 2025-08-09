use crate::builder::builder_gen::models::BuilderGenCtx;
use crate::builder::builder_gen::top_level_config::IntoFutureConfig;
use crate::util::prelude::*;

impl BuilderGenCtx {
    pub(super) fn derive_into_future(&self, config: &IntoFutureConfig) -> Result<TokenStream> {
        if self.finish_fn.asyncness.is_none() {
            // While it is technically possible to call a synchronous function
            // inside of the `IntoFuture::into_future()`, it's better force the
            // user to mark the function as `async` explicitly. Otherwise it may
            // indicate of some logic bug where the developer mistakenly marks
            // a function that could be sync with `derive(IntoFuture)`.
            bail!(
                &self.finish_fn.ident,
                "`#[builder(derive(IntoFuture(...)))` can only be used with async functions; \
                using it with a synchronous function is likely a mistake"
            );
        }

        if let Some(unsafety) = &self.finish_fn.unsafety {
            bail!(
                unsafety,
                "`#[builder(derive(IntoFuture(...)))` is not supported for unsafe functions \
                because `IntoFuture::into_future()` method is a safe method"
            );
        }

        if let Some(arg) = self.finish_fn_args().next() {
            bail!(
                &arg.config.finish_fn.span(),
                "`#[builder(derive(IntoFuture(...)))` is incompatible with `#[builder(finish_fn)]` members \
                because `IntoFuture::into_future()` method accepts zero parameters"
            );
        }

        let output_ty = match &self.finish_fn.output {
            syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
                paren_token: syn::token::Paren::default(),
                elems: syn::punctuated::Punctuated::new(),
            })),
            syn::ReturnType::Type(_, output_ty) => output_ty.clone(),
        };

        let state_mod = &self.state_mod.ident;
        let generics_decl = &self.generics.decl_without_defaults;
        let generic_args = &self.generics.args;
        let builder_ident = &self.builder_type.ident;
        let state_var = &self.state_var;
        let finish_fn_ident = &self.finish_fn.ident;

        let builder_ty = quote! {
            #builder_ident<#(#generic_args,)* #state_var>
        };

        let send_bound = if config.is_send {
            quote! { + ::core::marker::Send }
        } else {
            quote! {}
        };

        let tokens = quote! {
            #[automatically_derived]
            impl<
                #(#generics_decl,)*
                #state_var: #state_mod::IsComplete + 'static
            >
            ::core::future::IntoFuture for #builder_ty
            where
                #builder_ty: 'static,
            {
                type Output = #output_ty;
                type IntoFuture = ::std::pin::Pin<::std::boxed::Box<dyn ::core::future::Future<Output = #output_ty> #send_bound>>;

                fn into_future(self) -> Self::IntoFuture {
                    ::std::boxed::Box::pin(#builder_ident::#finish_fn_ident(self))
                }
            }
        };

        Ok(tokens)
    }
}
