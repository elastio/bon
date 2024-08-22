use crate::util::iterator::IntoIteratorExt;
use crate::util::prelude::*;
use syn::spanned::Spanned;

pub(crate) fn match_return_types(
    scrutinee: &syn::ReturnType,
    pattern: &syn::ReturnType,
) -> Result<bool> {
    match (scrutinee, pattern) {
        (syn::ReturnType::Default, syn::ReturnType::Default) => Ok(true),
        (syn::ReturnType::Default, syn::ReturnType::Type(_, pattern)) => {
            match_types(&syn::parse_quote!(()), pattern)
        }
        (syn::ReturnType::Type(_, scrutinee), syn::ReturnType::Default) => {
            Ok(**scrutinee == syn::parse_quote!(()))
        }
        (syn::ReturnType::Type(_, scrutinee), syn::ReturnType::Type(_, pattern)) => {
            match_types(scrutinee, pattern)
        }
    }
}

fn match_paths(scrutinee: &syn::Path, pattern: &syn::Path) -> Result<bool> {
    let verdict = scrutinee.leading_colon == pattern.leading_colon
        && scrutinee
            .segments
            .iter()
            .try_equals_with(&pattern.segments, |scrutinee, pattern| {
                let verdict = scrutinee.ident == pattern.ident
                    && match_path_args(&scrutinee.arguments, &pattern.arguments)?;

                Ok(verdict)
            })?;

    Ok(verdict)
}

fn match_path_args(scrutinee: &syn::PathArguments, pattern: &syn::PathArguments) -> Result<bool> {
    use syn::PathArguments::*;

    let verdict = match (scrutinee, pattern) {
        (None, None) => true,
        (AngleBracketed(scrutinee), AngleBracketed(pattern)) => {
            match_angle_bracketed_generic_args(scrutinee, pattern)?
        }
        (Parenthesized(scrutinee), Parenthesized(pattern)) => {
            scrutinee
                .inputs
                .iter()
                .try_equals_with(&pattern.inputs, match_types)?
                && match_return_types(&scrutinee.output, &pattern.output)?
        }
        _ => false,
    };

    Ok(verdict)
}

fn match_angle_bracketed_generic_args(
    scrutinee: &syn::AngleBracketedGenericArguments,
    pattern: &syn::AngleBracketedGenericArguments,
) -> Result<bool> {
    scrutinee
        .args
        .iter()
        .try_equals_with(&pattern.args, match_generic_args)
}

fn match_option<T>(
    scrutinee: &Option<T>,
    pattern: &Option<T>,
    compare: impl Fn(&T, &T) -> Result<bool>,
) -> Result<bool> {
    match (scrutinee, &pattern) {
        (None, None) => Ok(true),
        (Some(scrutinee), Some(pattern)) => compare(scrutinee, pattern),
        _ => Ok(false),
    }
}

fn match_generic_args(
    scrutinee: &syn::GenericArgument,
    pattern: &syn::GenericArgument,
) -> Result<bool> {
    use syn::GenericArgument::*;

    let verdict = match (scrutinee, pattern) {
        (Lifetime(scrutinee), Lifetime(pattern)) => scrutinee == pattern,
        (Type(scrutinee), Type(pattern)) => match_types(scrutinee, pattern)?,
        (Constraint(scrutinee), Constraint(pattern)) => scrutinee == pattern,
        (Const(scrutinee), Const(pattern)) => match_exprs(scrutinee, pattern),
        (AssocType(scrutinee), AssocType(pattern)) => {
            scrutinee.ident == pattern.ident
                && match_types(&scrutinee.ty, &pattern.ty)?
                && match_option(
                    &scrutinee.generics,
                    &pattern.generics,
                    match_angle_bracketed_generic_args,
                )?
        }
        (AssocConst(scrutinee), AssocConst(pattern)) => {
            scrutinee.ident == pattern.ident
                && match_option(
                    &scrutinee.generics,
                    &pattern.generics,
                    match_angle_bracketed_generic_args,
                )?
                && match_exprs(&scrutinee.value, &pattern.value)
        }
        _ => false,
    };

    Ok(verdict)
}

fn match_exprs(scrutinee: &syn::Expr, pattern: &syn::Expr) -> bool {
    matches!(pattern, syn::Expr::Infer(_)) || scrutinee == pattern
}

