use axum::{Extension, Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use db::assets::get::get_by_symbol;
use redis::AsyncCommands;
use types::balances::demo_credit::{DemoCreditItem, DemoCreditRequest, DemoCreditResponse};
use types::command::Command;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::{AppState, AssetQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
};

/// Fixed faucet pack for testing (atomic units).
const FAUCET_PACK: &[(&str, i64)] = &[
    ("USDC", 100_000_000_000),  // 100,000
    ("USDT", 50_000_000_000),   // 50,000
    ("BTC", 1_0000_0000),       // 1
    ("ETH", 10_0000_0000),      // 10
    ("SOL", 100_000_000_000),   // 100
    ("HYPE", 5_000_000_000),    // 5,000
    ("DOGE", 50_000_0000_0000), // 50,000
    ("LINK", 500_0000_0000),    // 500
    ("AVAX", 200_0000_0000),    // 200
    ("SUI", 1_000_000_000_000), // 1,000
];

pub async fn demo_credit(
    _: DynamicFixedWindowLimit<ClientIpUri, AssetQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(mut body): Json<DemoCreditRequest>,
) -> Result<Json<DemoCreditResponse>, (StatusCode, String)> {
    if let Some(symbol) = body.asset_symbol.as_mut() {
        *symbol = symbol.trim().to_uppercase();
    }
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let pack: Vec<(&str, i64)> = match body.asset_symbol.as_deref() {
        Some(symbol) => {
            let amount = FAUCET_PACK
                .iter()
                .find(|(sym, _)| *sym == symbol)
                .map(|(_, amount)| *amount)
                .ok_or_else(|| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("Asset {symbol} is not in the demo faucet"),
                    )
                })?;
            vec![(symbol, amount)]
        }
        None => FAUCET_PACK.to_vec(),
    };

    let mut credits = Vec::with_capacity(pack.len());
    let mut redis = app_state.redis.clone();

    for (symbol, amount) in pack {
        let asset = get_by_symbol(&app_state.pool, symbol)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => {
                    (StatusCode::BAD_REQUEST, format!("Asset {symbol} not found"))
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                ),
            })?;

        let command_id = Uuid::new_v4();
        let command = Command::CreditBalance {
            command_id,
            user_id: auth_user.user_id,
            asset_id: asset.id,
            amount,
        };

        let payload = serde_json::to_string(&command).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;

        redis
            .xadd::<_, _, _, _, String>(
                &app_state.engine_commands_stream,
                "*",
                &[("payload", payload.as_str())],
            )
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to enqueue credit command".to_string(),
                )
            })?;

        credits.push(DemoCreditItem {
            command_id,
            asset_symbol: symbol.to_string(),
            amount,
        });
    }

    Ok(Json(DemoCreditResponse { credits }))
}
