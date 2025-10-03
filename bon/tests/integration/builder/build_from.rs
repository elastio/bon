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
