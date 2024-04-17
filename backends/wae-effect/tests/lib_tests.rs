use wae_effect::*;

#[derive(Debug, Clone, serde::Deserialize)]
struct TestConfig {
    name: String,
}

#[test]
fn test_dependencies() {
    let mut deps = Dependencies::new();
    deps.register("test", 42i32);
    assert_eq!(deps.get::<i32>("test").unwrap(), 42);
}

#[test]
fn test_algebraic_effect() {
    let deps = AlgebraicEffect::new().with("config", TestConfig { name: "test".to_string() }).build();

    let config: TestConfig = deps.get("config").unwrap();
    assert_eq!(config.name, "test");
}

#[test]
fn test_algebraic_effect_simple() {
    let deps = AlgebraicEffect::new().with("answer", 42i32).build();
    assert_eq!(deps.get::<i32>("answer").unwrap(), 42);
}
