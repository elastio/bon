use crate::prelude::*;
use core::panic::Location;

#[test]
fn track_caller_fn() {
    #[builder]
    #[track_caller]
    fn dont_brick(_x: u32) -> &'static Location<'static> {
        Location::caller()
    }

    let location = dont_brick().x(10).call();
    assert_debug_eq(
        location,
        expect![[r#"
        Location {
            file: "bon/tests/integration/builder/track_caller.rs",
            line: 12,
            col: 39,
        }"#]],
    );

    #[track_caller]
    #[builder]
    fn dont_brick_2(_x: u32) -> &'static Location<'static> {
        Location::caller()
    }

    let location = dont_brick_2().x(10).call();
    assert_debug_eq(
        location,
        expect![[r#"
            Location {
                file: "bon/tests/integration/builder/track_caller.rs",
                line: 29,
                col: 41,
            }"#]],
    );
}

#[test]
fn track_caller_impl_block() {
    struct Brick;
    struct Senti;
    #[bon]
    impl Senti {
        #[builder(finish_fn = yatta)]
        #[track_caller]
        fn new(brick: Brick) -> &'static Location<'static> {
            let Brick = brick;
            Location::caller()
        }
    }
    let yatta = Senti::builder().brick(Brick).yatta();
    assert_debug_eq(
        yatta,
        expect![[r#"
            Location {
                file: "bon/tests/integration/builder/track_caller.rs",
                line: 54,
                col: 47,
            }"#]],
    );
}
