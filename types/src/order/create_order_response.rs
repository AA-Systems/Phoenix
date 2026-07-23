use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::order::OrderType;

#[derive(Serialize)]
pub struct CreateOrderResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub command_id: Uuid,
    pub market_symbol: String,
    pub order_type: OrderType,
    pub price: i64,
    pub quantity: i64,
}

impl CreateOrderResponse {
    pub fn accepted(
        command_id: Uuid,
        market_symbol: String,
        order_type: OrderType,
        price: i64,
        quantity: i64,
    ) -> Self {
        Self {
            status_code: StatusCode::ACCEPTED,
            command_id,
            market_symbol,
            order_type,
            price,
            quantity,
        }
    }
}

impl IntoResponse for CreateOrderResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderBody {
    pub command_id: Uuid,
    pub market_symbol: String,
    pub order_type: OrderType,
    pub price: i64,
    pub quantity: i64,
}
