mod member_level {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut<T> {
            #[builder(transparent)]
            regular: Option<u32>,

            #[builder(transparent)]
            generic: Option<T>,

            #[builder(transparent, into)]
            with_into: Option<u32>,

            #[builder(transparent, default = Some(99))]
            with_default: Option<u32>,

            #[builder(transparent, default = Some(10))]
            with_default_2: Option<u32>,
        }

        assert_debug_eq(
            Sut::builder()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .build(),
            expect![[r#"
                Sut {
                    regular: Some(
                        1,
                    ),
                    generic: Some(
                        false,
                    ),
                    with_into: Some(
                        2,
                    ),
                    with_default: Some(
                        99,
                    ),
                    with_default_2: Some(
                        3,
                    ),
                }"#]],
        );
    }

    #[test]
    fn test_free_fn() {
        #[builder]
        fn sut<T: fmt::Debug>(
            #[builder(transparent)] regular: Option<u32>,
            #[builder(transparent)] generic: Option<T>,
            #[builder(transparent, into)] with_into: Option<u32>,
            #[builder(transparent, default = Some(99))] with_default: Option<u32>,
            #[builder(transparent, default = Some(10))] with_default_2: Option<u32>,
        ) -> impl fmt::Debug {
            (regular, generic, with_into, with_default, with_default_2)
        }

        assert_debug_eq(
            sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }

    #[test]
    fn test_assoc_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut<T: fmt::Debug>(
                #[builder(transparent)] regular: Option<u32>,
                #[builder(transparent)] generic: Option<T>,
                #[builder(transparent, into)] with_into: Option<u32>,
                #[builder(transparent, default = Some(99))] with_default: Option<u32>,
                #[builder(transparent, default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                (regular, generic, with_into, with_default, with_default_2)
            }

            #[builder]
            fn with_self<T: fmt::Debug>(
                &self,
                #[builder(transparent)] regular: Option<u32>,
                #[builder(transparent)] generic: Option<T>,
                #[builder(transparent, into)] with_into: Option<u32>,
                #[builder(transparent, default = Some(99))] with_default: Option<u32>,
                #[builder(transparent, default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                let _ = self;
                (regular, generic, with_into, with_default, with_default_2)
            }
        }

        assert_debug_eq(
            Sut::sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );

        assert_debug_eq(
            Sut.with_self()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }
}

mod attr_on {
    use crate::prelude::*;
    use core::fmt;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[builder(on(_, transparent))]
        #[allow(dead_code)]
        struct Sut<T> {
            regular: Option<u32>,
            generic: Option<T>,

            #[builder(into)]
            with_into: Option<u32>,

            #[builder(default = Some(99))]
            with_default: Option<u32>,

            #[builder(default = Some(10))]
            with_default_2: Option<u32>,
        }

        assert_debug_eq(
            Sut::builder()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .build(),
            expect![[r#"
                Sut {
                    regular: Some(
                        1,
                    ),
                    generic: Some(
                        false,
                    ),
                    with_into: Some(
                        2,
                    ),
                    with_default: Some(
                        99,
                    ),
                    with_default_2: Some(
                        3,
                    ),
                }"#]],
        );
    }

    #[test]
    fn test_free_fn() {
        #[builder(on(_, transparent))]
        fn sut<T: fmt::Debug>(
            regular: Option<u32>,
            generic: Option<T>,
            #[builder(into)] with_into: Option<u32>,
            #[builder(default = Some(99))] with_default: Option<u32>,
            #[builder(default = Some(10))] with_default_2: Option<u32>,
        ) -> impl fmt::Debug {
            (regular, generic, with_into, with_default, with_default_2)
        }

        assert_debug_eq(
            sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }

    #[test]
    fn test_assoc_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(on(_, transparent))]
            fn sut<T: fmt::Debug>(
                regular: Option<u32>,
                generic: Option<T>,
                #[builder(into)] with_into: Option<u32>,
                #[builder(default = Some(99))] with_default: Option<u32>,
                #[builder(default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                (regular, generic, with_into, with_default, with_default_2)
            }

            #[builder(on(_, transparent))]
            fn with_self<T: fmt::Debug>(
                &self,
                regular: Option<u32>,
                generic: Option<T>,
                #[builder(into)] with_into: Option<u32>,
                #[builder(default = Some(99))] with_default: Option<u32>,
                #[builder(default = Some(10))] with_default_2: Option<u32>,
            ) -> impl fmt::Debug {
                let _ = self;
                (regular, generic, with_into, with_default, with_default_2)
            }
        }

        assert_debug_eq(
            Sut::sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );

        assert_debug_eq(
            Sut.with_self()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .maybe_with_default_2(Some(Some(3)))
                .call(),
            expect!["(Some(1), Some(false), Some(2), Some(99), Some(3))"],
        );
    }
}
