use super::member::SetterClosure;
use super::{BuilderGenCtx, NamedMember};
use crate::parsing::ItemParams;
use crate::util::prelude::*;
use std::iter;

pub(crate) struct SettersCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

impl<'a> SettersCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self { base, member }
    }

    pub(crate) fn setter_methods(&self) -> TokenStream {
        match SettersItems::new(self) {
            SettersItems::Required(item) => self.setter_for_required_member(item),
            SettersItems::Optional(setters) => self.setters_for_optional_member(setters),
        }
    }

    fn setter_for_required_member(&self, item: SetterItem) -> TokenStream {
        let input;
        let value;

        let member_type = self.member.ty.norm.as_ref();

        if let Some(closure) = &self.member.params.with {
            input = Self::underlying_input_from_closure(closure);
            value = self.member_value_from_closure(closure);
        } else if self.member.params.into.is_present() {
            input = quote!(value: impl Into<#member_type>);
            value = quote!(Into::into(value));
        } else {
            input = quote!(value: #member_type);
            value = quote!(value);
        };

        let body = SetterBody::SetMember {
            value: quote!(Some(#value)),
        };

        self.setter_method(Setter {
            item,
            imp: SetterImpl { input, body },
        })
    }

    fn setters_for_optional_member(&self, items: OptionalSettersItems) -> TokenStream {
        if let Some(closure) = &self.member.params.with {
            return self.setters_for_optional_member_with_closure(closure, items);
        }

        let underlying_ty = self.member.underlying_norm_ty();
        let underlying_ty = if self.member.params.into.is_present() {
            quote!(impl Into<#underlying_ty>)
        } else {
            quote!(#underlying_ty)
        };

        let some_fn = Setter {
            item: items.some_fn,
            imp: SetterImpl {
                input: quote!(value: #underlying_ty),
                body: SetterBody::Forward {
                    body: {
                        let option_fn_name = &items.option_fn.name;
                        quote! {
                            self.#option_fn_name(Some(value))
                        }
                    },
                },
            },
        };

        let option_fn = Setter {
            item: items.option_fn,
            imp: SetterImpl {
                input: quote!(value: Option<#underlying_ty>),
                body: SetterBody::SetMember {
                    value: if self.member.params.into.is_present() {
                        quote! {
                            Option::map(value, Into::into)
                        }
                    } else {
                        quote!(value)
                    },
                },
            },
        };

        [self.setter_method(some_fn), self.setter_method(option_fn)].concat()
    }

    fn setters_for_optional_member_with_closure(
        &self,
        closure: &SetterClosure,
        items: OptionalSettersItems,
    ) -> TokenStream {
        let idents = closure.inputs.iter().map(|input| &input.pat.ident);

        // If the closure accepts just a single input avoid wrapping it
        // in a tuple in the `option_fn` setter.
        let tuple_if_many = |val: TokenStream| -> TokenStream {
            if closure.inputs.len() == 1 {
                val
            } else {
                quote!((#val))
            }
        };

        let idents = tuple_if_many(quote!( #( #idents ),* ));

        let some_fn = Setter {
            item: items.some_fn,
            imp: SetterImpl {
                input: Self::underlying_input_from_closure(closure),
                body: SetterBody::Forward {
                    body: {
                        let option_fn_name = &items.option_fn.name;
                        quote! {
                            self.#option_fn_name(Some(#idents))
                        }
                    },
                },
            },
        };

        let option_fn_impl = SetterImpl {
            input: {
                let input_types = closure.inputs.iter().map(|input| &input.ty);
                let input_types = tuple_if_many(quote!(#( #input_types, )*));
                quote!(value: Option<#input_types>)
            },
            body: SetterBody::SetMember {
                value: {
                    let value = self.member_value_from_closure(closure);
                    quote! {
                        match value {
                            Some(#idents) => Some(#value),
                            None => None,
                        }
                    }
                },
            },
        };

        let option_fn = Setter {
            item: items.option_fn,
            imp: option_fn_impl,
        };

        [self.setter_method(some_fn), self.setter_method(option_fn)].concat()
    }

    /// This method is reused between the setter for the required member and
    /// the `some_fn` setter for the optional member.
    ///
    /// We intentionally keep the name and signature of the setter method
    /// for an optional member that accepts the value under the option the
    /// same as the setter method for the required member to keep the API
    /// of the builder compatible when a required member becomes optional.
    /// To be able to explicitly pass an `Option` value to the setter method
    /// users need to use the `maybe_{member_ident}` method.
    fn underlying_input_from_closure(closure: &SetterClosure) -> TokenStream {
        let pats = closure.inputs.iter().map(|input| &input.pat);
        let types = closure.inputs.iter().map(|input| &input.ty);
        quote! {
            #( #pats: #types ),*
        }
    }

    fn member_value_from_closure(&self, closure: &SetterClosure) -> TokenStream {
        let body = &closure.body;

        let ty = self.member.underlying_norm_ty().to_token_stream();

        let output = Self::maybe_wrap_in_result(closure, ty.to_token_stream());

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

    fn maybe_wrap_in_result(closure: &SetterClosure, ty: TokenStream) -> TokenStream {
        let output = match closure.output.as_ref() {
            Some(output) => output,
            None => return ty,
        };
        let result_path = &output.result_path;
        let err_ty = output.err_ty.iter();
        quote! {
            #result_path< #ty #(, #err_ty )* >
        }
    }

    fn setter_method(&self, setter: Setter) -> TokenStream {
        let Setter { item, imp } = setter;

        let maybe_mut = match imp.body {
            SetterBody::Forward { .. } => None,
            SetterBody::SetMember { .. } => Some(syn::Token![mut](Span::call_site())),
        };

        let body = match imp.body {
            SetterBody::Forward { body } => body,
            SetterBody::SetMember { value } => {
                let index = &self.member.index;

                let fields = &self.base.ident_pool;
                let phantom_field = &fields.phantom;
                let receiver_field = &fields.receiver;
                let start_fn_args_field = &fields.start_fn_args;
                let named_members_field = &fields.named_members;

                let mut output = if self.member.is_stateful() {
                    let builder_ident = &self.base.builder_type.ident;

                    let maybe_receiver_field = self
                        .base
                        .receiver()
                        .map(|_| quote!(#receiver_field: self.#receiver_field,));

                    let maybe_start_fn_args_field = self
                        .base
                        .start_fn_args()
                        .next()
                        .map(|_| quote!(#start_fn_args_field: self.#start_fn_args_field,));

                    quote! {
                        #builder_ident {
                            #phantom_field: ::core::marker::PhantomData,
                            #maybe_receiver_field
                            #maybe_start_fn_args_field
                            #named_members_field: self.#named_members_field,
                        }
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
                    self.#named_members_field.#index = #value;
                    #output
                }
            }
        };

        let state_mod = &self.base.state_mod.ident;

        let mut return_type = if !self.member.is_stateful() {
            quote! { Self }
        } else {
            let state_transition = format_ident!("Set{}", self.member.name.pascal_str);
            let builder_ident = &self.base.builder_type.ident;
            let generic_args = &self.base.generics.args;
            let state_var = &self.base.state_var;

            quote! {
                #builder_ident<#(#generic_args,)* #state_mod::#state_transition<#state_var>>
            }
        };

        if let Some(closure) = &self.member.params.with {
            return_type = Self::maybe_wrap_in_result(closure, return_type);
        }

        let state_var = &self.base.state_var;

        let where_clause = (!self.member.params.overwritable.is_present()).then(|| {
            let member_pascal = &self.member.name.pascal;
            quote! {
                where
                    #state_var::#member_pascal: #state_mod::IsUnset,
            }
        });

        let SetterItem { name, vis, docs } = item;
        let input = imp.input;

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
            #vis fn #name(#maybe_mut self, #input) -> #return_type
            #where_clause
            {
                #body
            }
        }
    }
}

struct Setter {
    item: SetterItem,
    imp: SetterImpl,
}

struct SetterImpl {
    input: TokenStream,
    body: SetterBody,
}

enum SetterBody {
    /// The setter forwards the call to another method.
    Forward { body: TokenStream },

    /// The setter sets the member as usual and transitions the builder state.
    SetMember { value: TokenStream },
}

enum SettersItems {
    Required(SetterItem),
    Optional(OptionalSettersItems),
}

struct OptionalSettersItems {
    some_fn: SetterItem,
    option_fn: SetterItem,
}

struct SetterItem {
    name: syn::Ident,
    vis: syn::Visibility,
    docs: Vec<syn::Attribute>,
}

impl SettersItems {
    fn new(ctx: &SettersCtx<'_>) -> Self {
        let SettersCtx {
            member,
            base: builder_gen,
        } = ctx;
        let builder_type = &builder_gen.builder_type;

        let params = member.params.setters.as_ref();

        let common_name = params.and_then(|params| params.name.as_deref());
        let common_vis = params.and_then(|params| params.vis.as_deref());
        let common_docs = params.and_then(|params| params.docs.as_deref().map(Vec::as_slice));

        let doc = |docs: &str| iter::once(syn::parse_quote!(#[doc = #docs]));

        if member.is_required() {
            let docs = common_docs.unwrap_or(&member.docs);

            let header = "\
                | **Required** |\n\
                | -- |\n\n";

            let docs = doc(header).chain(docs.iter().cloned()).collect();

            return Self::Required(SetterItem {
                name: common_name.unwrap_or(&member.name.snake).clone(),
                vis: common_vis.unwrap_or(&builder_type.vis).clone(),
                docs,
            });
        }

        let some_fn = params.and_then(|params| params.fns.some_fn.as_deref());
        let some_fn_name = some_fn
            .and_then(ItemParams::name)
            .or(common_name)
            .unwrap_or(&member.name.snake)
            .clone();

        let option_fn = params.and_then(|params| params.fns.option_fn.as_deref());
        let option_fn_name = option_fn
            .and_then(ItemParams::name)
            .cloned()
            .unwrap_or_else(|| {
                let base_name = common_name.unwrap_or(&member.name.snake);
                // Preserve the original identifier span to make IDE's
                // "go to definition" works correctly
                format_ident!("maybe_{}", base_name)
            });

        let default = member.params.default.as_deref().and_then(|default| {
            let default = default
                .clone()
                .or_else(|| well_known_default(&member.ty.norm))
                .unwrap_or_else(|| syn::parse_quote!(Default::default()));

            let file = syn::parse_quote!(const _: () = #default;);
            let file = prettyplease::unparse(&file);

            let begin = file.find('=')?;
            let default = file.get(begin + 1..)?.trim();
            let default = default.strip_suffix(';')?;

            Some(default.to_owned())
        });

        let default = default.as_deref();

        // FIXME: the docs shouldn't reference the companion setter if that
        // setter has a lower visibility.
        let some_fn_docs = some_fn
            .and_then(ItemParams::docs)
            .or(common_docs)
            .unwrap_or(&member.docs);

        let some_fn_docs = {
            let header = optional_setter_docs(default, &option_fn_name, "accepts an `Option`");

            doc(&header).chain(some_fn_docs.iter().cloned()).collect()
        };

        let option_fn_docs = option_fn
            .and_then(ItemParams::docs)
            .or(common_docs)
            .unwrap_or(&member.docs);

        let option_fn_docs = {
            let header = optional_setter_docs(
                default,
                &some_fn_name,
                "wraps the value with `Some` internally",
            );

            doc(&header).chain(option_fn_docs.iter().cloned()).collect()
        };

        let some_fn = SetterItem {
            name: some_fn_name,
            vis: some_fn
                .and_then(ItemParams::vis)
                .or(common_vis)
                .unwrap_or(&builder_type.vis)
                .clone(),

            docs: some_fn_docs,
        };

        let option_fn = params.and_then(|params| params.fns.option_fn.as_deref());
        let option_fn = SetterItem {
            name: option_fn_name,

            vis: option_fn
                .and_then(ItemParams::vis)
                .or(common_vis)
                .unwrap_or(&builder_type.vis)
                .clone(),

            docs: option_fn_docs,
        };

        Self::Optional(OptionalSettersItems { some_fn, option_fn })
    }
}

fn optional_setter_docs(
    default: Option<&str>,
    other_setter: &syn::Ident,
    description: &str,
) -> String {
    let default = default
        .map(|default| {
            if default.contains('\n') || default.len() > 80 {
                format!("**Default:**\n````rust,ignore\n{default}\n````\n\n")
            } else {
                format!("**Default:** ```{default}```.\n\n")
            }
        })
        .unwrap_or_default();

    format!(
        "| **Optional** |\n\
         | -- |\n\n\
         **See also** [`{other_setter}()`](Self::{other_setter}), which is a companion setter that {description}.
        \n\n{default}",
    )
}

fn well_known_default(ty: &syn::Type) -> Option<syn::Expr> {
    let path = match ty {
        syn::Type::Path(syn::TypePath { path, qself: None }) => path,
        _ => return None,
    };

    use syn::parse_quote as pq;

    let ident = path.get_ident()?.to_string();

    let value = match ident.as_str() {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "isize" => pq!(0),
        "f32" | "f64" => pq!(0.0),
        "bool" => pq!(false),
        "char" => pq!('\0'),
        "String" => pq!(""),
        _ => return None,
    };

    Some(value)
}
