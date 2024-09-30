use crate::builder::builder_gen::member::NamedMember;
use crate::util::prelude::*;
use quote::quote;

impl super::BuilderGenCtx {
    fn state_transition_aliases(&self) -> Vec<syn::ItemType> {
        let vis_child = &self.state_mod.vis_child;

        let stateful_members = self.stateful_members().collect::<Vec<_>>();
        let is_single_member = stateful_members.len() == 1;

        stateful_members
            .iter()
            .map(|member| {
                let states = stateful_members.iter().map(|other_member| {
                    if other_member.orig_ident == member.orig_ident {
                        let ident = &member.public_ident();
                        quote! {
                            ::bon::private::Set<members::#ident>
                        }
                    } else {
                        let member_pascal = &other_member.norm_ident_pascal;
                        quote! {
                            S::#member_pascal
                        }
                    }
                });

                let member_ident = member.public_ident();
                let alias_ident =
                    quote::format_ident!("Set{}", member.norm_ident_pascal.raw_name());

                let docs = format!(
                    "Returns a [`State`] that has [`IsSet`] implemented for `{member_ident}`\n\
                    \n\
                    [`State`]: self::State\n\
                    [`IsSet`]: ::bon::IsSet",
                );

                if is_single_member {
                    return syn::parse_quote! {
                        #[doc = #docs]
                        #vis_child type #alias_ident = ( #(#states,)* );
                    };
                }

                syn::parse_quote! {
                    #[doc = #docs]
                    #vis_child type #alias_ident<
                        S: self::State = self::AllUnset
                    > = (
                        #(#states,)*
                    );
                }
            })
            .collect()
    }

