// IntoFuture can only be used with async functions
#[bon::builder]
#[builder(derive(IntoFuture(Box)))]
fn sync_function() -> u32 {
    42
}

// IntoFuture is not supported for unsafe functions
#[bon::builder]
#[builder(derive(IntoFuture(Box)))]
async unsafe fn unsafe_async_function() -> u32 {
    42
}

// IntoFuture is incompatible with finish_fn members
#[bon::builder]
#[builder(derive(IntoFuture(Box)))]
async fn with_finish_fn(
    #[builder(finish_fn)] value: u32
) -> u32 {
    value
}

// IntoFuture requires Box argument
#[bon::builder]
#[builder(derive(IntoFuture))]
async fn missing_box_arg() -> u32 {
    42
}

// Only Box is supported as future container
#[bon::builder]
#[builder(derive(IntoFuture(Arc)))]
async fn wrong_container() -> u32 {
    42
}

// Wrong syntax for ?Send
#[bon::builder]
#[builder(derive(IntoFuture(Box, Send)))]
async fn wrong_send_syntax() -> u32 {
    42
}

use bon::Builder;

// Cannot be used on structs
#[derive(Builder)]
#[builder(derive(IntoFuture(Box)))]
struct AsyncConfig {
    value: u32,
}

fn main() {}