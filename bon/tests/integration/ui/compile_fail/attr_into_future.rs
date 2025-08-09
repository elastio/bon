use bon::{bon, builder, Builder};
use core::future::IntoFuture;

// IntoFuture can only be used with async functions
#[builder(derive(IntoFuture(Box)))]
fn sync_function() -> u32 {
    42
}

// IntoFuture is not supported for unsafe functions
#[builder(derive(IntoFuture(Box)))]
async unsafe fn unsafe_async_function() -> u32 {
    42
}

// IntoFuture is incompatible with finish_fn members
#[builder(derive(IntoFuture(Box)))]
async fn with_finish_fn(#[builder(finish_fn)] value: u32) -> u32 {
    value
}

// IntoFuture requires Box argument
#[builder(derive(IntoFuture))]
async fn missing_box_arg() -> u32 {
    42
}

// Only Box is supported as future container
#[builder(derive(IntoFuture(Arc)))]
async fn wrong_container() -> u32 {
    42
}

// Wrong syntax for ?Send
#[builder(derive(IntoFuture(Box, Send)))]
async fn wrong_send_syntax() -> u32 {
    42
}

// Cannot be used on structs
#[derive(Builder)]
#[builder(derive(IntoFuture(Box)))]
struct AsyncConfig {
    value: u32,
}

fn _non_send() {
    struct Sut;

    fn assert_send(_: &dyn Send) {}

    #[bon]
    impl Sut {
        #[builder(derive(IntoFuture(Box, ?Send)))]
        async fn sut(&self, value: u32) -> u32 {
            value * 2
        }
    }

    assert_send(&Sut.sut().value(21).into_future());

    #[builder(derive(IntoFuture(Box, ?Send)))]
    async fn sut(value: u32) -> u32 {
        value * 2
    }

    assert_send(&sut().value(21).into_future());
}

fn main() {}
