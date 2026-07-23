use order_engine::commands::apply_command::{ApplyOutcome, apply_command, apply_command_effects};
use types::{
    command::Command,
    ledger_entries::LedgerEntryType,
    order::{OrderStatus, OrderType},
};
use uuid::Uuid;

use crate::common::{BUY_NOTIONAL, MARKET, PRICE, QTY, USDC_AVAILABLE, fixture};

#[test]
fn create_buy_locks_quote_and_rests_on_book() {
    let mut fx = fixture();

    apply_command(
        &mut fx.state,
        Command::CreateOrder {
            command_id: Uuid::new_v4(),
            user_id: fx.user_id,
            market_symbol: MARKET.into(),
            order_type: OrderType::Buy,
            price: PRICE,
            quantity: QTY,
        },
    )
    .unwrap();

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE - BUY_NOTIONAL);
    assert_eq!(usdc.locked, BUY_NOTIONAL);

    let order = fx.state.orders.get("1").unwrap();
    assert_eq!(order.status, OrderStatus::Active);
    assert_eq!(order.order_type, OrderType::Buy);

    let book = fx.state.books.get(MARKET).unwrap();
    let queue = book.bids.get(&PRICE).unwrap();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front().unwrap().order_id, "1");
}

#[test]
fn create_buy_emits_lock_ledger_intent() {
    let mut fx = fixture();
    let command_id = Uuid::new_v4();

    let outcome = apply_command_effects(
        &mut fx.state,
        Command::CreateOrder {
            command_id,
            user_id: fx.user_id,
            market_symbol: MARKET.into(),
            order_type: OrderType::Buy,
            price: PRICE,
            quantity: QTY,
        },
    )
    .unwrap();

    match outcome {
        ApplyOutcome::Applied { intents, .. } => {
            assert_eq!(intents.len(), 1);
            let intent = &intents[0];
            assert_eq!(intent.command_id, command_id);
            assert_eq!(intent.sequence, 0);
            assert_eq!(intent.entry_type, LedgerEntryType::Lock);
            assert_eq!(intent.asset_id, fx.usdc_id);
            assert_eq!(intent.available_delta, -BUY_NOTIONAL);
            assert_eq!(intent.locked_delta, BUY_NOTIONAL);
            assert_eq!(intent.available_after, USDC_AVAILABLE - BUY_NOTIONAL);
            assert_eq!(intent.locked_after, BUY_NOTIONAL);
        }
        other => panic!("expected Applied, got {other:?}"),
    }
}
