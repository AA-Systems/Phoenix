use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;

pub fn cancel_order_req(access_token: Option<&str>, order_id: &str) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/orders/cancel")
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(token) = access_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    builder
        .body(Body::from(
            json!({
                "order_id": order_id
            })
            .to_string(),
        ))
        .unwrap()
}
