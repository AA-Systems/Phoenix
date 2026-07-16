use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::assets::Asset;

#[derive(Serialize)]
pub struct InsertAssetResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub asset: Asset,
}

impl InsertAssetResponse {
    pub fn created(asset: Asset) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            asset,
        }
    }
}

impl IntoResponse for InsertAssetResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Deserialize)]
pub struct InsertAssetBody {
    pub asset: Asset,
}
