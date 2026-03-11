use std::time::Duration;
use wae_testing::load_test::*;

#[test]
fn test_load_test_config() {
    let config = LoadTestConfig::new().concurrency(20).total_requests(5000).warmup_requests(200);

    assert_eq!(config.concurrency, 20);
    assert_eq!(config.total_requests, 5000);
    assert_eq!(config.warmup_requests, 200);
}

#[test]
fn test_request_result() {
    let success = RequestResult::success(Duration::from_millis(10));
    assert!(success.success);
    assert_eq!(success.latency, Duration::from_millis(10));
    assert!(success.error.is_none());

    let error = RequestResult::error(Duration::from_millis(5), "timeout".to_string());
    assert!(!error.success);
    assert_eq!(error.latency, Duration::from_millis(5));
    assert_eq!(error.error, Some("timeout".to_string()));
}

#[test]
fn test_sync_load_tester() {
    let config = LoadTestConfig::new().concurrency(2).total_requests(10).warmup_requests(0);

    let tester = SyncLoadTester::new(config);

    let result = tester.run(|| RequestResult::success(Duration::from_millis(1)));

    assert_eq!(result.total_requests, 10);
    assert_eq!(result.success_count, 10);
    assert_eq!(result.error_count, 0);
    assert_eq!(result.error_rate, 0.0);
}