pub(crate) fn match_types(scrutinee: &syn::Type, pattern: &syn::Type) -> Result<bool> {
    use syn::Type::*;

    let pattern = pattern.peel();

    if let Infer(_) = pattern {
        return Ok(true);
    }

    let scrutinee = scrutinee.peel();

    let verdict = match pattern {
        Array(pattern) => {
            let Array(scrutinee) = scrutinee else {
                return Ok(false);
            };

            match_types(&scrutinee.elem, &pattern.elem)?
                && match_exprs(&scrutinee.len, &pattern.len)
        }
        Path(pattern) => {
            if let Some(qself) = &pattern.qself {
                return Err(unsupported_syntax_error(qself, "<T as Trait> syntax"));
            }

            let Path(scrutinee) = scrutinee else {
                return Ok(false);
            };

            scrutinee.qself.is_none() && match_paths(&scrutinee.path, &pattern.path)?
        }
        Ptr(pattern) => {
            let Ptr(scrutinee) = scrutinee else {
                return Ok(false);
            };
            scrutinee.const_token == pattern.const_token
                && scrutinee.mutability == pattern.mutability
                && match_types(&scrutinee.elem, &pattern.elem)?
        }
        Reference(pattern) => {
            if let Some(lifetime) = &pattern.lifetime {
                return Err(unsupported_syntax_error(
                    lifetime,
                    "Lifetimes are ignored during type pattern matching. \
                    Explicit lifetime syntax",
                ));
            }

            let Reference(scrutinee) = scrutinee else {
                return Ok(false);
            };

            scrutinee.mutability == pattern.mutability
                && match_types(&scrutinee.elem, &pattern.elem)?
        }
        Slice(pattern) => {
            let Slice(scrutinee) = scrutinee else {
                return Ok(false);
            };
            match_types(&scrutinee.elem, &pattern.elem)?
        }
        Tuple(pattern) => {
            let Tuple(scrutinee) = scrutinee else {
                return Ok(false);
            };
            scrutinee
                .elems
                .iter()
                .try_equals_with(&pattern.elems, match_types)?
        }

        Never(_) => matches!(scrutinee, Never(_)),

        _ => return Err(unsupported_syntax_error(&pattern, "This syntax")),
    };

    Ok(verdict)
}

fn unsupported_syntax_error(spanned: &impl Spanned, syntax: &str) -> Error {
    err!(
        spanned,
        "{syntax} is not supported in type patterns yet. If you have \
        a use case for this, please open an issue at \
        https://github.com/elastio/bon/issues."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote as pq;

    #[track_caller]
    fn assert_match_types(scrutinee: syn::Type, pattern: syn::Type) {
        // Make sure pure wildcard matches everything
        assert!(scrutinee.matches(&pq!(_)).unwrap());

        assert!(scrutinee.matches(&pattern).unwrap());
    }

    #[track_caller]
    fn assert_not_match_types(scrutinee: syn::Type, pattern: syn::Type) {
        assert!(!scrutinee.matches(&pattern).unwrap());
    }

    #[track_caller]
    fn assert_unsupported(pattern: syn::Type) {
        let scrutinee: syn::Type = syn::parse_quote!(());
        let result = scrutinee.matches(&pattern);
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("is not supported in type patterns yet"),
            "Error: {err}"
        );
    }

    #[test]
    fn arrays() {
        assert_match_types(pq!([u8; 4]), pq!([u8; 4]));
        assert_match_types(pq!([u8; 4]), pq!([_; 4]));
        assert_match_types(pq!([u8; 4]), pq!([_; _]));
        assert_match_types(pq!([u8; 4]), pq!([u8; _]));

        assert_not_match_types(pq!([u8; 4]), pq!([u8; 5]));
        assert_not_match_types(pq!([u8; 4]), pq!([_; 5]));

        assert_not_match_types(pq!([u8; 4]), pq!([u16; 4]));
        assert_not_match_types(pq!([u8; 4]), pq!([u16; _]));

        assert_not_match_types(pq!([u8; 4]), pq!([_]));
        assert_not_match_types(pq!([u8; 4]), pq!([u8]));
    }

    #[test]
    fn paths() {
        assert_match_types(pq!(bool), pq!(bool));
        assert_match_types(pq!(foo::Bar), pq!(foo::Bar));
        assert_match_types(pq!(crate::foo::Bar), pq!(crate::foo::Bar));
        assert_match_types(pq!(super::foo::Bar), pq!(super::foo::Bar));

        assert_not_match_types(pq!(::Bar), pq!(Bar));
        assert_not_match_types(pq!(Bar), pq!(::Bar));
        assert_not_match_types(pq!(super::foo::Bar), pq!(crate::foo::Bar));

        assert_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar<_>));
        assert_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar<u32>));
        assert_match_types(pq!(foo::Bar<u32, bool>), pq!(foo::Bar<u32, _>));
        assert_match_types(pq!(foo::Bar<u32, bool>), pq!(foo::Bar<_, bool>));
        assert_match_types(pq!(foo::Bar<u32, bool>), pq!(foo::Bar<u32, bool>));
        assert_match_types(pq!(foo::Bar<u32, bool>), pq!(foo::Bar<_, _>));

        assert_not_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar<bool>));
        assert_not_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar));
        assert_not_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar<u32, _>));
        assert_not_match_types(pq!(foo::Bar<u32>), pq!(foo::Bar<_, _>));
        assert_not_match_types(pq!(foo::Foo<u32>), pq!(foo::Bar<u32>));
    }

    #[test]
    fn unsupported() {
        assert_unsupported(pq!(dyn Trait));
        assert_unsupported(pq!(dyn FnOnce()));

        assert_unsupported(pq!(impl Trait));
        assert_unsupported(pq!(impl FnOnce()));

        assert_unsupported(pq!(fn()));

        assert_unsupported(pq!(&'a _));
        assert_unsupported(pq!(&'_ _));
        assert_unsupported(pq!(&'static _));
    }
}
