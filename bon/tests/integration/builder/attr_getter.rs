use crate::prelude::*;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Debug, Clone))]
    #[allow(dead_code)]

    struct Sut<T> {
        #[builder(getter, start_fn)]
        x1: u32,

        #[builder(getter(name = x2_with_custom_name))]
        x2: String,

        #[builder(getter(vis = "pub(crate)", doc {
            /// Docs on the getter
        }))]
        x3: u32,

        #[builder(into, getter(name = x5, vis = "pub(crate)", doc {
            /// The name is a lie
        }))]
        x4_but_its_actually_5: String,

        not_a_getter: u32,

        #[builder(getter)]
        generic_option_getter: Option<T>,

        x6: (),
    }

    #[allow(clippy::redundant_clone)]
    let sut = Sut::<()>::builder(0u32).clone();

    let actual = sut.x2("2".to_owned()).x3(3);

    let x3 = actual.get_x3();
    assert_eq!(x3, &3);

    let actual = actual.x4_but_its_actually_5("4".to_owned());
    let x5 = actual.x5();
    assert_eq!(x5, "4");

    let actual = actual.not_a_getter(5).x6(());

    assert_debug_eq(
        &actual,
        expect![[r#"
            SutBuilder {
                x1: 0,
                x2: "2",
                x3: 3,
                x4_but_its_actually_5: "4",
                not_a_getter: 5,
                x6: (),
            }"#]],
    );
}
