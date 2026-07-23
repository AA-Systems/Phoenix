use order_engine::queries::answer_query::{answer_query, balances_for_user};
use types::query::{EngineQuery, EngineReply};
use uuid::Uuid;

use crate::common::{USDC_AVAILABLE, fixture};

#[test]
fn balances_for_user_maps_assets() {
    let fx = fixture();
    let balances = balances_for_user(&fx.state, fx.user_id);
    assert!(
        balances
            .iter()
            .any(|b| b.symbol == "USDC" && b.available == USDC_AVAILABLE)
    );
    assert!(balances.iter().any(|b| b.symbol == "SOL"));
}

#[test]
fn answer_get_balances_query() {
    let fx = fixture();
    let request_id = Uuid::new_v4();
    let reply = answer_query(
        &fx.state,
        EngineQuery::GetBalances {
            request_id,
            user_id: fx.user_id,
        },
    );

    match reply {
        EngineReply::GetBalances {
            request_id: replied,
            balances,
        } => {
            assert_eq!(replied, request_id);
            assert!(!balances.is_empty());
        }
    }
}
