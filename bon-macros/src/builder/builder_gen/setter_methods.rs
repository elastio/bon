use super::member::SetterClosure;
use super::{BuilderGenCtx, NamedMember};
use crate::util::prelude::*;

pub(crate) struct SettersCtx<'a> {
    builder_gen: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

impl<'a> SettersCtx<'a> {
    pub(crate) fn new(builder_gen: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self {
            builder_gen,
            member,
        }
    }

    pub(crate) fn setter_methods(&self) -> TokenStream {
        if self.member.is_required() {
            self.setters_for_required_member()
        } else {
            self.setters_for_optional_member()
        }
    }

    fn setters_for_required_member(&self) -> TokenStream {
        let fn_inputs;
        let member_init;

        if let Some(closure) = &self.member.params.with {
            fn_inputs = Self::inputs_from_closure(closure);

            let member_init_override = self.member_init_from_closure(closure);

            member_init = quote!(Some(#member_init_override));
        } else {
            let member_type = self.member.norm_ty.as_ref();
            let has_into = self.member.params.into.is_present();

            if has_into {
                fn_inputs = quote!(value: impl Into<#member_type>);
                member_init = quote!(Some(::core::convert::Into::into(value)));
            } else {
                fn_inputs = quote!(value: #member_type);
                member_init = quote!(Some(value));
            }
        }

        self.setter_method(Setter {
            method_name: self.member.public_ident().clone(),
            fn_inputs,
            overwrite_docs: None,
            body: SetterBody::Default { member_init },
        })
    }

    fn setters_for_optional_member(&self) -> TokenStream {
        let member_name = self.member.public_ident().clone();

        // Preserve the original identifier span to make IDE's "go to definition" work correctly
        let option_fn_name = syn::Ident::new(
            &format!("maybe_{}", member_name.raw_name()),
            member_name.span(),
        );

        let some_fn_inputs;
        // Option-less setter is just a shortcut for wrapping the value in `Some`.
        let some_fn_body;

        let option_fn_inputs;
        let option_fn_member_init;

        if let Some(closure) = &self.member.params.with {
            some_fn_inputs = Self::inputs_from_closure(closure);

            // If the closure accepts just a single input avoid wrapping it
            // in a tuple in the `option_fn` setter.
            let maybe_wrap_in_tuple = |val: TokenStream| -> TokenStream {
                if closure.inputs.len() == 1 {
                    val
                } else {
                    quote!((#val))
                }
            };

            let idents = closure.inputs.iter().map(|input| &input.pat.ident);
            let idents = maybe_wrap_in_tuple(quote!( #( #idents ),* ));

            some_fn_body = {
                quote! {
                    self.#option_fn_name(Some(#idents))
                }
            };

            option_fn_inputs = {
                let inputs = closure.inputs.iter().map(|input| &input.ty);
                let inputs = maybe_wrap_in_tuple(quote!(#( #inputs, )*));
                quote!(value: Option<#inputs>)
            };

            option_fn_member_init = {
                let init = self.member_init_from_closure(closure);
                quote! {
                    match value {
                        Some(#idents) => Some(#init),
                        None => None,
                    }
                }
            }
        } else {
            let underlying_ty = self.member.underlying_norm_ty();
            let has_into = self.member.params.into.is_present();

            let inner_type = if has_into {
                quote!(impl Into<#underlying_ty>)
            } else {
                quote!(#underlying_ty)
            };

            some_fn_inputs = quote!(value: #inner_type);
            some_fn_body = quote! {
                self.#option_fn_name(Some(value))
            };
            option_fn_inputs = quote!(value: Option<#inner_type>);

            option_fn_member_init = if has_into {
                quote!(::core::option::Option::map(
                    value,
                    ::core::convert::Into::into
                ))
            } else {
                quote!(value)
            };
        }

        let methods = [
            Setter {
                method_name: option_fn_name,
                fn_inputs: option_fn_inputs,
                overwrite_docs: Some(format!(
                    "Same as [`Self::{member_name}`], but accepts \
                    an `Option` as input. See that method's documentation for \
                    more details.",
                )),
                body: SetterBody::Default {
                    member_init: option_fn_member_init,
                },
            },
            // We intentionally keep the name and signature of the setter method
            // for an optional member that accepts the value under the option the
            // same as the setter method for the required member to keep the API
            // of the builder compatible when a required member becomes optional.
            // To be able to explicitly pass an `Option` value to the setter method
            // users need to use the `maybe_{member_ident}` method.
            Setter {
                method_name: member_name,
                fn_inputs: some_fn_inputs,
                overwrite_docs: None,
                body: SetterBody::Custom(some_fn_body),
            },
        ];

        methods
            .into_iter()
            .map(|method| self.setter_method(method))
            .collect()
    }

    fn inputs_from_closure(closure: &SetterClosure) -> TokenStream {
        let pats = closure.inputs.iter().map(|input| &input.pat);
        let types = closure.inputs.iter().map(|input| &input.ty);
        quote! {
            #( #pats: #types ),*
        }
    }

    fn member_init_from_closure(&self, closure: &SetterClosure) -> TokenStream {
        let body = &closure.body;

        let ty = self.member.underlying_norm_ty().to_token_stream();

        let output = Self::result_output_from_closure(closure, || &ty)
            .unwrap_or_else(|| ty.to_token_stream());

        // Avoid wrapping the body in a block if it's already a block.
        let body = if matches!(body.as_ref(), syn::Expr::Block(_)) {
            body.to_token_stream()
        } else {
            quote!({ #body })
        };

        let question_mark = closure
            .output
            .is_some()
            .then(|| syn::Token![?](Span::call_site()));

        quote! {
            (move || -> #output #body)() #question_mark
        }
    }

    fn result_output_from_closure<T: ToTokens>(
        closure: &SetterClosure,
        default_ty: impl FnOnce() -> T,
    ) -> Option<TokenStream> {
        let output = closure.output.as_ref()?;
        let result_path = &output.result_path;
        let err_ty = output.err_ty.iter();
        let default_ty = default_ty();
        Some(quote! {
            #result_path< #default_ty #(, #err_ty )* >
        })
    }

    fn setter_method(&self, method: Setter) -> TokenStream {
        let Setter {
            method_name,
            fn_inputs,
            overwrite_docs,
            body,
        } = method;

        let docs = match overwrite_docs {
            Some(docs) => vec![syn::parse_quote!(#[doc = #docs])],
            None if !self.member.docs.is_empty() => self.member.docs.clone(),
            None => self.generate_docs_for_setter(),
        };

        let body = match body {
            SetterBody::Custom(body) => body,
            SetterBody::Default { member_init } => {
                let index = &self.member.index;

                let mut output = if self.member.is_stateful() {
                    quote! {
                        Self::__private_transition_type_state(self)
                    }
                } else {
                    quote! {
                        self
                    }
                };

                let result_output = self
                    .member
                    .params
                    .with
                    .as_ref()
                    .and_then(|closure| closure.output.as_ref());

                if let Some(result_output) = result_output {
                    let result_path = &result_output.result_path;
                    output = quote!(#result_path::Ok(#output));
                }

                quote! {
                    self.__private_named_members.#index = #member_init;
                    #output
                }
            }
        };

        let member_pascal = &self.member.norm_ident_pascal;

        let state_transition = format_ident!("Set{}", self.member.norm_ident_pascal.raw_name());

        let state_mod = &self.builder_gen.state_mod.ident;
        let generic_param = if self.builder_gen.stateful_members().take(2).count() == 1 {
            quote!()
        } else {
            quote!(<BuilderState>)
        };

        let state_transition = quote! {
            #state_mod::#state_transition #generic_param
        };

        let builder_ident = &self.builder_gen.builder_type.ident;
        let generic_args = &self.builder_gen.generics.args;

        let mut return_type = if self.member.is_stateful() {
            quote! {
                #builder_ident<#(#generic_args,)* #state_transition>
            }
        } else {
            quote! { Self }
        };

        if let Some(closure) = &self.member.params.with {
            if let Some(overridden) = Self::result_output_from_closure(closure, || &return_type) {
                return_type = overridden;
            }
        }

        let where_clause =
            if self.member.is_stateful() && !self.member.params.overwritable.is_present() {
                quote! {
                    where
                        BuilderState::#member_pascal: #state_mod::IsUnset,
                }
            } else {
                quote! {}
            };

        let vis = &self.builder_gen.builder_type.vis;

        quote! {
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We don't want to avoid using `impl Trait` in the setter. This way
                // the setter signature is easier to read, and anyway if you want to
                // specify a type hint for the method that accepts an `impl Into`, then
                // your design of this setter already went wrong.
                clippy::impl_trait_in_params
            )]
            #[inline(always)]
            #vis fn #method_name(mut self, #fn_inputs) -> #return_type
            #where_clause
            {
                #body
            }
        }
    }

    fn generate_docs_for_setter(&self) -> Vec<syn::Attribute> {
        let setter_core_name = self.member.public_ident();
        let start_fn_ident = &self.builder_gen.start_fn.ident;

        let more = |start_fn_path: &std::fmt::Arguments<'_>| {
            format!(" See {start_fn_path} for more info.")
        };

        let suffix = self
            .builder_gen
            .assoc_method_ctx
            .as_ref()
            .map(|assoc_ctx| {
                let ty_path = match assoc_ctx.self_ty.as_path() {
                    Some(ty_path) => ty_path,

                    // The type is quite complex. It's hard to generate a workable
                    // intra-doc link for it. So in order to avoid the broken
                    // intra-doc links lint we'll just skip adding more info.
                    _ => return String::new(),
                };

                let prefix = darling::util::path_to_string(&ty_path.path);
                more(&format_args!("[`{prefix}::{start_fn_ident}()`]"))
            })
            .unwrap_or_else(|| more(&format_args!("[`{start_fn_ident}()`]")));

        let docs = format!("Sets the value of `{setter_core_name}`.{suffix}");

        vec![syn::parse_quote!(#[doc = #docs])]
    }
}

enum SetterBody {
    Custom(TokenStream),
    Default { member_init: TokenStream },
}

struct Setter {
    method_name: syn::Ident,
    fn_inputs: TokenStream,
    overwrite_docs: Option<String>,
    body: SetterBody,
}
