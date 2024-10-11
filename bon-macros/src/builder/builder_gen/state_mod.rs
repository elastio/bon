use super::BuilderGenCtx;
use crate::util::prelude::*;

pub(super) struct StateModGenCtx<'a> {
    builder_gen: &'a BuilderGenCtx,
    stateful_members_snake: Vec<&'a syn::Ident>,
    stateful_members_pascal: Vec<&'a syn::Ident>,
    sealed_method_decl: TokenStream,
    sealed_method_impl: TokenStream,
}

impl<'a> StateModGenCtx<'a> {
    pub(super) fn new(builder_gen: &'a BuilderGenCtx) -> Self {
        Self {
            builder_gen,

            stateful_members_snake: builder_gen
                .stateful_members()
                .map(|member| &member.name.snake)
                .collect(),

            stateful_members_pascal: builder_gen
                .stateful_members()
                .map(|member| &member.name.pascal)
                .collect(),

            // A method without `self` makes the trait non-object safe,
            // which is convenient, because we want that in this case.
            sealed_method_decl: quote! {
                #[doc(hidden)]
                fn __sealed(_: sealed::Sealed);
            },

            sealed_method_impl: quote! {
                fn __sealed(_: sealed::Sealed) {}
            },
        }
    }

    pub(super) fn state_mod(&self) -> TokenStream {
        let vis_mod = &self.builder_gen.state_mod.vis;
        let vis_child = &self.builder_gen.state_mod.vis_child;
        let vis_child_child = &self.builder_gen.state_mod.vis_child_child;

        let state_mod_docs = &self.builder_gen.state_mod.docs;
        let state_mod_ident = &self.builder_gen.state_mod.ident;

        let state_trait = self.state_trait();
        let is_complete_trait = self.is_complete_trait();
        let members_names_mod = self.members_names_mod();
        let (state_transitions, parent_state_transition_items) = self.state_transitions();

        quote! {
            #parent_state_transition_items

            #[allow(
                // These are intentional. By default, the builder module is private
                // and can't be accessed outside of the module where the builder
                // type is defined. This makes the builder type "anonymous" to
                // the outside modules, which is a good thing if users don't want
                // to expose this API surface.
                //
                // Also, there are some genuinely private items like the `Sealed`
                // enum and members "name" enums that we don't want to expose even
                // to the module that defines the builder. These APIs are not
                // public, and users instead should only reference the traits
                // and state transition type aliases from here.
                unnameable_types, unreachable_pub, clippy::redundant_pub_crate
            )]
            #( #state_mod_docs )*
            #vis_mod mod #state_mod_ident {
                #[doc(inline)]
                #vis_child use ::bon::private::{IsSet, IsUnset};
                use ::bon::private::{Set, Unset};

                mod sealed {
                    #vis_child_child enum Sealed {}
                }

