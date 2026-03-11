use wae_schema::openapi::{builder::*, *};

#[test]
fn test_basic_builder() {
    let doc = OpenApiBuilder::new().title("Test API").version("1.0.0").description("This is a test API").build();

    assert_eq!(doc.info.title, "Test API");
    assert_eq!(doc.info.version, "1.0.0");
    assert_eq!(doc.info.description, Some("This is a test API".to_string()));
}

#[test]
fn test_with_title_and_version() {
    let doc = OpenApiBuilder::with_title_and_version("My API", "2.0.0").build();

    assert_eq!(doc.info.title, "My API");
    assert_eq!(doc.info.version, "2.0.0");
}

#[test]
fn test_add_path() {
    let path_item = PathItem::new();
    let doc = OpenApiBuilder::new().path("/users", path_item).build();

    assert!(doc.paths.contains_key("/users"));
}

#[test]
fn test_add_schema() {
    let schema = Schema::object();
    let doc = OpenApiBuilder::new().schema("User", schema).build();

    let components = doc.components.as_ref().unwrap();
    assert!(components.schemas.contains_key("User"));
}
