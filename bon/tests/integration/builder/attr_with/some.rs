use crate::prelude::*;
use core::fmt;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Clone))]
    struct Sut<T> {
        #[builder(transparent, with = Some)]
        _required: Option<i32>,

        #[builder(transparent, with = Some, default = Some(()))]
        _optional: Option<()>,

        #[builder(transparent, with = Some)]
        _generic: Option<T>,

        #[builder(transparent, with = Some, default = None)]
        _optional_generic: Option<T>,
    }

    let builder = Sut::builder();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.build();

    assert_debug_eq(&sut, expect![[r#"
        Sut {
            _required: Some(
                99,
            ),
            _optional: Some(
                (),
            ),
            _generic: Some(
                2,
            ),
            _optional_generic: Some(
                22,
            ),
        }"#]]);
}

#[test]
fn test_free_fn() {
    #[builder(derive(Clone))]
    fn sut<T: fmt::Debug>(
        #[builder(transparent, with = Some)] required: Option<i32>,
        #[builder(transparent, with = Some, default = Some(()))] optional: Option<()>,
        #[builder(transparent, with = Some)] generic: Option<T>,
        #[builder(transparent, with = Some)] impl_trait: Option<impl fmt::Debug>,
        #[builder(transparent, with = Some, default = None)] optional_generic: Option<T>,
    ) -> impl fmt::Debug {
        (required, optional, generic, impl_trait, optional_generic)
    }

    let builder = sut();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);
    let builder = builder.impl_trait("impl Trait");

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.call();

    assert_debug_eq(&sut, expect![[r#"(Some(99), Some(()), Some(2), Some("impl Trait"), Some(22))"#]]);
}

#[test]
fn test_assoc_method() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(derive(Clone))]
        fn sut<T: fmt::Debug>(
            #[builder(transparent, with = Some)] required: Option<i32>,
            #[builder(transparent, with = Some, default = Some(()))] optional: Option<()>,
            #[builder(transparent, with = Some)] generic: Option<T>,
            #[builder(transparent, with = Some)] impl_trait: Option<impl fmt::Debug>,
            #[builder(transparent, with = Some, default = None)] optional_generic: Option<T>,
        ) -> impl fmt::Debug {
            (required, optional, generic, impl_trait, optional_generic)
        }

        #[builder(derive(Clone))]
        fn with_self<T: fmt::Debug>(
            &self,
            #[builder(transparent, with = Some)] required: Option<i32>,
            #[builder(transparent, with = Some, default = Some(()))] optional: Option<()>,
            #[builder(transparent, with = Some)] generic: Option<T>,
            #[builder(transparent, with = Some)] impl_trait: Option<impl fmt::Debug>,
            #[builder(transparent, with = Some, default = None)] optional_generic: Option<T>,
        ) -> impl fmt::Debug {
            let _ = self;
            (required, optional, generic, impl_trait, optional_generic)
        }
    }

    let builder = Sut::sut();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);
    let builder = builder.impl_trait("impl Trait");

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.call();

    assert_debug_eq(&sut, expect![[r#"(Some(99), Some(()), Some(2), Some("impl Trait"), Some(22))"#]]);

    let builder = Sut.with_self();

    let builder = builder.required(99);

    let _ignore = builder.clone().optional(());
    let builder = builder.maybe_optional(None);

    let builder = builder.generic(2);
    let builder = builder.impl_trait("impl Trait");

    let _ignore = builder.clone().optional_generic(21);
    let builder = builder.maybe_optional_generic(Some(22));

    let sut = builder.call();

    assert_debug_eq(&sut, expect![[r#"(Some(99), Some(()), Some(2), Some("impl Trait"), Some(22))"#]]);
}
