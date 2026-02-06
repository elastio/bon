use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in various type positions
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct OptionField<T> {
    value: core::option::Option<T>,
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct TupleField<T> {
    value: (T, T),
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct ArrayField<T> {
    value: [T; 2],
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct RawPointerField<T> {
    value: *const T,
}

trait MyTrait {
    type Assoc;
}

impl MyTrait for () {
    type Assoc = u32;
}

impl MyTrait for bool {
    type Assoc = u64;
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct AssocTypeField<T: MyTrait> {
    value: T::Assoc,
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct QualifiedPathField<T: MyTrait> {
    value: <T as MyTrait>::Assoc,
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct FnPointerField<T> {
    value: fn(T) -> T,
}

fn main() {
    // Test Option<T> - can't change type after setting field
    OptionField::<()>::builder()
        .value(None)
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test (T, T) - can't change type after setting field
    TupleField::<()>::builder()
        .value(((), ()))
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test [T; N] - can't change type after setting field
    ArrayField::<()>::builder()
        .value([(), ()])
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test *const T - can't change type after setting field
    RawPointerField::<()>::builder()
        .value(core::ptr::null())
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test T::Assoc - can't change type after setting field
    AssocTypeField::<()>::builder()
        .value(42)
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test <T as Trait>::Assoc - can't change type after setting field
    QualifiedPathField::<()>::builder()
        .value(42)
        .conv_t::<bool>()
        .build(); //~ ERROR

    // Test fn(T) -> T - can't change type after setting field
    FnPointerField::<()>::builder()
        .value(|x| x)
        .conv_t::<bool>()
        .build(); //~ ERROR
}
