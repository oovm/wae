use std::collections::HashMap;
use wae_types::*;

#[test]
fn test_value_null() {
    let v = Value::null();
    assert!(v.is_null());
    assert_eq!(v.to_json_string(), "null");
}

#[test]
fn test_value_bool() {
    let v = Value::bool(true);
    assert!(v.is_bool());
    assert_eq!(v.as_bool(), Some(true));
}

#[test]
fn test_value_integer() {
    let v = Value::integer(42);
    assert!(v.is_integer());
    assert_eq!(v.as_integer(), Some(42));
}

#[test]
fn test_value_float() {
    let v = Value::float(1.5);
    assert!(v.is_float());
    assert_eq!(v.as_float(), Some(1.5));
}

#[test]
fn test_value_string() {
    let v = Value::string("hello");
    assert!(v.is_string());
    assert_eq!(v.as_str(), Some("hello"));
}

#[test]
fn test_value_array() {
    let v = Value::array(vec![Value::integer(1), Value::integer(2)]);
    assert!(v.is_array());
    assert_eq!(v.as_array().map(|a| a.len()), Some(2));
}

#[test]
fn test_value_object() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), Value::string("test"));
    let v = Value::object(map);
    assert!(v.is_object());
    assert_eq!(v.get("name").and_then(|v| v.as_str()), Some("test"));
}

#[test]
fn test_json_parse() {
    let v = Value::from_json_str("{\"name\":\"test\",\"age\":42}").unwrap();
    assert_eq!(v.get("name").and_then(|v| v.as_str()), Some("test"));
    assert_eq!(v.get("age").and_then(|v| v.as_integer()), Some(42));
}

#[test]
fn test_json_roundtrip() {
    let original = object! {
        "name" => "test",
        "count" => 42,
        "active" => true,
        "tags" => array!["a", "b", "c"],
    };
    let json = original.to_json_string();
    let parsed = Value::from_json_str(&json).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_object_macro() {
    let v = object! {
        "name" => "test",
        "count" => 42,
    };
    assert!(v.is_object());
    assert_eq!(v.get("name").and_then(|v| v.as_str()), Some("test"));
    assert_eq!(v.get("count").and_then(|v| v.as_integer()), Some(42));
}

#[test]
fn test_array_macro() {
    let v = array![1, 2, 3];
    assert!(v.is_array());
    assert_eq!(v.as_array().map(|a| a.len()), Some(3));
}
