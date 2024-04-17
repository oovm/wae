use hyper::StatusCode;
use wae_https::error::*;

#[test]
fn test_http_error_status_code() {
    assert_eq!(HttpError::BadRequest("test".to_string()).status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(HttpError::Unauthorized("test".to_string()).status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(HttpError::NotFound("test".to_string()).status_code(), StatusCode::NOT_FOUND);
    assert_eq!(HttpError::InternalServerError("test".to_string()).status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_http_error_error_code() {
    assert_eq!(HttpError::BadRequest("test".to_string()).error_code(), "BAD_REQUEST");
    assert_eq!(HttpError::Unauthorized("test".to_string()).error_code(), "UNAUTHORIZED");
    assert_eq!(HttpError::NotFound("test".to_string()).error_code(), "NOT_FOUND");
}

#[test]
fn test_error_response() {
    let error = HttpError::NotFound("资源不存在".to_string());
    let response = ErrorResponse::from_error(&error);

    assert!(!response.success);
    assert_eq!(response.code, "NOT_FOUND");
    assert_eq!(response.message, "资源不存在");
    assert!(response.details.is_none());
    assert!(response.trace_id.is_none());
}

#[test]
fn test_error_ext() {
    let result: Result<(), &str> = Err("test error");
    let http_result = result.bad_request();

    match http_result {
        Err(HttpError::BadRequest(msg)) => assert_eq!(msg, "test error"),
        _ => panic!("Expected BadRequest error"),
    }
}

#[test]
fn test_from_wae_error() {
    let wae_error = wae_types::WaeError::not_found("资源不存在");
    let http_error: HttpError = wae_error.into();

    assert!(matches!(http_error, HttpError::NotFound(_)));
}
