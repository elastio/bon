use crate::prelude::*;
use core::future::ready;

#[tokio::test]
async fn into_future_basic() {
    #[builder(derive(IntoFuture(Box)))]
    async fn simple_async_fn(value: u32) -> u32 {
        ready(value * 2).await
    }

    // Test direct call.
    let result = simple_async_fn().value(21).call().await;
    assert_eq!(result, 42);

    // Test using IntoFuture with await.
    let result = simple_async_fn().value(21).await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn into_future_non_send() {
    #[builder(derive(IntoFuture(Box, ?Send)))]
    async fn non_send_async_fn(value: u32) -> u32 {
        // This future can be !Send.
        ready(value * 2).await
    }

    // Test with non-Send future.
    let result = non_send_async_fn().value(21).await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn into_future_with_result() {
    #[builder(derive(IntoFuture(Box)))]
    async fn async_with_result(value: u32) -> Result<u32, &'static str> {
        ready(if value > 0 {
            Ok(value * 2)
        } else {
            Err("Value must be positive")
        })
        .await
    }

    // Test successful case.
    let result = async_with_result().value(21).await;
    assert_eq!(result.unwrap(), 42);

    // Test error case.
    let result = async_with_result().value(0).await;
    result.unwrap_err();
}

#[tokio::test]
async fn into_future_with_impl() {
    struct Calculator;

    #[bon]
    impl Calculator {
        #[builder]
        #[builder(derive(IntoFuture(Box)))]
        async fn multiply(a: u32, b: u32) -> u32 {
            ready(a * b).await
        }
    }

    // Test using IntoFuture on impl method.
    let result = Calculator::multiply().a(6).b(7).await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn into_future_with_optional() {
    #[builder(derive(IntoFuture(Box)))]
    async fn optional_param(#[builder(default = 100)] value: u32) -> u32 {
        ready(value).await
    }

    // Test with value.
    let result = optional_param().value(42).await;
    assert_eq!(result, 42);

    // Test without value (using default).
    let result = optional_param().await;
    assert_eq!(result, 100);
}

#[tokio::test]
async fn references_in_params() {
    struct Dummy;

    #[builder(derive(IntoFuture(Box)))]
    async fn sut<'named1, 'named2>(
        _x1: &Dummy,
        _x2: &Dummy,
        x3: &'named1 Dummy,
        x4: &'named2 Dummy,
    ) -> &'named2 Dummy {
        let _: &'named1 Dummy = x3;
        ready(x4).await
    }

    // Store the dummy struct in local variables to make sure no `'static`
    // lifetime promotion happens
    let local_x1 = Dummy;
    let local_x2 = Dummy;
    let local_x3 = Dummy;
    let local_x4 = Dummy;

    let _: &Dummy = sut()
        .x1(&local_x1)
        .x2(&local_x2)
        .x3(&local_x3)
        .x4(&local_x4)
        .await;
}

#[tokio::test]
async fn anon_lifetime_in_return_type() {
    struct Dummy;

    #[builder(derive(IntoFuture(Box)))]
    async fn sut(x1: &Dummy) -> &Dummy {
        ready(x1).await
    }

    // Store the dummy struct in local variables to make sure no `'static`
    // lifetime promotion happens
    let local_x1 = Dummy;

    let _: &Dummy = sut().x1(&local_x1).await;
}