                #state_trait
                #is_complete_trait
                #state_transitions
                #members_names_mod
            }
        }
    }

    fn state_transitions(&self) -> (TokenStream, TokenStream) {
        let transition_tuples = self
            .builder_gen
            .stateful_members()
            .map(|member| {
                let states = self.builder_gen.stateful_members().map(|other_member| {
                    if other_member.is(member) {
                        let member_snake = &member.name.snake;
                        quote! {
                            Set<members::#member_snake>
                        }
                    } else {
                        let member_pascal = &other_member.name.pascal;
                        quote! {
                            <S as State>::#member_pascal
                        }
                    }
                });

                quote! {
                    ( #( #states, )* )
                }
            })
            .collect::<Vec<_>>();

        let (set_member_aliases_docs, set_member_aliases): (Vec<_>, Vec<_>) = self
            .builder_gen
            .stateful_members()
            .map(|member| {
                let alias = format_ident!("Set{}", member.name.pascal_str);
                let member_snake = &member.name.snake;

                let docs = format!(
                    "Returns a [`State`] that has [`IsSet`] implemented for `{member_snake}`\n\
                    \n\
                    [`State`]: self::State\n\
                    [`IsSet`]: ::bon::IsSet",
                );

                (docs, alias)
            })
            .unzip();

        let vis_child = &self.builder_gen.state_mod.vis_child;
        let vis_child_child = &self.builder_gen.state_mod.vis_child_child;
        let stateful_members_snake = &self.stateful_members_snake;

        let state_param = (self.stateful_members_snake.len() > 1).then(|| {
            quote! {
                <S: State = AllUnset>
            }
        });

        // This code is a bit overcomplicated for a reason. We could avoid
        // defining the `mod type_aliases` and a `use type_aliases::*` but
        // we do this to have prettier documentation generated by `rustdoc`.
        //
        // The problem is that `rustdoc` inlines the type aliases if they
        // are unreachable from outside of the crate. It means that even if
        // the type alias is declared as `pub` but it resides in a private
        // module, `rustdoc` will inline it. This is not what we want, because
        // the type aliases grow very large and make the documentation noisy.
        //
        // As a workaround we generate a bit different code for `rustdoc`
        // that uses indirection via an associated type of a trait. The
        // macro `__prettier_type_aliases_docs` does that if `doc` cfg
        // is enabled. That macro modifies the contents of the module
        // to add that indirection.
        //
        // It's implemented this way to optimize the compilation perf. for
        // the case when `doc` cfg is disabled. In this case, we pay only
        // for a single `#[cfg_attr(doc, ...)]` that expands to nothing.
        //
        // We could just generate the code that `__prettier_type_aliases_docs`
        // generates eagerly and put it under `#[cfg(doc)]` but that increases
        // the compile time, because the compiler needs to parse all that code
        // even if it's not used and it's a lot of code.
        let mod_items = quote! {
            #vis_child use type_aliases::*;

            #[cfg_attr(doc, ::bon::private::__prettier_type_aliases_docs)]
            mod type_aliases {
                use super::{members, State, Unset, Set};

                /// Initial state of the builder where all members are unset
                #vis_child_child type AllUnset = (
                    #( Unset<members::#stateful_members_snake>, )*
                );

                #(
                    #[doc = #set_member_aliases_docs]
                    #vis_child_child type #set_member_aliases #state_param = #transition_tuples;
                )*
            }
        };

        let state_mod = &self.builder_gen.state_mod.ident;
        let builder_vis = &self.builder_gen.builder_type.vis;

        // This is a workaround for `rustdoc`. Without this `use` statement,
        // it inlines the type aliases. Although for this workaround to work,
        // all items from the current module need to be reexported via a `*`
        // reexport, or the items need to be defined in the root lib.rs file.
        //
        // Therefore we use a `#[prettier_type_aliases_docs]` attribute to
        // hide the internals of the type aliases from the documentation
        // in all other cases.
        let parent_items = quote! {
            #[doc(hidden)]
            #[cfg(doc)]
            #[allow(unused_import_braces)]
            #builder_vis use #state_mod::{ AllUnset as _ #(, #set_member_aliases as _)* };
        };

        (mod_items, parent_items)
    }

    fn state_trait(&self) -> TokenStream {
        let assoc_types_docs = self.stateful_members_snake.iter().map(|member_snake| {
            format!(
                "Type state of the member `{member_snake}`.\n\
                \n\
                It can implement either [`IsSet`] or [`IsUnset`].\n\
                \n\
                [`IsSet`]: ::bon::IsSet\n\
                [`IsUnset`]: ::bon::IsUnset",
            )
        });

        let vis_child = &self.builder_gen.state_mod.vis_child;
        let sealed_method_decl = &self.sealed_method_decl;
        let sealed_method_impl = &self.sealed_method_impl;
        let stateful_members_snake = &self.stateful_members_snake;
        let stateful_members_pascal = &self.stateful_members_pascal;

        quote! {
            /// Builder's type state specifies if members are set or not (unset).
            ///
            /// You can use the associated types of this trait to control the state of individual members
            /// with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with
            /// the `Set*` type aliases available in this module.
            ///
            /// [`IsSet`]: ::bon::IsSet
            /// [`IsUnset`]: ::bon::IsUnset
            #vis_child trait State: ::core::marker::Sized {
                #(
                    #[doc = #assoc_types_docs]
                    type #stateful_members_pascal: ::bon::private::MemberState<
                        self::members::#stateful_members_snake
                    >;
                )*

                #sealed_method_decl
            }

            // Using `self::State` explicitly to avoid name conflicts with the
            // members named `state` which would create a generic param named `State`
            // that would shadow the trait `State` in the same scope.
            #[doc(hidden)]
            impl<#(
                #stateful_members_pascal: ::bon::private::MemberState<
                    self::members::#stateful_members_snake
                >,
            )*>
            self::State for ( #(#stateful_members_pascal,)* )
            {
                #( type #stateful_members_pascal = #stateful_members_pascal; )*

                #sealed_method_impl
            }
        }
    }

    fn is_complete_trait(&self) -> TokenStream {
        let required_members_pascal = self
            .builder_gen
            .named_members()
            .filter(|member| member.is_required())
            .map(|member| &member.name.pascal)
            .collect::<Vec<_>>();

        let maybe_assoc_type_bounds = cfg!(feature = "implied-bounds").then(|| {
            quote! {
                < #( #required_members_pascal: IsSet, )* >
            }
        });

        let vis_child = &self.builder_gen.state_mod.vis_child;
        let sealed_method_decl = &self.sealed_method_decl;
        let sealed_method_impl = &self.sealed_method_impl;

        let on_unimplemented =
            Self::on_unimplemented("can't finish building yet; not all required members are set");

        quote! {
            /// Marker trait that indicates that all required members are set.
            ///
            /// In this state, the builder
            #on_unimplemented
            #vis_child trait IsComplete: State #maybe_assoc_type_bounds {
                #sealed_method_decl
            }

            #[doc(hidden)]
            impl<State: self::State> IsComplete for State
            where
                #(
                    State::#required_members_pascal: IsSet,
                )*
            {
                #sealed_method_impl
            }
        }
    }

    fn members_names_mod(&self) -> TokenStream {
        let vis_child_child = &self.builder_gen.state_mod.vis_child_child;
        let stateful_members_snake = &self.stateful_members_snake;

        quote! {
            #[deprecated =
                "this is an implementation detail and should not be \
                used directly; use the Set* type aliases to control the \
                state of members instead"
            ]
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            mod members {
                #(
                    #vis_child_child enum #stateful_members_snake {}
                )*
            }
        }
    }

    fn on_unimplemented(message: &str) -> TokenStream {
        quote! {
            #[::bon::private::rustversion::attr(
                since(1.78.0),
                diagnostic::on_unimplemented(message = #message, label = #message)
            )]
        }
    }
}

