use order_engine::commands::apply_command::{ApplyError, apply_command};
use types::{
    command::Command,
    order::{OrderStatus, OrderType},
};
use uuid::Uuid;

use crate::common::{BUY_NOTIONAL, MARKET, PRICE, QTY, USDC_AVAILABLE, fixture};

#[test]
fn cancel_unlocks_quote_and_removes_from_book() {
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

    apply_command(
        &mut fx.state,
        Command::CancelOrder {
            command_id: Uuid::new_v4(),
            user_id: fx.user_id,
            order_id: "1".into(),
        },
    )
    .unwrap();

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE);
    assert_eq!(usdc.locked, 0);

    let order = fx.state.orders.get("1").unwrap();
    assert_eq!(order.status, OrderStatus::Cancelled);
    assert!(order.cancelled_at.is_some());

    let book = fx.state.books.get(MARKET).unwrap();
    assert!(book.bids.get(&PRICE).is_none());
}

#[test]
fn cancel_rejects_other_users_order() {
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

    let err = apply_command(
        &mut fx.state,
        Command::CancelOrder {
            command_id: Uuid::new_v4(),
            user_id: fx.other_user_id,
            order_id: "1".into(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ApplyError::Unauthorized);

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.locked, BUY_NOTIONAL);
    assert_eq!(
        fx.state.orders.get("1").unwrap().status,
        OrderStatus::Active
    );
}
