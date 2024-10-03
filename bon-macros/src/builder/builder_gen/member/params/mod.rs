mod blanket;
mod setter;

pub(crate) use blanket::*;
pub(crate) use setter::*;

use super::MemberOrigin;
use crate::parsing::{SimpleClosure, SpannedKey};
use crate::util::prelude::*;
use darling::FromMeta;
use std::fmt;

#[derive(Debug, darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct MemberParams {
    /// Enables an `Into` conversion for the setter method.
    pub(crate) into: darling::util::Flag,

    /// Assign a default value to the member it it's not specified.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expr, map = Some)]
    pub(crate) default: Option<SpannedKey<Option<syn::Expr>>>,

    /// Skip generating a setter method for this member.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expr, map = Some)]
    pub(crate) skip: Option<SpannedKey<Option<syn::Expr>>>,

    /// Rename the name exposed in the builder API.
    pub(crate) name: Option<syn::Ident>,

    /// Configurations for the setter methods.
    #[darling(with = crate::parsing::require_paren_delim_for_meta_list)]
    pub(crate) setters: Option<SettersParams>,

    /// Where to place the member in the generated builder methods API.
    /// By default the member is treated like a named parameter that
    /// gets its own setter methods.
    pub(crate) start_fn: darling::util::Flag,
    pub(crate) finish_fn: darling::util::Flag,

    /// Allows setting the value for the member repeatedly. This reduces the
    /// number of type states and thus increases the compilation performance.
    ///
    /// However, this also means that unintended overwrites won't be caught
    /// at compile time. Measure the compilation time before and after enabling
    /// this option to see if it's worth it.
    pub(crate) overwritable: darling::util::Flag,

    /// Customize the setter signature and body with a custom closure. The closure
    /// must return the value of the type of the member, or optionally a `Result<_>`
    /// type where `_` is used to mark the type of the member. In this case the
    /// generated setters will be fallible (they'll propagate the `Result`).
    pub(crate) with: Option<SpannedKey<SetterClosure>>,

    /// Disables the special handling for a member of type `Option<T>`. The
    /// member no longer has the default on `None`. It also becomes a required
    /// member unless a separate `#[builder(default = ...)]` attribute is
    /// also specified.
    pub(crate) transparent: darling::util::Flag,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ParamName {
    Default,
    FinishFn,
    Into,
    Name,
    Overwritable,
    Setters,
    Skip,
    StartFn,
    Transparent,
    With,
}

impl fmt::Display for ParamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Default => "default",
            Self::FinishFn => "finish_fn",
            Self::Into => "into",
            Self::Name => "name",
            Self::Overwritable => "overwritable",
            Self::Setters => "setters",
            Self::Skip => "skip",
            Self::StartFn => "start_fn",
            Self::Transparent => "transparent",
            Self::With => "with",
        };
        f.write_str(str)
    }
}

impl MemberParams {
    fn validate_mutually_exclusive(
        &self,
        attr_name: ParamName,
        attr_span: Span,
        mutually_exclusive: &[ParamName],
    ) -> Result<()> {
        self.validate_compat(attr_name, attr_span, mutually_exclusive, true)
    }

    fn validate_mutually_allowed(
        &self,
        attr_name: ParamName,
        attr_span: Span,
        mutually_allowed: &[ParamName],
    ) -> Result<()> {
        self.validate_compat(attr_name, attr_span, mutually_allowed, false)
    }

    fn validate_compat(
        &self,
        attr_name: ParamName,
        attr_span: Span,
        patterns: &[ParamName],
        mutually_exclusive: bool,
    ) -> Result<()> {
        let conflicting: Vec<_> = self
            .specified_param_names()
            .filter(|name| *name != attr_name && patterns.contains(name) == mutually_exclusive)
            .collect();

        if conflicting.is_empty() {
            return Ok(());
        }

        let conflicting = conflicting
            .iter()
            .map(|name| format!("`{name}`"))
            .join(", ");

        bail!(
            &attr_span,
            "`{attr_name}` attribute can't be specified together with {conflicting}",
        );
    }

    fn specified_param_names(&self) -> impl Iterator<Item = ParamName> {
        let Self {
            default,
            finish_fn,
            into,
            name,
            overwritable,
            setters,
            skip,
            start_fn,
            transparent,
            with,
        } = self;

        let attrs = [
            (default.is_some(), ParamName::Default),
            (finish_fn.is_present(), ParamName::FinishFn),
            (into.is_present(), ParamName::Into),
            (name.is_some(), ParamName::Name),
            (overwritable.is_present(), ParamName::Overwritable),
            (setters.is_some(), ParamName::Setters),
            (skip.is_some(), ParamName::Skip),
            (start_fn.is_present(), ParamName::StartFn),
            (transparent.is_present(), ParamName::Transparent),
            (with.is_some(), ParamName::With),
        ];

        attrs
            .into_iter()
            .filter(|(is_present, _)| *is_present)
            .map(|(_, name)| name)
    }

