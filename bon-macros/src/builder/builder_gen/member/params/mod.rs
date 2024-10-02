mod blanket;
mod setter;

pub(crate) use blanket::{BlanketParamName, EvalBlanketFlagParam};
pub(crate) use setter::SettersParams;

use super::MemberOrigin;
use crate::util::prelude::*;
use darling::util::SpannedValue;
use std::fmt;
use syn::spanned::Spanned;

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
    pub(crate) default: Option<SpannedValue<Option<syn::Expr>>>,

    /// Skip generating a setter method for this member.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expr, map = Some)]
    pub(crate) skip: Option<SpannedValue<Option<syn::Expr>>>,

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

    #[darling(default, with = parse_expr_closure, map = Some)]
    pub(crate) with: Option<syn::ExprClosure>,
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
            Self::With => "with",
        };
        f.write_str(str)
    }
}

impl MemberParams {
    fn validate_mutually_allowed(
        &self,
        attr_name: ParamName,
        attr_span: Span,
        allowed: &[ParamName],
    ) -> Result<()> {
        let conflicting: Vec<_> = self
            .specified_param_names()
            .filter(|name| *name != attr_name && !allowed.contains(name))
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
                        &skip.span(),
                        "`skip` attribute is not supported on function arguments; \
                        use a local variable instead.",
                    );
                }
                MemberOrigin::StructField => {}
            }

            if let Some(Some(_expr)) = self.default.as_deref() {
                bail!(
                    &skip.span(),
                    "`skip` attribute can't be specified with the `default` attribute; \
                    if you wanted to specify a value for the member, then use \
                    the following syntax instead `#[builder(skip = value)]`",
                );
            }

            self.validate_mutually_allowed(ParamName::Skip, skip.span(), &[])?;
        }

        Ok(())
    }
}

fn parse_optional_expr(meta: &syn::Meta) -> Result<SpannedValue<Option<syn::Expr>>> {
    match meta {
        syn::Meta::Path(_) => Ok(SpannedValue::new(None, meta.span())),
        syn::Meta::List(_) => Err(Error::unsupported_format("list").with_span(meta)),
        syn::Meta::NameValue(meta) => Ok(SpannedValue::new(Some(meta.value.clone()), meta.span())),
    }
}

fn parse_expr_closure(meta: &syn::Meta) -> Result<syn::ExprClosure> {
    let err = || {
        let path = darling::util::path_to_string(meta.path());
        err!(
            meta,
            "expected a closure e.g. `{path} = |param: T| expression`"
        )
    };

    let meta = match meta {
        syn::Meta::NameValue(meta) => meta,
        _ => return Err(err()),
    };

    match &meta.value {
        syn::Expr::Closure(closure) => Ok(closure.clone()),
        _ => Err(err()),
    }
}
