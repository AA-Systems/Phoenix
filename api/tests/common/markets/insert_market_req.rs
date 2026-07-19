use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;

pub fn insert_market_req(
    symbol: &str,
    name: &str,
    base_asset_symbol: &str,
    quote_asset_symbol: &str,
    authed: bool,
) -> Request<Body> {
    let insert_body = Body::from(
        json!({
            "symbol": symbol,
            "name": name,
            "base_asset_symbol": base_asset_symbol,
            "quote_asset_symbol": quote_asset_symbol,
            "price_tick_size": 10_000,
            "quantity_step_size": 1_000_000,
            "min_order_quantity": 10_000_000,
            "min_order_notional": 5_000_000
        })
        .to_string(),
    );

    let mut builder = Request::builder()
        .method("POST")
        .uri("/api/v1/markets/admin/insert")
        .header(header::CONTENT_TYPE, "application/json");

    if authed {
        builder = builder.header(header::AUTHORIZATION, "Bearer test-token");
    }

    builder.body(insert_body).unwrap()
}