    pub(crate) fn validate(&self, origin: MemberOrigin) -> Result {
        if self.start_fn.is_present() {
            self.validate_mutually_allowed(
                ParamName::StartFn,
                self.start_fn.span(),
                &[ParamName::Into],
            )?;
        }

        if self.finish_fn.is_present() {
            self.validate_mutually_allowed(
                ParamName::FinishFn,
                self.finish_fn.span(),
                &[ParamName::Into],
            )?;
        }

        if let Some(skip) = &self.skip {
            match origin {
                MemberOrigin::FnArg => {
                    bail!(
                        &skip.key.span(),
                        "`skip` attribute is not supported on function arguments; \
                        use a local variable instead.",
                    );
                }
                MemberOrigin::StructField => {}
            }

            if let Some(Some(_expr)) = self.default.as_deref() {
                bail!(
                    &skip.key.span(),
                    "`skip` attribute can't be specified with the `default` attribute; \
                    if you wanted to specify a value for the member, then use \
                    the following syntax instead `#[builder(skip = value)]`",
                );
            }

            self.validate_mutually_allowed(ParamName::Skip, skip.key.span(), &[])?;
        }

        if let Some(with) = &self.with {
            self.validate_mutually_exclusive(ParamName::With, with.key.span(), &[ParamName::Into])?;
        }

        Ok(())
    }
}

fn parse_optional_expr(meta: &syn::Meta) -> Result<SpannedKey<Option<syn::Expr>>> {
    match meta {
        syn::Meta::Path(path) => SpannedKey::new(path, None),
        syn::Meta::List(_) => Err(Error::unsupported_format("list").with_span(meta)),
        syn::Meta::NameValue(meta) => SpannedKey::new(&meta.path, Some(meta.value.clone())),
    }
}

#[derive(Debug)]
pub(crate) struct SetterClosure {
    pub(crate) inputs: Vec<SetterClosureInput>,
    pub(crate) body: Box<syn::Expr>,
    pub(crate) output: Option<SetterClosureOutput>,
}

#[derive(Debug)]
pub(crate) struct SetterClosureOutput {
    pub(crate) result_path: syn::Path,
    pub(crate) err_ty: Option<syn::Type>,
}

#[derive(Debug)]
pub(crate) struct SetterClosureInput {
    pub(crate) pat: syn::PatIdent,
    pub(crate) ty: Box<syn::Type>,
}

impl FromMeta for SetterClosure {
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        let closure = SimpleClosure::from_meta(item)?;

        let inputs = closure
            .inputs
            .into_iter()
            .map(|input| {
                Ok(SetterClosureInput {
                    ty: input.ty.ok_or_else(|| {
                        err!(&input.pat, "expected a type for the setter input parameter")
                    })?,
                    pat: input.pat,
                })
            })
            .collect::<Result<_>>()?;

        let return_type = match closure.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => {
                let err = || {
                    err!(
                        &ty,
                        "expected one of the following syntaxes:\n\
                        (1) no return type annotation;\n\
                        (2) `-> Result<_, {{ErrorType}}>` or `-> Result<_>` return type annotation;\n\n\
                        in the case (1), the closure is expected to return a value \
                        of the same type as the member's type;\n\n\
                        in the case (2), the closure is expected to return a `Result` \
                        where the `Ok` variant is of the same type as the member's type; \
                        the `_` placeholder must be spelled literally to mark \
                        the type of the member; an optional second generic parameter \
                        for the error type is allowed"
                    )
                };

                let args = ty.as_generic_angle_bracketed("Result").ok_or_else(err)?;

                if args.len() != 1 && args.len() != 2 {
                    return Err(err());
                }

                let mut args = args.into_iter();
                let ok_ty = args.next().ok_or_else(err)?;

                if !matches!(ok_ty, syn::GenericArgument::Type(syn::Type::Infer(_))) {
                    return Err(err());
                }

                let err_ty = args
                    .next()
                    .map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => Ok(ty.clone()),
                        _ => Err(err()),
                    })
                    .transpose()?;

                Some(SetterClosureOutput {
                    result_path: syn::parse_quote!(Result),
                    err_ty,
                })
            }
        };

        Ok(Self {
            inputs,
            body: closure.body,
            output: return_type,
        })
    }
}
