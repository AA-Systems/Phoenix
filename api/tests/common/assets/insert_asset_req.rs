use axum::{
    body::Body,
    http::{Request, header},
};
use serde_json::json;

pub fn insert_asset_req(
    uri: &str,
    symbol: &str,
    name: &str,
    decimal: u32,
    authed: bool,
) -> Request<Body> {
    let insert_body = Body::from(
        json!({
            "symbol": symbol,
            "name": name,
            "decimals": decimal
        })
        .to_string(),
    );

    let mut builder = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");

    if authed {
        builder = builder.header(header::AUTHORIZATION, "Bearer test-token")
    }

    builder.body(insert_body).unwrap()
}
