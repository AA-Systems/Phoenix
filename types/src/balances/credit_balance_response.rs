use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreditBalanceResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub command_id: Uuid,
    pub user_id: Uuid,
    pub asset_symbol: String,
    pub amount: i64,
}

impl CreditBalanceResponse {
    pub fn accepted(command_id: Uuid, user_id: Uuid, asset_symbol: String, amount: i64) -> Self {
        Self {
            status_code: StatusCode::ACCEPTED,
            command_id,
            user_id,
            asset_symbol,
            amount,
        }
    }
}

impl IntoResponse for CreditBalanceResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreditBalanceBody {
    pub command_id: Uuid,
    pub user_id: Uuid,
    pub asset_symbol: String,
    pub amount: i64,
}
