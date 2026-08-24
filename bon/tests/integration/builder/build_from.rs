use crate::prelude::*;

#[derive(Builder, Clone)]
#[builder(build_from, build_from_clone)]
struct Sut {
    name: String,
    age: u8,
}

#[test]
fn test_build_from_works() {
    let jon = Sut::builder().name("Jon".into()).age(25).build();
    let alice = Sut::builder().name("Alice".into()).build_from(jon);
    assert_eq!(alice.age, 25);
    assert_eq!(alice.name, "Alice");
}

#[test]
fn test_build_from_clone_works() {
    let jon = Sut::builder().name("Jon".into()).age(25).build();
    let alice = Sut::builder().name("Alice".into()).build_from_clone(&jon);
    assert_eq!(alice.age, 25);
    assert_eq!(alice.name, "Alice");
}

#[builder(build_from, build_from_clone)]
fn create_user(name: String, age: u8) -> Sut {
    Sut { name, age }
}

#[test]
fn test_function_build_from_works() {
    let jon = create_user().name("Jon".into()).age(25).call();
    let alice = create_user().name("Alice".into()).call_from(jon);
    assert_eq!(alice.age, 25);
    assert_eq!(alice.name, "Alice");
}

#[bon]
impl Sut {
    #[builder(build_from, build_from_clone)]
    fn from_parts(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

#[test]
fn test_method_build_from_clone_works() {
    let jon = Sut::from_parts().name("Jon".into()).age(25).call();
    let alice = Sut::from_parts().name("Alice".into()).call_from_clone(&jon);
    assert_eq!(alice.age, 25);
    assert_eq!(alice.name, "Alice");
}

#[test]
fn test_build_from_path_and_generics() {
    {
        pub(crate) mod models {
            #[derive(Debug, PartialEq)]
            pub(crate) struct Sut<T> {
                pub(crate) value: T,
                pub(crate) flag: bool,
            }
        }
        #[derive(Builder)]
        #[builder(build_from)]
        struct Source {
            value: String,
            flag: bool,
        }
        let src = Source::builder()
            .value("Veetah".to_string())
            .flag(true)
            .build();

        let actual = models::Sut::<String> {
            value: src.value,
            flag: src.flag,
        };
        assert_eq!(actual.value, "Veetah");
    }
}

#[test]
fn test_build_from_custom_vis_and_docs() {
    {
        pub(crate) mod external {
            use crate::prelude::*;

            #[derive(Builder, Clone)]
            #[builder(build_from)]
            pub(crate) struct Source {
                pub(crate) id: u32,
            }
        }
        let base = external::Source::builder().id(99).build();
        let actual = external::Source::builder().build_from(base);
        assert_eq!(actual.id, 99);
    }
}

#[test]
fn test_build_from_nested_path_and_generics_enforcement() {
    {
        pub(crate) mod complex_namespace {
            pub(crate) mod deeply_nested {
                pub(crate) struct TargetSut<T> {
                    pub(crate) data: String,
                    pub(crate) extra: T,
                }
            }
        }
        #[builder(build_from)]
        fn create_complex_target(
            data: String,
            extra: u32,
        ) -> complex_namespace::deeply_nested::TargetSut<u32> {
            complex_namespace::deeply_nested::TargetSut { data, extra }
        }
        let base = complex_namespace::deeply_nested::TargetSut {
            data: "BaseData".to_string(),
            extra: 100u32,
        };
        let actual = create_complex_target()
            .data("Veetah".to_string())
            .call_from(base);
        assert_eq!(actual.data, "Veetah");
        assert_eq!(actual.extra, 100);
    }
}
