mod single_arg {
    use crate::prelude::*;
    use core::net::IpAddr;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut<T: Clone> {
            #[builder(with = |x: u32| x + y)]
            required: u32,

            #[builder(with = |x: u32| 2 * x)]
            optional: Option<u32>,

            #[builder(with = |x: u32| x + y, default)]
            default: u32,

            #[builder(with = |value: &T| value.clone())]
            generic: T,

            #[builder(with = |value: impl Into<IpAddr>| value)]
            impl_trait: IpAddr,
        }

        assert_debug_eq(
            Sut::builder()
                .required(1)
                .optional(2)
                .default(3)
                .generic("hello")
                .impl_trait([127, 0, 0, 1])
                .build(),
            expect![],
        )
    }
}
