use crate::prelude::*;

#[allow(clippy::single_component_path_imports)]
use bon;

#[test]
fn test_struct() {
    {
        #[derive(Builder)]
        #[builder(
            crate = crate::builder::attr_crate::bon,
            derive(Debug, Clone)
        )]
        struct Sut {
            _a: u32,
            _b: u32,
        }

        let _ = Sut::builder().a(1).b(2).build();
    }
    {
        macro_rules! in_macro {
            () => {
                #[derive(Builder)]
                #[builder(crate = $crate::builder::attr_crate::bon, derive(Debug, Clone))]
                struct Sut {
                    _a: u32,
                    _b: u32,
                }
            };
        }
        in_macro!();

        let _ = Sut::builder().a(1).b(2).build();
    }
    {
        #[derive(Builder)]
        #[builder(
            crate = ::bon,
            derive(Debug, Clone)
        )]
        struct Sut {
            _a: u32,
            _b: u32,
        }

        let _ = Sut::builder().a(1).b(2).build();
    }
}

#[test]
fn test_free_fn() {
    {
        #[builder(
            crate = crate::builder::attr_crate::bon,
            derive(Debug, Clone)
        )]
        fn sut(_a: u32, _b: u32) {}

        sut().a(1).b(2).call();
    }
    {
        macro_rules! in_macro {
            () => {
                #[builder(crate = $crate::builder::attr_crate::bon, derive(Debug, Clone))]
                fn sut(_a: u32, _b: u32) {}
            };
        }
        in_macro!();

        sut().a(1).b(2).call();
    }
    {
        #[builder(
            crate = ::bon,
            derive(Debug, Clone)
        )]
        fn sut(_a: u32, _b: u32) {}

        sut().a(1).b(2).call();
    }
}

#[test]
fn test_assoc_method() {
    {
        struct Sut;

        #[bon(crate = crate::builder::attr_crate::bon)]
        impl Sut {
            #[builder(derive(Debug, Clone))]
            fn sut(_a: u32, _b: u32) {}
        }

        Sut::sut().a(1).b(2).call();
    }
    {
        macro_rules! in_macro {
            () => {
                struct Sut;

                #[bon(crate = $crate::builder::attr_crate::bon)]
                impl Sut {
                    #[builder(derive(Debug, Clone))]
                    fn sut(_a: u32, _b: u32) {}
                }
            };
        }
        in_macro!();

        Sut::sut().a(1).b(2).call();
    }
    {
        struct Sut;

        #[bon(crate = ::bon)]
        impl Sut {
            #[builder(derive(Debug, Clone))]
            fn sut(_a: u32, _b: u32) {}
        }

        Sut::sut().a(1).b(2).call();
    }
}
