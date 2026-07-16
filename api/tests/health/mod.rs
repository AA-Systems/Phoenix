use api::build_app;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use crate::common::test_state;

#[tokio::test]
async fn test_health_endpoint() {
    let app = build_app(test_state().await);
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
