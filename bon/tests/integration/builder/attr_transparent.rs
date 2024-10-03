mod smoke {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Debug, Builder)]
        #[allow(dead_code)]
        struct Sut<T> {
            #[builder(transparent)]
            regular: Option<u32>,

            #[builder(transparent)]
            generic: Option<T>,

            #[builder(into, transparent)]
            with_into: Option<u32>,
        }

        assert_debug_eq(
            Sut::builder()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
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
                }"#]],
        );
    }

    #[test]
    fn test_free_fn() {
        #[builder]
        fn sut<T>(
            #[builder(transparent)] regular: Option<u32>,
            #[builder(transparent)] generic: Option<T>,
            #[builder(into, transparent)] with_into: Option<u32>,
        ) -> (Option<u32>, Option<T>, Option<u32>) {
            (regular, generic, with_into)
        }

        assert_debug_eq(
            sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .call(),
            expect!["(Some(1), Some(false), Some(2))"],
        );
    }

    #[test]
    fn test_assoc_method_method() {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder]
            fn sut<T>(
                #[builder(transparent)] regular: Option<u32>,
                #[builder(transparent)] generic: Option<T>,
                #[builder(into, transparent)] with_into: Option<u32>,
            ) -> (Option<u32>, Option<T>, Option<u32>) {
                (regular, generic, with_into)
            }

            #[builder]
            fn with_self<T>(
                &self,
                #[builder(transparent)] regular: Option<u32>,
                #[builder(transparent)] generic: Option<T>,
                #[builder(into, transparent)] with_into: Option<u32>,
            ) -> (Option<u32>, Option<T>, Option<u32>) {
                let _ = self;
                (regular, generic, with_into)
            }
        }

        assert_debug_eq(
            Sut::sut()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .call(),
            expect!["(Some(1), Some(false), Some(2))"],
        );

        assert_debug_eq(
            Sut.with_self()
                .regular(Some(1))
                .generic(Some(false))
                .with_into(2)
                .call(),
            expect!["(Some(1), Some(false), Some(2))"],
        );
    }
}
