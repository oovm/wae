
use wae_cache::{memory_cache, CacheConfig, EvictionPolicy};

#[tokio::test]
async fn test_cache_config_default() {
    let config = CacheConfig::default();
    assert!(config.key_prefix.is_empty());
    assert_eq!(config.eviction_policy, EvictionPolicy::None);
}

#[test]
fn test_eviction_policy_default() {
    let policy = EvictionPolicy::default();
    assert_eq!(policy, EvictionPolicy::None);
}
