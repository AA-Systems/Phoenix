use order_engine::commands::apply_command::apply_command;
use types::{
    command::Command,
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
