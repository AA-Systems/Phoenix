use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CancelOrderResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub command_id: Uuid,
    pub order_id: String,
}

impl CancelOrderResponse {
    pub fn accepted(command_id: Uuid, order_id: String) -> Self {
        Self {
            status_code: StatusCode::ACCEPTED,
            command_id,
            order_id,
        }
    }
}

impl IntoResponse for CancelOrderResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderBody {
    pub command_id: Uuid,
    pub order_id: String,
}
