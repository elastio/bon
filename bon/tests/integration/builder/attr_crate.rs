use crate::prelude::*;

#[test]
fn test_struct() {
    #[allow(clippy::single_component_path_imports)]
    use bon;

    #[derive(Builder)]
    #[builder(crate = self::bon)]
    struct Sut {}
}
