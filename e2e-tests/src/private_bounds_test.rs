// This is extremely perplexing, but by wrapping the struct in a macro, the lint
// for `private_bounds` is triggered if code generated by `bon` violates it. Without
// it no lint is triggered.
macro_rules! test {
    () => {
        #[::bon::builder]
        pub struct PrivateBoundsTrigger {
            _field: u32,
        }
    };
}

test!();
