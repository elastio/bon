#![deny(warnings)]
use bon::{bon, builder, Builder};

fn main() {
    // Test #[must_use]
    {
        #[allow(dead_code)]
        #[derive(Builder)]
        struct Example {
            x: u32,
            y: u32,
        }

        #[bon]
        impl Example {
            #[builder]
            #[must_use]
            fn must_use() -> u32 {
                99
            }
        }

        #[builder]
        #[must_use]
        fn must_use() -> u32 {
            99
        }

        Example::builder();
        Example::must_use();
        must_use();

        Example::builder().x(1);
        Example::builder().x(1).y(2).build();

        Example::must_use().call();

        must_use().call();
        __orig_must_use();
    }
}
