use std::time::Duration;
use wae_testing::benchmark::*;

#[test]
fn test_simple_timer() {
    let timer = SimpleTimer::start();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = timer.elapsed();
    assert!(elapsed >= Duration::from_millis(10));
}

#[test]
fn test_performance_stats() {
    let mut stats = PerformanceStats::new();
    stats.add_sample(Duration::from_millis(10));
    stats.add_sample(Duration::from_millis(20));
    stats.add_sample(Duration::from_millis(30));

    assert_eq!(stats.sample_count(), 3);
    assert_eq!(stats.mean(), Some(Duration::from_millis(20)));
    assert_eq!(stats.min(), Some(Duration::from_millis(10)));
    assert_eq!(stats.max(), Some(Duration::from_millis(30)));
    assert_eq!(stats.median(), Some(Duration::from_millis(20)));
}
