use serde::{Deserialize, Serialize};
use wae_config::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    port: u16,
    debug: bool,
}

#[test]
fn test_default_values() {
    let defaults = TestConfig { name: "default".to_string(), port: 8080, debug: false };

    let config: TestConfig = ConfigLoader::new().with_defaults(&defaults).extract().unwrap();

    assert_eq!(config, defaults);
}