    #[allow(clippy::cognitive_complexity)]
    pub(super) fn state_mod(&self) -> TokenStream2 {
        let builder_vis = &self.builder_type.vis;
        let vis_mod = &self.state_mod.vis;
        let vis_child = &self.state_mod.vis_child;
        let vis_child_child = &self.state_mod.vis_child_child;

        let state_mod_docs = &self.state_mod.docs;
        let state_mod_ident = &self.state_mod.ident;
        let state_transition_aliases = self.state_transition_aliases();

        let stateful_members_idents = self
            .stateful_members()
            .map(NamedMember::public_ident)
            .collect::<Vec<_>>();

        let assoc_types_docs = self.stateful_members().map(|member| {
            let ident = &member.public_ident();
            format!(
                "Type state of the member `{ident}`.\n\
                \n\
                It can implement either [`IsSet`] or [`IsUnset`].\n\
                \n\
                [`IsSet`]: ::bon::IsSet\n\
                [`IsUnset`]: ::bon::IsUnset",
            )
        });

        let stateful_members_pascal = self
            .stateful_members()
            .map(|member| &member.norm_ident_pascal)
            .collect::<Vec<_>>();

        let required_members_pascal = self
            .named_members()
            .filter(|member| member.is_required())
            .map(|member| &member.norm_ident_pascal)
            .collect::<Vec<_>>();

        let type_aliases_for_rustdoc = [&quote::format_ident!("AllUnset")];
        let type_aliases_for_rustdoc = state_transition_aliases
            .iter()
            .map(|alias| &alias.ident)
            .chain(type_aliases_for_rustdoc);

        quote! {
            // This is a workaround for `rustdoc`. Without these `use` statements,
            // it inlines the type aliases
            #(
                #[cfg(doc)]
                #[doc(hidden)]
                #builder_vis use self::#state_mod_ident::#type_aliases_for_rustdoc as _;
            )*

            #( #state_mod_docs )*
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
            #[allow(unnameable_types, unreachable_pub)]
            #[doc(hidden)]
            #vis_mod mod #state_mod_ident {
                /// Marker trait implemented by members that are set.
                #[::bon::private::rustversion::attr(
                    since(1.78.0),
                    diagnostic::on_unimplemented(
                        message = "the member `{Self}` was not set, but this method requires it to be set",
                        label = "the member `{Self}` was not set, but this method requires it to be set",
                    )
                )]
                #vis_child trait IsSet {
                    // Also a method without `self` makes the trait non-object safe
                    #[doc(hidden)]
                    fn __sealed(_: sealed::Sealed);
                }

                #[doc(hidden)]
                impl<Name> IsSet for ::bon::private::Set<Name> {
                    fn __sealed(_: sealed::Sealed) {}
                }


                // use private::sealed::Sealed;
                //
                // /// Marker trait that indicates that the member is not set, i.e. none of its setters were called.
                // ///
                // /// You should use this trait bound, for example, if you want to extend the builder with custom
                // /// setters.
                // ///
                // /// **Example:**
                // ///
                // /// ```
                // /// #[derive(bon::Builder)]
                // /// struct Example {
                // ///     x: i32,
                // ///     y: i32,
                // /// }
                // ///
                // /// // Import the type aliases for transforming the builder's type state
                // /// use example_builder::{SetX, SetY};
                // ///
                // /// // Add method to the builder
                // /// impl<State: example_builder::State> ExampleBuilder<State> {
                // ///     fn x_doubled(self, value: i32) -> ExampleBuilder<SetX<State>>
                // ///     where
                // ///         // The code won't compile without this bound
                // ///         State::X: bon::IsUnset,
                // ///     {
                // ///         self.x(value * 2)
                // ///     }
                // ///
                // ///     fn y_doubled(self, value: i32) -> ExampleBuilder<SetY<State>>
                // ///     where
                // ///         // The code won't compile without this bound
                // ///         State::Y: bon::IsUnset,
                // ///     {
                // ///        self.y(value * 2)
                // ///     }
                // /// }
                // ///
                // /// let example = Example::builder()
                // ///     .x_doubled(2)
                // ///     .y_doubled(3)
                // ///     .build();
                // ///
                // /// assert_eq!(example.x, 4);
                // /// assert_eq!(example.y, 6);
                // /// ```
                // #[rustversion::attr(
                //     since(1.78.0),
                //     diagnostic::on_unimplemented(
                //         message = "the member `{Self}` was already set, but this method requires it to be unset",
                //         label = "the member `{Self}` was already set, but this method requires it to be unset",
                //     )
                // )]
                // pub trait IsUnset: Sealed {}

                // /// Marker trait that indicates that the member is set, i.e. at least one of its setters was called.
                // // TODO: add examples (they would require having custom renames and visibility overrides for default setters)
                // #[rustversion::attr(
                //     since(1.78.0),
                //     diagnostic::on_unimplemented(
                //         message = "the member `{Self}` was not set, but this method requires it to be set",
                //         label = "the member `{Self}` was not set, but this method requires it to be set",
                //     )
                // )]
                // pub trait IsSet: Sealed {}
                /// Marker trait implemented by members that are not set.
                #[::bon::private::rustversion::attr(
                    since(1.78.0),
                    diagnostic::on_unimplemented(
                        message = "the member `{Self}` was already set, but this method requires it to be unset",
                        label = "the member `{Self}` was already set, but this method requires it to be unset",
                    )
                )]
                #vis_child trait IsUnset {
                    // Also a method without `self` makes the trait non-object safe
                    #[doc(hidden)]
                    fn __sealed(_: sealed::Sealed);
                }

                #[doc(hidden)]
                impl<Name> IsUnset for ::bon::private::Unset<Name> {
                    fn __sealed(_: sealed::Sealed) {}
                }

                #[::bon::private::rustversion::attr(
                    since(1.78.0),
                    diagnostic::on_unimplemented(
                        message = "can't finish building yet; not all required members are set",
                        label = "can't finish building yet; not all required members are set",
                    )
                )]
                #vis_child trait IsComplete: State {
                    // Also a method without `self` makes the trait non-object safe
                    #[doc(hidden)]
                    fn __sealed(_: sealed::Sealed);
                }

                #[doc(hidden)]
                impl<State: self::State> IsComplete for State
                where
                    #(
                        State::#required_members_pascal: IsSet,
                    )*
                {
                    fn __sealed(_: sealed::Sealed) {}
                }

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
                            self::members::#stateful_members_idents
                        >;
                    )*

                    #[doc(hidden)]
                    fn __sealed(_: self::sealed::Sealed);
                }

                mod sealed {
                    #vis_child_child enum Sealed {}
                }

                // Using `self::State` explicitly to avoid name conflicts with the
                // members named `state` which would create a generic param named `State`
                // that would shadow the trait `State` in the same scope.
                #[doc(hidden)]
                impl<#(
                    #stateful_members_pascal: ::bon::private::MemberState<
                        self::members::#stateful_members_idents
                    >,
                )*>
                self::State for ( #(#stateful_members_pascal,)* )
                {
                    #( type #stateful_members_pascal = #stateful_members_pascal; )*

                    fn __sealed(_: self::sealed::Sealed) {}
                }

                /// Initial state of the builder where all members are unset
                #vis_child type AllUnset = (
                    #(::bon::private::Unset<members::#stateful_members_idents>,)*
                );

                #[deprecated =
                    "this is an implementation detail and should not be \
                    used directly; use the Set* type aliases to control the \
                    state of members instead"
                ]
                #[doc(hidden)]
                mod members {
                    #(
                        #[allow(non_camel_case_types)]
                        #vis_child_child enum #stateful_members_idents {}
                    )*
                }

                #( #state_transition_aliases )*
            }
        }
    }
}
