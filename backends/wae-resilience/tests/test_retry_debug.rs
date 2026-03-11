use std::time::Duration;
use wae_resilience::*;

#[tokio::test]
async fn debug_retry_logic() {
    let config = RetryConfig::new(2, RetryPolicy::fixed(Duration::from_millis(10)));
    let mut attempts = 0;
    let result: Result<i32, _> = retry_async(&config, |ctx| {
        attempts += 1;
        println!("Attempt: {}, ctx.attempt: {}", attempts, ctx.attempt);
        async { Err::<i32, &str>("persistent error") }
    })
    .await;
    assert!(result.is_err());
    println!("Final attempts: {}", attempts);
}
