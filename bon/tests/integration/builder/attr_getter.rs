use crate::prelude::*;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Debug, Clone))]
    #[allow(dead_code)]
    struct Sut<T> {
        #[builder(start_fn)]
        x1: u32,

        #[builder(getter(name = x2_with_custom_name))]
        x2: &'static str,

        #[builder(getter(vis = "pub(crate)", doc {
            /// Docs on the getter
        }))]
        x3: u32,

        #[builder(into, getter(name = x5, vis = "pub(crate)", doc {
            /// The name is a lie
        }))]
        x4_but_its_actually_5: &'static str,

        not_a_getter: u32,

        #[builder(getter)]
        generic_option_getter: Option<T>,

        x6: (),

        #[builder(getter, default)]
        x7: u32,
    }

    #[allow(clippy::redundant_clone)]
    let sut = Sut::<()>::builder(0u32).clone();

    let actual = sut.x2("2").x3(3);

    let actual = actual.x4_but_its_actually_5("4");
    let x5 = actual.x5();
    assert_eq!(*x5, "4");

    let actual = actual.not_a_getter(5).x6(());

    let x2 = actual.x2_with_custom_name();
    assert_eq!(*x2, "2");

    let x3 = actual.get_x3();
    assert_eq!(x3, &3);

    let actual = actual.maybe_generic_option_getter(None);

    let gen_opt_get = actual.get_generic_option_getter();
    assert_eq!(gen_opt_get, None);

    let actual = actual.x7(7);
    assert_eq!(actual.get_x7(), Some(&7));

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
                x7: 7,
            }"#]],
    );
}
