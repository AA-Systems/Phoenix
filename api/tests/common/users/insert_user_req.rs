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

pub fn refresh_token_req(refresh_cookie: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/v1/auth/refresh")
        .header(header::COOKIE, refresh_cookie)
        .body(Body::empty())
        .unwrap()
}

pub fn logout_req(refresh_cookie: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/v1/auth/logout")
        .header(header::COOKIE, refresh_cookie)
        .body(Body::empty())
        .unwrap()
}

pub fn me_req(access_token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method("GET").uri("/api/v1/auth/me");

    if let Some(token) = access_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    builder.body(Body::empty()).unwrap()
}

pub fn get_balances_req(access_token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/balances/get");

    if let Some(token) = access_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    builder.body(Body::empty()).unwrap()
}

pub fn get_ledger_req(access_token: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/balances/ledger");

    if let Some(token) = access_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    builder.body(Body::empty()).unwrap()
}

pub fn unique_email() -> String {
    format!("user-{}@example.com", uuid::Uuid::new_v4())
}
