use bon::Builder;

#[test]
fn test_simple_syntax() {
    #[derive(Builder)]
    #[builder(generics(setters = "conv_{}"))]
    struct Sut<A, B> {
        x1: u32,
        x2: A,
        x3: B,
    }

    use sut_builder::{IsUnset, SetX2, SetX3, State};

    impl<A1, B1, S: State> SutBuilder<A1, B1, S> {
        fn x2_and_x3<A2, B2>(self, x2: A2, x3: B2) -> SutBuilder<A2, B2, SetX3<SetX2<S>>>
        where
            S::X2: IsUnset,
            S::X3: IsUnset,
        {
            self.conv_a().x2(x2).conv_b().x3(x3)
        }
    }

    // Start with () types, then convert to the actual types
    let result = Sut::<(), ()>::builder()
        .x1(42)
        .x2_and_x3("hello", [1, 2, 3])
        .build();

    assert_eq!(result.x1, 42);
    assert_eq!(result.x2, "hello");
    assert_eq!(result.x3, [1, 2, 3]);
}

#[test]
fn test_complex_syntax_with_name() {
    #[derive(Builder)]
    #[builder(generics(setters(name = "with_{}")))]
    struct Sut<T> {
        value: T,
    }

    impl<T1, S: sut_builder::State> SutBuilder<T1, S> {
        fn convert_and_set<T2>(self, value: T2) -> SutBuilder<T2, sut_builder::SetValue<S>>
        where
            S::Value: sut_builder::IsUnset,
        {
            self.with_t().value(value)
        }
    }

    let result = Sut::<()>::builder().convert_and_set(42).build();
    assert_eq!(result.value, 42);
}

#[test]
fn test_complex_syntax_with_vis() {
    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}", vis = "pub(self)")))]
    struct Sut<T> {
        value: T,
    }

    impl<T1, S: sut_builder::State> SutBuilder<T1, S> {
        fn convert_and_set<T2>(self, value: T2) -> SutBuilder<T2, sut_builder::SetValue<S>>
        where
            S::Value: sut_builder::IsUnset,
        {
            self.conv_t().value(value)
        }
    }

    let result = Sut::<()>::builder().convert_and_set(100).build();
    assert_eq!(result.value, 100);
}

#[test]
fn test_complex_syntax_with_docs() {
    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}", doc {
        /// Custom documentation for generic conversion.
    })))]
    struct Sut<T> {
        value: T,
    }

    let result = Sut::<()>::builder().conv_t::<u32>().value(42).build();
    assert_eq!(result.value, 42);
}

#[test]
fn test_with_trait_bounds() {
    trait MyTrait {
        type Assoc;
    }

    impl MyTrait for () {
        type Assoc = u32;
    }

    impl MyTrait for bool {
        type Assoc = u64;
    }

    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}")))]
    struct AssocTypeField<T: MyTrait> {
        value: T::Assoc,
    }

    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}")))]
    struct QualifiedPathField<T: MyTrait> {
        value: <T as MyTrait>::Assoc,
    }

    let result1 = AssocTypeField::<()>::builder()
        .conv_t::<bool>()
        .value(42u64)
        .build();
    assert_eq!(result1.value, 42u64);

    let result2 = QualifiedPathField::<()>::builder()
        .conv_t::<bool>()
        .value(99u64)
        .build();
    assert_eq!(result2.value, 99u64);
}

#[test]
fn test_with_trait_bounds_false_friend() {
    // The associated type is also called T, but should not be replaced by the generic <T>

    trait MyTrait {
        type T;
    }

    impl MyTrait for () {
        type T = u32;
    }

    impl MyTrait for bool {
        type T = u64;
    }

    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}")))]
    struct AssocTypeField<T: MyTrait> {
        value: T::T,
    }

    #[derive(Builder)]
    #[builder(generics(setters(name = "conv_{}")))]
    struct QualifiedPathField<T: MyTrait> {
        value: <T as MyTrait>::T,
    }

    let result1 = AssocTypeField::<()>::builder()
        .conv_t::<bool>()
        .value(42u64)
        .build();
    assert_eq!(result1.value, 42u64);

    let result2 = QualifiedPathField::<()>::builder()
        .conv_t::<bool>()
        .value(99u64)
        .build();
    assert_eq!(result2.value, 99u64);
}
