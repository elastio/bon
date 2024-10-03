use crate::prelude::*;
use core::convert::Infallible;
use core::net::IpAddr;
use core::num;
type ParseIntResult<T> = Result<T, num::ParseIntError>;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct Sut<T: Default + Clone> {
        #[builder(with = |x: u32| x + 1, overwritable)]
        required: u32,

        #[builder(with = |x: u32| x + 1, overwritable)]
        optional: Option<u32>,

        #[builder(with = |x: u32| x + 1, default, overwritable)]
        default: u32,

        #[builder(with = |value: &T| value.clone(), overwritable)]
        generic: T,

        #[builder(with = |value: &T| value.clone(), overwritable)]
        optional_generic: Option<T>,

        #[builder(with = |value: &T| value.clone(), default, overwritable)]
        default_generic: T,

        #[builder(with = |value: impl Into<IpAddr>| value.into(), overwritable)]
        impl_trait: IpAddr,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() }, overwritable)]
        try_required: u32,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() }, overwritable)]
        try_optional: Option<u32>,

        #[builder(with = |value: &str| -> ParseIntResult<_> { value.parse() }, default, overwritable)]
        try_default: u32,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, overwritable)]
        try_optional_generic: Option<T>,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default, overwritable)]
        try_default_generic: T,

        #[builder(with = |value: impl Into<IpAddr>| -> Result<_, Infallible> { Ok(value.into()) }, overwritable)]
        try_impl_trait: IpAddr,
    }

    let builder = Sut::builder()
        .required(1)
        .required(2)
        .optional(3)
        .optional(4)
        .default(5)
        .default(6)
        .generic(&"hello")
        .generic(&"hello2")
        .optional_generic(&"hello you")
        .maybe_optional_generic(Some(&"littlepip"))
        .default_generic(&"blackjack")
        .maybe_default_generic(Some(&"<3"))
        .impl_trait([127, 0, 0, 1])
        .impl_trait([127, 0, 0, 2])
        .try_required("7")
        .unwrap()
        .try_required("8")
        .unwrap()
        .try_optional("9")
        .unwrap()
        .maybe_try_optional(Some("10"))
        .unwrap()
        .try_default("11")
        .unwrap()
        .maybe_try_default(Some("12"))
        .unwrap()
        .try_optional_generic(&"13")
        .unwrap()
        .maybe_try_optional_generic(Some(&"14"))
        .unwrap()
        .try_default_generic(&"15")
        .unwrap()
        .maybe_try_default_generic(Some(&"16"))
        .unwrap()
        .try_impl_trait([127, 0, 0, 3])
        .unwrap()
        .try_impl_trait([127, 0, 0, 4])
        .unwrap();

    assert_debug_eq(
        builder.build(),
        expect![[r#"
            Sut {
                required: 3,
                optional: Some(
                    5,
                ),
                default: 7,
                generic: "hello2",
                optional_generic: Some(
                    "littlepip",
                ),
                default_generic: "<3",
                impl_trait: 127.0.0.2,
                try_required: 8,
                try_optional: Some(
                    10,
                ),
                try_default: 12,
                try_optional_generic: Some(
                    "14",
                ),
                try_default_generic: "16",
                try_impl_trait: 127.0.0.4,
            }"#]],
    );
}
