use bon::Builder;

#[derive(Builder)]
struct InvalidOnRequiredMember {
    #[builder(transparent)]
    member: i32,
}

#[derive(Builder)]
struct InvalidOnStartFnMember {
    #[builder(start_fn, transparent)]
    member: Option<i32>,
}

#[derive(Builder)]
struct InvalidOnFnMember {
    #[builder(finish_fn, transparent)]
    member: Option<i32>,
}

#[derive(Builder)]
struct InvalidOnSkippedMember {
    #[builder(skip, transparent)]
    member: Option<i32>,
}

#[derive(Builder)]
struct Valid {
    #[builder(transparent)]
    member: Option<u32>,
}

fn main() {
    // Make sure there is no `maybe_` setter generated
    let _ = Valid::builder().maybe_member(Some(42));

    // Another way to get transparency
    {
        type OpaqueOption<T> = Option<T>;

        #[derive(Builder)]
        struct Sut {
            arg1: OpaqueOption<u32>,
        }

        // Should not be allowed `OpaqueOption` is required
        let _ = Sut::builder().build();
    }
}
