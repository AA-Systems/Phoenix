use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::markets::Market;

#[derive(Serialize)]
pub struct InsertMarketResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub market: Market,
}

impl InsertMarketResponse {
    pub fn created(market: Market) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            market,
        }
    }
}

impl IntoResponse for InsertMarketResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Deserialize)]
pub struct InsertMarketBody {
    pub market: Market,
}
