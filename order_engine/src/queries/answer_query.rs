use types::balances::AssetBalance;
use types::query::{EngineQuery, EngineReply};
use uuid::Uuid;

use crate::memory::OrderEngineState;

pub fn answer_query(state: &OrderEngineState, query: EngineQuery) -> EngineReply {
    match query {
        EngineQuery::GetBalances {
            request_id,
            user_id,
        } => EngineReply::GetBalances {
            request_id,
            balances: balances_for_user(state, user_id),
        },
    }
}

pub fn balances_for_user(state: &OrderEngineState, user_id: Uuid) -> Vec<AssetBalance> {
    let mut balances: Vec<AssetBalance> = state
        .balances
        .iter()
        .filter(|((uid, _), _)| *uid == user_id)
        .filter_map(|((_, asset_id), balance)| {
            let asset = state.assets.iter().find(|asset| asset.id == *asset_id)?;
            Some(AssetBalance {
                asset_id: *asset_id,
                symbol: asset.symbol.clone(),
                name: asset.name.clone(),
                decimals: asset.decimals,
                available: balance.available,
                locked: balance.locked,
                updated_at: balance.updated_at,
            })
        })
        .collect();

    balances.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    balances
}
