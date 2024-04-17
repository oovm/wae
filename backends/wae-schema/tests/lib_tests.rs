use wae_schema::*;

#[test]
fn test_string_schema() {
    let schema = Schema::string().description("用户名").min_length(1).max_length(100);

    assert_eq!(schema.schema_type, SchemaType::String);
    assert_eq!(schema.min_length, Some(1));
    assert_eq!(schema.max_length, Some(100));
}

#[test]
fn test_object_schema() {
    let schema = Schema::object()
        .title("User")
        .property("id", Schema::integer())
        .property("name", Schema::string())
        .required(vec!["id", "name"]);

    assert_eq!(schema.schema_type, SchemaType::Object);
    assert!(schema.properties.is_some());
}

#[test]
fn test_openapi_doc() {
    let doc = OpenApiDoc::new("Test API", "1.0.0")
        .description("A test API")
        .server("https://api.example.com", Some("Production".to_string()));

    assert_eq!(doc.openapi, "3.1.0");
    assert_eq!(doc.info.title, "Test API");
}
