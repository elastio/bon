use super::MemberOrigin;
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromMeta;
use std::fmt;
use syn::spanned::Spanned;

#[derive(Debug, Clone, darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct MemberParams {
    /// Enables an `Into` conversion for the setter method.
    pub(crate) into: darling::util::Flag,

    /// Assign a default value to the member it it's not specified.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expression, map = Some)]
    pub(crate) default: Option<SpannedValue<Option<syn::Expr>>>,

    /// Skip generating a setter method for this member.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expression, map = Some)]
    pub(crate) skip: Option<SpannedValue<Option<syn::Expr>>>,

    /// Rename the name exposed in the builder API.
    pub(crate) name: Option<syn::Ident>,

    /// Where to place the member in the generate builder methods API.
    /// By default the member is treated like a named parameter that
    /// gets its own setter methods.
    pub(crate) source: Option<MemberInputSource>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ParamName {
    Default,
    Into,
    Name,
    Source,
    Skip,
}

impl fmt::Display for ParamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Default => "default",
            Self::Into => "into",
            Self::Name => "name",
            Self::Source => "source",
            Self::Skip => "skip",
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
            "`{attr_name}` attribute can't be specified with other \
            attributes like {}",
            conflicting,
        );
    }

    fn specified_param_names(&self) -> impl Iterator<Item = ParamName> {
        let Self {
            into,
            default,
            skip,
            name,
            source: pos,
        } = self;

        let attrs = [
            (default.is_some(), ParamName::Default),
            (name.is_some(), ParamName::Name),
            (into.is_present(), ParamName::Into),
            (pos.is_some(), ParamName::Source),
            (skip.is_some(), ParamName::Skip),
        ];

        attrs
            .into_iter()
            .filter(|(is_present, _)| *is_present)
            .map(|(_, name)| name)
    }

    pub(crate) fn validate(&self, origin: MemberOrigin) -> Result {
        if let Some(pos) = self.source {
            self.validate_mutually_allowed(ParamName::Source, pos.span(), &[ParamName::Into])?;
        }

        if let Some(skip) = &self.skip {
            match origin {
                MemberOrigin::FnArg => {
                    bail!(
                        &skip.span(),
                        "`skip` attribute is not supported on function arguments. \
                        Use a local variable instead.",
                    );
                }
                MemberOrigin::StructField => {}
            }

            if let Some(Some(_expr)) = self.default.as_deref() {
                bail!(
                    &skip.span(),
                    "`skip` attribute can't be specified with `default` attribute; \
                    if you wanted to specify a value for the member, then use \
                    the following syntax instead `#[builder(skip = value)]`",
                );
            }

            self.validate_mutually_allowed(ParamName::Skip, skip.span(), &[])?;
        }

        Ok(())
    }
}

fn parse_optional_expression(meta: &syn::Meta) -> Result<SpannedValue<Option<syn::Expr>>> {
    match meta {
        syn::Meta::Path(_) => Ok(SpannedValue::new(None, meta.span())),
        syn::Meta::List(_) => Err(Error::unsupported_format("list").with_span(meta)),
        syn::Meta::NameValue(nv) => Ok(SpannedValue::new(Some(nv.value.clone()), nv.span())),
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum MemberInputSource {
    /// The member must be a positional argument in the start function
    StartFn(Span),

    /// The member must be a positional argument in the finish function
    FinishFn(Span),
}

impl FromMeta for MemberInputSource {
    fn from_expr(expr: &syn::Expr) -> Result<Self> {
        let err = || err!(expr, "expected one of `start_fn` or `finish_fn`");
        let path = match expr {
            syn::Expr::Path(path) if path.attrs.is_empty() && path.qself.is_none() => path,
            _ => return Err(err()),
        };

        let ident = path.path.get_ident().ok_or_else(err)?.to_string();

        let kind = match ident.as_str() {
            "start_fn" => Self::StartFn,
            "finish_fn" => Self::FinishFn,
            _ => return Err(err()),
        };

        Ok(kind(path.span()))
    }
}

impl MemberInputSource {
    fn span(self) -> Span {
        match self {
            Self::StartFn(span) | Self::FinishFn(span) => span,
        }
    }
}
