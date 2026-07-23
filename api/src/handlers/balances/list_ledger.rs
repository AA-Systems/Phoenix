use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::balances::list_ledger;
use types::ledger_entries::LedgerEntryView;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 100;

pub async fn list_ledger_entries(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<LedgerEntryView>>, (StatusCode, String)> {
    let entries = list_ledger::list_by_user(
        &app_state.pool,
        auth_user.user_id,
        DEFAULT_LIMIT.min(MAX_LIMIT),
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(entries))
}
