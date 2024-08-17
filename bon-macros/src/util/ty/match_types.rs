use crate::util::iterator::IntoIteratorExt;
use crate::util::prelude::*;
use syn::punctuated::Punctuated;

pub(crate) fn match_return_types(scrutinee: &syn::ReturnType, pattern: &syn::ReturnType) -> bool {
    match (scrutinee, pattern) {
        (syn::ReturnType::Default, syn::ReturnType::Default) => true,
        (syn::ReturnType::Default, syn::ReturnType::Type(_, pattern)) => {
            match_types(&syn::parse_quote!(()), pattern)
        }
        (syn::ReturnType::Type(_, scrutinee), syn::ReturnType::Default) => {
            **scrutinee == syn::parse_quote!(())
        }
        (syn::ReturnType::Type(_, scrutinee), syn::ReturnType::Type(_, pattern)) => {
            match_types(scrutinee, pattern)
        }
    }
}

pub(crate) fn match_bare_fn_args(
    scrutinee: &Punctuated<syn::BareFnArg, syn::Token![,]>,
    pattern: &Punctuated<syn::BareFnArg, syn::Token![,]>,
) -> bool {
    scrutinee.equals_with(pattern, |scrutinee, pattern| {
        match_types(&scrutinee.ty, &pattern.ty)
    })
}

fn match_paths(scrutinee: &syn::Path, pattern: &syn::Path) -> bool {
    scrutinee.leading_colon == pattern.leading_colon
        && scrutinee
            .segments
            .iter()
            .equals_with(&pattern.segments, |scrutinee, pattern| {
                scrutinee.ident == pattern.ident
                    && match_path_args(&scrutinee.arguments, &pattern.arguments)
            })
}

fn match_path_args(scrutinee: &syn::PathArguments, pattern: &syn::PathArguments) -> bool {
    use syn::PathArguments::*;

    match (scrutinee, pattern) {
        (None, None) => true,
        (AngleBracketed(scrutinee), AngleBracketed(pattern)) => {
            match_angle_bracketed_generic_args(scrutinee, pattern)
        }
        (Parenthesized(scrutinee), Parenthesized(pattern)) => {
            scrutinee
                .inputs
                .iter()
                .equals_with(&pattern.inputs, |scrutinee, pattern| {
                    match_types(scrutinee, pattern)
                })
                && match_return_types(&scrutinee.output, &pattern.output)
        }
        _ => false,
    }
}

fn match_angle_bracketed_generic_args(
    scrutinee: &syn::AngleBracketedGenericArguments,
    pattern: &syn::AngleBracketedGenericArguments,
) -> bool {
    scrutinee
        .args
        .iter()
        .equals_with(&pattern.args, |scrutinee, pattern| {
            match_generic_args(scrutinee, pattern)
        })
}

fn match_option<T>(
    scrutinee: &Option<T>,
    pattern: &Option<T>,
    compare: impl Fn(&T, &T) -> bool,
) -> bool {
    match (scrutinee, &pattern) {
        (None, None) => true,
        (Some(scrutinee), Some(pattern)) => compare(scrutinee, pattern),
        _ => false,
    }
}

fn match_generic_args(scrutinee: &syn::GenericArgument, pattern: &syn::GenericArgument) -> bool {
    use syn::GenericArgument::*;

    match (scrutinee, pattern) {
        (Lifetime(scrutinee), Lifetime(pattern)) => scrutinee == pattern,
        (Type(scrutinee), Type(pattern)) => match_types(scrutinee, pattern),
        (Constraint(scrutinee), Constraint(pattern)) => scrutinee == pattern,
        (Const(scrutinee), Const(pattern)) => match_exprs(scrutinee, pattern),
        (AssocType(scrutinee), AssocType(pattern)) => {
            scrutinee.ident == pattern.ident
                && match_types(&scrutinee.ty, &pattern.ty)
                && match_option(
                    &scrutinee.generics,
                    &pattern.generics,
                    match_angle_bracketed_generic_args,
                )
        }
        (AssocConst(scrutinee), AssocConst(pattern)) => {
            scrutinee.ident == pattern.ident
                && match_option(
                    &scrutinee.generics,
                    &pattern.generics,
                    match_angle_bracketed_generic_args,
                )
                && match_exprs(&scrutinee.value, &pattern.value)
        }
        _ => false,
    }
}

fn match_exprs(scrutinee: &syn::Expr, pattern: &syn::Expr) -> bool {
    matches!(pattern, syn::Expr::Infer(_)) || scrutinee == pattern
}

pub(crate) fn match_types(scrutinee: &syn::Type, pattern: &syn::Type) -> bool {
    use syn::Type::*;

    if let Infer(_) = pattern {
        return true;
    }

    match (scrutinee.peel(), pattern.peel()) {
        (Array(scrutinee), Array(pattern)) => {
            match_types(&scrutinee.elem, &pattern.elem) && match_exprs(&scrutinee.len, &pattern.len)
        }
        (BareFn(scrutinee), BareFn(pattern)) => {
            scrutinee.lifetimes == pattern.lifetimes
                && scrutinee.unsafety == pattern.unsafety
                && scrutinee.abi == pattern.abi
                && scrutinee.variadic == pattern.variadic
                && match_return_types(&scrutinee.output, &pattern.output)
                && match_bare_fn_args(&scrutinee.inputs, &pattern.inputs)
        }
        (Path(scrutinee), Path(pattern)) => {
            scrutinee.qself == pattern.qself && match_paths(&scrutinee.path, &pattern.path)
        }
        (Ptr(scrutinee), Ptr(pattern)) => {
            scrutinee.const_token == pattern.const_token
                && scrutinee.mutability == pattern.mutability
                && match_types(&scrutinee.elem, &pattern.elem)
        }
        (Reference(scrutinee), Reference(pattern)) => {
            scrutinee.lifetime == pattern.lifetime
                && scrutinee.mutability == pattern.mutability
                && match_types(&scrutinee.elem, &pattern.elem)
        }
        (Slice(scrutinee), Slice(pattern)) => match_types(&scrutinee.elem, &pattern.elem),

        (Tuple(scrutinee), Tuple(pattern)) => scrutinee
            .elems
            .iter()
            .equals_with(&pattern.elems, match_types),

        (ImplTrait(_), ImplTrait(_))
        | (Macro(_), Macro(_))
        | (Never(_), Never(_))
        | (TraitObject(_), TraitObject(_))
        | (Verbatim(_), Verbatim(_)) => scrutinee == pattern,

        _ => false,
    }
}