pub(crate) fn prettier_type_aliases_docs(module: TokenStream) -> TokenStream {
    try_prettier_type_aliases_docs(module.clone())
        .unwrap_or_else(|err| [module, err.write_errors()].concat())
}

fn try_prettier_type_aliases_docs(module: TokenStream) -> Result<TokenStream> {
    let mut module: syn::ItemMod = syn::parse2(module)?;
    let (_, module_items) = module.content.as_mut().ok_or_else(|| {
        err!(
            &Span::call_site(),
            "expected an inline module with type aliases inside"
        )
    })?;

    let mut aliases = module_items
        .iter_mut()
        .filter(|item| !matches!(item, syn::Item::Use(_)))
        .map(require_type_alias)
        .collect::<Result<Vec<_>>>()?;

    if aliases.is_empty() {
        bail!(
            &Span::call_site(),
            "expected at least one type alias inside the module (e.g. AllUnset)"
        )
    };

    let all_unset = aliases.remove(0);
    let set_members = aliases;

    // This is the case where there is one or zero stateful members. In this
    // case type aliases don't have any generic parameters to avoid the error
    // that the generic parameter is unused in the type alias definition.
    // This case is small and rare enough that we may just avoid doing any
    // special handling for it.
    if set_members.len() <= 1 {
        for alias in set_members {
            assert!(alias.generics.params.is_empty());
        }

        return Ok(module.into_token_stream());
    }

    let vis = all_unset.vis.clone().into_equivalent_in_child_module()?;

    let set_members_idents = set_members.iter().map(|alias| &alias.ident);
    let set_members_idents2 = set_members_idents.clone();
    let set_members_bodies = set_members.iter().map(|alias| &alias.ty);

    let all_unset_body = &all_unset.ty;

    let trait_module: syn::Item = syn::parse_quote! {
        mod private {
            use super::*;

            #vis trait OpaqueConst {
                type AllUnset;
            }

            impl OpaqueConst for () {
                type AllUnset = #all_unset_body;
            }

            #vis trait Opaque {
                #( type #set_members_idents; )*
            }

            impl<S: State> Opaque for S {
                #( type #set_members_idents2 = #set_members_bodies; )*
            }
        }
    };

    all_unset.ty = syn::parse_quote! {
        <() as private::OpaqueConst>::AllUnset
    };

    for alias in set_members {
        let alias_ident = &alias.ident;
        alias.ty = syn::parse_quote! {
            <S as private::Opaque>::#alias_ident
        };
    }

    module_items.push(trait_module);

    Ok(module.into_token_stream())
}

fn require_type_alias(item: &mut syn::Item) -> Result<&mut syn::ItemType> {
    match item {
        syn::Item::Type(item_type) => Ok(item_type),
        _ => bail!(
            &item,
            "expected a type alias inside the module, but found a different item"
        ),
    }
}
