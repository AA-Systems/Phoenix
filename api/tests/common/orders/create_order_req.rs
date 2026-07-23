use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;

pub fn create_order_req(
    access_token: Option<&str>,
    market_symbol: &str,
    order_type: &str,
    price: i64,
    quantity: i64,
) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/orders/create")
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(token) = access_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    builder
        .body(Body::from(
            json!({
                "market_symbol": market_symbol,
                "order_type": order_type,
                "price": price,
                "quantity": quantity
            })
            .to_string(),
        ))
        .unwrap()
}
