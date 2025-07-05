use crate::prelude::*;

#[derive(Builder, Clone)]
#[builder(build_from, build_from_clone)]
struct User {
    name: String,
    age: u8,
}

#[test]
fn test_build_from_works() {
    let jon = User::builder().name("Jon".into()).age(25).build();
    let alice = User::builder().name("Alice".into()).build_from(&jon);
    assert_eq!(alice.age, jon.age);
    assert_eq!(alice.name, "Alice");
}

#[test]
fn test_build_from_clone_works() {
    let jon = User::builder().name("Jon".into()).age(25).build();
    let alice = User::builder()
        .name("Alice".into())
        .build_from_clone(jon.clone());
    assert_eq!(alice.age, jon.age);
    assert_eq!(alice.name, "Alice");
}
