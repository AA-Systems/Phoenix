use order_engine::commands::apply_command::apply_command;
use types::{
    command::Command,
    order::{OrderStatus, OrderType},
};
use uuid::Uuid;

use crate::common::{BUY_NOTIONAL, MARKET, PRICE, QTY, SOL_AVAILABLE, USDC_AVAILABLE, fixture};

#[test]
fn full_fill_settles_balances_and_clears_book() {
    let mut fx = fixture();

    apply_command(
        &mut fx.state,
        Command::CreateOrder {
            command_id: Uuid::new_v4(),
            user_id: fx.other_user_id,
            market_symbol: MARKET.into(),
            order_type: OrderType::Sell,
            price: PRICE,
            quantity: QTY,
        },
    )
    .unwrap();

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

    assert_eq!(
        fx.state.orders.get("1").unwrap().status,
        OrderStatus::Filled
    );
    assert_eq!(
        fx.state.orders.get("2").unwrap().status,
        OrderStatus::Filled
    );
    assert_eq!(fx.state.trades.len(), 1);
    assert_eq!(fx.state.trades[0].price, PRICE);
    assert_eq!(fx.state.trades[0].quantity, QTY);

    let buyer_usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(buyer_usdc.available, USDC_AVAILABLE - BUY_NOTIONAL);
    assert_eq!(buyer_usdc.locked, 0);
    let buyer_sol = fx.state.balances.get(&(fx.user_id, fx.sol_id)).unwrap();
    assert_eq!(buyer_sol.available, SOL_AVAILABLE + QTY);

    let seller_sol = fx
        .state
        .balances
        .get(&(fx.other_user_id, fx.sol_id))
        .unwrap();
    assert_eq!(seller_sol.available, SOL_AVAILABLE - QTY);
    assert_eq!(seller_sol.locked, 0);
    let seller_usdc = fx
        .state
        .balances
        .get(&(fx.other_user_id, fx.usdc_id))
        .unwrap();
    assert_eq!(seller_usdc.available, USDC_AVAILABLE + BUY_NOTIONAL);

    let book = fx.state.books.get(MARKET).unwrap();
    assert!(book.bids.is_empty());
    assert!(book.asks.is_empty());
}

#[test]
fn partial_fill_rests_remaining_buy() {
    let mut fx = fixture();
    let sell_qty = QTY / 2; // 0.05 SOL
    let fill_notional = BUY_NOTIONAL / 2;

    apply_command(
        &mut fx.state,
        Command::CreateOrder {
            command_id: Uuid::new_v4(),
            user_id: fx.other_user_id,
            market_symbol: MARKET.into(),
            order_type: OrderType::Sell,
            price: PRICE,
            quantity: sell_qty,
        },
    )
    .unwrap();

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

    let sell = fx.state.orders.get("1").unwrap();
    assert_eq!(sell.status, OrderStatus::Filled);

    let buy = fx.state.orders.get("2").unwrap();
    assert_eq!(buy.status, OrderStatus::PartiallyFilled);
    assert_eq!(buy.filled_quantity, sell_qty);
    assert_eq!(buy.remaining(), sell_qty);

    let buyer_usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(buyer_usdc.available, USDC_AVAILABLE - BUY_NOTIONAL);
    assert_eq!(buyer_usdc.locked, BUY_NOTIONAL - fill_notional);

    let book = fx.state.books.get(MARKET).unwrap();
    let queue = book.bids.get(&PRICE).unwrap();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front().unwrap().quantity, sell_qty);
    assert!(book.asks.is_empty());
}
