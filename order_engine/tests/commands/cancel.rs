use order_engine::commands::apply_command::{ApplyError, apply_command};
use types::{
    command::Command,
    order::{OrderStatus, OrderType},
};

use crate::common::{BUY_NOTIONAL, MARKET, PRICE, QTY, USDC_AVAILABLE, fixture};

#[test]
fn cancel_unlocks_quote_and_removes_from_book() {
    let mut fx = fixture();

    apply_command(
        &mut fx.state,
        Command::CreateOrder(fx.user_id, MARKET.into(), OrderType::Buy, PRICE, QTY),
    )
    .unwrap();

    apply_command(&mut fx.state, Command::CancelOrder(fx.user_id, "1".into())).unwrap();

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
        Command::CreateOrder(fx.user_id, MARKET.into(), OrderType::Buy, PRICE, QTY),
    )
    .unwrap();

    let err = apply_command(
        &mut fx.state,
        Command::CancelOrder(fx.other_user_id, "1".into()),
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
