//! WAE 集成测试主入口
//!
//! 这个包包含了 WAE 框架的集成测试，用于测试多个模块之间的协作。

use std::env;
use std::fs;
use wae_config::{ConfigLoader, load_config};
use wae_types::WaeResult;
use wae_testing::{create_test_env, RandomString, RandomNumber};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct AppConfig {
    app_name: String,
    http_port: u16,
    database_url: String,
    debug_mode: bool,
}

fn set_env_var(key: &str, value: &str) {
    unsafe {
        env::set_var(key, value);
    }
}

fn remove_env_var(key: &str) {
    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_config_with_types_and_testing() {
    let _test_env = create_test_env();
    
    let random_name = RandomString::new().length(10).generate().unwrap();
    let random_port = RandomNumber::<i32>::new(8000, 9000).generate().unwrap() as u16;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let toml_path = temp_dir.path().join("app.toml");
    
    let toml_content = format!(
        r#"
app_name = "{random_name}"
http_port = {random_port}
database_url = "postgres://localhost:5432/wae_test"
debug_mode = true
"#
    );
    
    fs::write(&toml_path, toml_content).unwrap();
    
    let config: AppConfig = ConfigLoader::new()
        .with_toml(toml_path.to_str().unwrap())
        .extract()
        .unwrap();
    
    assert_eq!(config.app_name, random_name);
    assert_eq!(config.http_port, random_port);
    assert_eq!(config.database_url, "postgres://localhost:5432/wae_test");
    assert!(config.debug_mode);
}

#[test]
fn test_config_load_with_error_handling() {
    let loader = ConfigLoader::new();
    let result: WaeResult<AppConfig> = loader.extract();
    
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    assert!(!format!("{:?}", err).is_empty());
}

#[tokio::test]
async fn test_async_config_loading() {
    let temp_dir = tempfile::tempdir().unwrap();
    let toml_path = temp_dir.path().join("async_app.toml");
    
    let toml_content = r#"
app_name = "async-test"
http_port = 8888
database_url = "redis://localhost:6379"
debug_mode = false
"#;
    
    fs::write(&toml_path, toml_content).unwrap();
    
    let result: WaeResult<AppConfig> = tokio::task::spawn_blocking(move || {
        load_config(toml_path.to_str().unwrap(), "ASYNC_")
    }).await.unwrap();
    
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.app_name, "async-test");
}

#[test]
fn test_multi_source_config_loading() {
    let temp_dir = tempfile::tempdir().unwrap();
    let toml_path = temp_dir.path().join("multi_config.toml");
    
    let toml_content = r#"
app_name = "toml-app"
http_port = 8080
database_url = "postgres://localhost:5432/wae"
debug_mode = false
"#;
    
    fs::write(&toml_path, toml_content).unwrap();
    
    set_env_var("MULTI_APP_NAME", "env-app");
    set_env_var("MULTI_HTTP_PORT", "9090");
    
    let config: AppConfig = ConfigLoader::new()
        .with_toml(toml_path.to_str().unwrap())
        .with_env("MULTI_")
        .extract()
        .unwrap();
    
    assert_eq!(config.app_name, "env-app");
    assert_eq!(config.http_port, 9090);
    assert_eq!(config.database_url, "postgres://localhost:5432/wae");
    assert!(!config.debug_mode);
    
    remove_env_var("MULTI_APP_NAME");
    remove_env_var("MULTI_HTTP_PORT");
}

#[test]
fn test_config_with_defaults_and_env() {
    let defaults = AppConfig {
        app_name: "default-app".to_string(),
        http_port: 3000,
        database_url: "sqlite::memory:".to_string(),
        debug_mode: true,
    };
    
    set_env_var("DEFAULT_HTTP_PORT", "4000");
    
    let config: AppConfig = ConfigLoader::new()
        .with_defaults(&defaults)
        .with_env("DEFAULT_")
        .extract()
        .unwrap();
    
    assert_eq!(config.app_name, "default-app");
    assert_eq!(config.http_port, 4000);
    assert_eq!(config.database_url, "sqlite::memory:");
    assert!(config.debug_mode);
    
    remove_env_var("DEFAULT_HTTP_PORT");
}
