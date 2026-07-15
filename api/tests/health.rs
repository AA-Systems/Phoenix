use api::{app_state::AppState, build_app};
use axum::http::{Request, StatusCode};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://admin:supersecretpassword@localhost:5433/cex_test")
        .expect("lazy pool");
    let state = AppState {
        pool,
        admin_api_token: "test-token".into(),
    };

    let app = build_app(state);
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();

    // Send the request directly to the router without network overhead
    let response = app.oneshot(request).await.unwrap();
    println!("{:?}", response);

    // Assert HTTP status code
    assert_eq!(response.status(), StatusCode::OK);
}
