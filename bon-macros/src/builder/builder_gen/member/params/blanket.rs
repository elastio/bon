use super::MemberParams;
use crate::builder::builder_gen::builder_params::OnParams;
use crate::builder::builder_gen::member::MemberOrigin;
use crate::util::prelude::*;
use std::fmt;

pub(crate) enum BlanketParamName {
    Into,
    Mutable,
}

impl fmt::Display for BlanketParamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Into => fmt::Display::fmt(&super::ParamName::Into, f),
            Self::Mutable => fmt::Display::fmt(&super::ParamName::Mutable, f),
        }
    }
}

impl BlanketParamName {
    fn value_in_on_params(&self, on_params: &OnParams) -> darling::util::Flag {
        match self {
            Self::Into => on_params.into,
            Self::Mutable => on_params.mutable,
        }
    }

    fn value_in_member_params(&self, member_params: &MemberParams) -> darling::util::Flag {
        match self {
            Self::Into => member_params.into,
            Self::Mutable => member_params.mutable,
        }
    }
}

pub(crate) struct EvalBlanketFlagParam<'a> {
    pub(crate) on_params: &'a [OnParams],
    pub(crate) param_name: BlanketParamName,
    pub(crate) member_params: &'a MemberParams,
    pub(crate) scrutinee: &'a syn::Type,
    pub(crate) origin: MemberOrigin,
}

impl EvalBlanketFlagParam<'_> {
    pub(crate) fn eval(self) -> Result<darling::util::Flag> {
        let Self {
            on_params,
            param_name,
            member_params,
            scrutinee,
            origin,
        } = self;

        let verdict_from_on_params = on_params
            .iter()
            .map(|params| Ok((params, scrutinee.matches(&params.type_pattern)?)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|(_, matched)| *matched)
            .map(|(params, _)| param_name.value_in_on_params(&params))
            .find(|flag| flag.is_present());

        let value_in_member = param_name.value_in_member_params(member_params);
        let flag = match (verdict_from_on_params, value_in_member.is_present()) {
            (Some(_), true) => {
                bail!(
                    &value_in_member.span(),
                    "this `#[builder({param_name})]` attribute is redundant, because \
                    `{param_name}` is already implied for this member via the \
                    `#[builder(on(...))]` at the top of the {}",
                    origin.parent_construct(),
                );
            }
            (Some(flag), false) => flag,
            (None, _) => value_in_member,
        };

        Ok(flag)
    }
}
