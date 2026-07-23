use sqlx::PgPool;
use tracing::info;

use super::OrderEngineState;

pub async fn load_from_db(pool: &PgPool) -> Result<OrderEngineState, sqlx::Error> {
    let assets = db::assets::get::list_all(pool).await?;
    let markets = db::markets::get::list_all(pool).await?;
    let balance_rows = db::balances::list_all::list_all(pool).await?;

    let mut state = OrderEngineState::new();
    state.assets = assets;
    state.markets = markets;
    state.balances = balance_rows
        .into_iter()
        .map(|balance| ((balance.user_id, balance.asset_id), balance))
        .collect();

    // Open orders / books / stream cursor come from a full snapshot later.
    info!(
        assets = state.assets.len(),
        markets = state.markets.len(),
        balances = state.balances.len(),
        "loaded engine state from database"
    );

    Ok(state)
}
