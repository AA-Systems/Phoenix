use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;

pub fn register_user_req(name: &str, email: &str, password: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "name": name,
                "email": email,
                "password": password
            })
            .to_string(),
        ))
        .unwrap()
}

pub fn login_user_req(email: &str, password: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/v1/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": email,
                "password": password
            })
            .to_string(),
        ))
        .unwrap()
}

pub fn unique_email() -> String {
    format!("user-{}@example.com", uuid::Uuid::new_v4())
}
