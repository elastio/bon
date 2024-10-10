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
        let vis_child_child = &self.builder_gen.state_mod.vis_child_child;

        let state_mod_docs = &self.builder_gen.state_mod.docs;
        let state_mod_ident = &self.builder_gen.state_mod.ident;

        let state_trait = self.state_trait();
        let is_set_trait = self.is_set_trait();
        let is_unset_trait = self.is_unset_trait();
        let is_complete_trait = self.is_complete_trait();
        let members_names_mod = self.members_names_mod();
        let (state_transitions, parent_state_transition_items) = self.state_transitions();

        quote! {
            #parent_state_transition_items

            // This is intentional. By default, the builder module is private
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
            #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
            #( #state_mod_docs )*
            #vis_mod mod #state_mod_ident {
                mod sealed {
                    #vis_child_child enum Sealed {}
                }

                #state_trait
                #is_set_trait
                #is_unset_trait
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
                            ::bon::private::Set<members::#member_snake>
                        }
                    } else {
                        let member_pascal = &other_member.name.pascal;
                        quote! {
                            <Self as State>::#member_pascal
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
        let stateful_members_pascal = &self.stateful_members_pascal;
        let sealed_method_impl = &self.sealed_method_impl;

        let mod_items = quote! {
            /// Initial state of the builder where all members are unset
            #vis_child struct AllUnset {
                _private: ()
            }

            impl State for AllUnset {
                #(
                    type #stateful_members_pascal = ::bon::private::Unset<members::#stateful_members_snake>;
                )*

                #sealed_method_impl
            }

            #(
                #[doc = #set_member_aliases_docs]
                #vis_child type #set_member_aliases<S: State = AllUnset> =
                    <S as private::StateExt>::#set_member_aliases;
            )*

            mod private {
                #[doc(hidden)]
                #vis_child_child trait StateExt {
                    #( type #set_member_aliases; )*
                }
            }

            impl<S: State> private::StateExt for S {
                #(type #set_member_aliases = #transition_tuples; )*
            }
        };

        let state_mod = &self.builder_gen.state_mod.ident;
        let builder_vis = &self.builder_gen.builder_type.vis;

        // This is a workaround for `rustdoc`. Without this `use` statement,
        // it inlines the type aliases. Although for this workaround to work,
        // all items from the current module need to be reexported via a `*`
        // reexport, or the items need to be defined in the root lib.rs file.
        let parent_items = quote! {
            #[doc(hidden)]
            #[cfg(doc)]
            #[allow(unused_import_braces)]
            #builder_vis use #state_mod::{ #( #set_member_aliases as _ ,)* };
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

        let maybe_assoc_type_bounds = cfg!(feature = "msrv-1-79-0").then(|| {
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

    fn is_set_trait(&self) -> TokenStream {
        let vis_child = &self.builder_gen.state_mod.vis_child;
        let sealed_method_decl = &self.sealed_method_decl;
        let sealed_method_impl = &self.sealed_method_impl;

        let on_unimplemented = Self::on_unimplemented(
            "the member `{Self}` was not set, but this method requires it to be set",
        );

        quote! {
            /// Marker trait that indicates that the member is set, i.e. at least
            /// one of its setters was called.
            // TODO: add examples (they would require having custom renames and
            // visibility overrides for default setters)
            #on_unimplemented
            #vis_child trait IsSet {
                #sealed_method_decl
            }

            #[doc(hidden)]
            impl<Name> IsSet for ::bon::private::Set<Name> {
                #sealed_method_impl
            }
        }
    }

    fn is_unset_trait(&self) -> TokenStream {
        let vis_child = &self.builder_gen.state_mod.vis_child;
        let sealed_method_decl = &self.sealed_method_decl;
        let sealed_method_impl = &self.sealed_method_impl;

        let on_unimplemented = Self::on_unimplemented(
            "the member `{Self}` was already set, but this method requires it to be unset",
        );

        quote! {
            /// Marker trait implemented by members that are not set.
            #on_unimplemented
            #vis_child trait IsUnset {
                #sealed_method_decl
            }

            #[doc(hidden)]
            impl<Name> IsUnset for ::bon::private::Unset<Name> {
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
