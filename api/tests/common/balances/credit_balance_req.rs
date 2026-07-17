use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;
use uuid::Uuid;

use crate::common::ADMIN_TOKEN;

pub fn credit_balance_req(
    user_id: Uuid,
    asset_symbol: &str,
    amount: i64,
    authed: bool,
) -> Request<Body> {
    let body = Body::from(
        json!({
            "user_id": user_id,
            "asset_symbol": asset_symbol,
            "amount": amount
        })
        .to_string(),
    );

    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/balances/admin/credit")
        .header(header::CONTENT_TYPE, "application/json");

    if authed {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {ADMIN_TOKEN}"));
    }

    builder.body(body).unwrap()
}
