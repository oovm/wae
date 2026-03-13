
use wae_service::{ServiceRegistration, HealthCheckConfig};

#[test]
fn test_service_registration_default() {
    let reg = ServiceRegistration::default();
    assert_eq!(reg.name, "unnamed-service");
}

#[test]
fn test_health_check_config_default() {
    let config = HealthCheckConfig::default();
    assert_eq!(config.path, "/health");
}
