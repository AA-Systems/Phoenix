use chrono::Utc;
use order_engine::memory::OrderEngineState;
use types::{
    assets::{Asset, AssetStatus},
    balances::Balance,
    markets::{Market, MarketStatus},
};
use uuid::Uuid;

pub const MARKET: &str = "SOL_USDC";
// 150 USDC (6dp) × 0.1 SOL (9dp) → notional 15 USDC
pub const PRICE: i64 = 150_000_000;
pub const QTY: i64 = 100_000_000;
pub const BUY_NOTIONAL: i64 = 15_000_000;
pub const USDC_AVAILABLE: i64 = 100_000_000;
pub const SOL_AVAILABLE: i64 = 1_000_000_000;

pub struct Fixture {
    pub state: OrderEngineState,
    pub user_id: Uuid,
    pub other_user_id: Uuid,
    pub sol_id: Uuid,
    pub usdc_id: Uuid,
}

pub fn fixture() -> Fixture {
    let now = Utc::now();
    let user_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let sol_id = Uuid::new_v4();
    let usdc_id = Uuid::new_v4();

    let mut state = OrderEngineState::new();
    state.assets = vec![
        Asset {
            id: sol_id,
            symbol: "SOL".into(),
            name: "Solana".into(),
            decimals: 9,
            status: AssetStatus::Active,
            created_at: now,
        },
        Asset {
            id: usdc_id,
            symbol: "USDC".into(),
            name: "USD Coin".into(),
            decimals: 6,
            status: AssetStatus::Active,
            created_at: now,
        },
    ];
    state.markets = vec![Market {
        id: Uuid::new_v4(),
        symbol: MARKET.into(),
        name: "SOL / USDC".into(),
        base_asset_id: sol_id,
        quote_asset_id: usdc_id,
        status: MarketStatus::Trading,
        price_tick_size: 10_000,
        quantity_step_size: 1_000_000,
        min_order_quantity: 10_000_000,
        min_order_notional: 5_000_000,
        created_at: now,
    }];

    for (uid, available_sol, available_usdc) in [
        (user_id, SOL_AVAILABLE, USDC_AVAILABLE),
        (other_user_id, SOL_AVAILABLE, USDC_AVAILABLE),
    ] {
        state.balances.insert(
            (uid, usdc_id),
            Balance {
                user_id: uid,
                asset_id: usdc_id,
                available: available_usdc,
                locked: 0,
                updated_at: now,
            },
        );
        state.balances.insert(
            (uid, sol_id),
            Balance {
                user_id: uid,
                asset_id: sol_id,
                available: available_sol,
                locked: 0,
                updated_at: now,
            },
        );
    }

    Fixture {
        state,
        user_id,
        other_user_id,
        sol_id,
        usdc_id,
    }
}
