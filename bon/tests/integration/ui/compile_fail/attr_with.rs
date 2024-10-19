use bon::Builder;

#[derive(Builder)]
struct InvalidWithExpr {
    #[builder(with = 42)]
    x: u32,
}

#[derive(Builder)]
struct ConflictingInto {
    #[builder(into, with = |x: u32| x + 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectForSyntax {
    #[builder(with = for<'a> |x: &'a u32| -> u32 { x + 1 })]
    value: u32,
}

#[derive(Builder)]
struct RejectConstSyntax {
    #[builder(with = const || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectStaticSyntax {
    #[builder(with = static || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectAsyncSyntax {
    #[builder(with = async || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectMoveSyntax {
    #[builder(with = move || 1)]
    value: u32,
}

#[derive(Builder)]
struct UnexpectedReturnTypeShape {
    #[builder(with = |x: u32| -> u32 { x + 1 })]
    value: u32,
}

#[derive(Builder)]
struct InvalidReturnTypeInResultClosure {
    #[builder(with = |value: &str| -> ::core::result::Result<_, core::num::ParseIntError> {
        Ok(value)
    })]
    value: u32,
}

#[derive(Builder)]
struct InvalidReturnTypeInImplTraitClosure {
    #[builder(with = |value: impl Into<::core::net::IpAddr>| value)]
    value: u32,
}

fn main() {}
