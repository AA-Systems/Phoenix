use order_engine::commands::apply_command::apply_command;
use types::{command::Command, order::OrderType};
use uuid::Uuid;

use crate::common::{BUY_NOTIONAL, MARKET, PRICE, QTY, USDC_AVAILABLE, fixture};

#[test]
fn duplicate_create_command_is_ignored() {
    let mut fx = fixture();
    let command_id = Uuid::new_v4();
    let command = Command::CreateOrder {
        command_id,
        user_id: fx.user_id,
        market_symbol: MARKET.into(),
        order_type: OrderType::Buy,
        price: PRICE,
        quantity: QTY,
    };

    apply_command(&mut fx.state, command).unwrap();

    let command_again = Command::CreateOrder {
        command_id,
        user_id: fx.user_id,
        market_symbol: MARKET.into(),
        order_type: OrderType::Buy,
        price: PRICE,
        quantity: QTY,
    };
    apply_command(&mut fx.state, command_again).unwrap();

    assert_eq!(fx.state.orders.len(), 1);
    assert_eq!(fx.state.next_order_id, 2);
    assert!(fx.state.processed_commands.contains(&command_id));

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE - BUY_NOTIONAL);
    assert_eq!(usdc.locked, BUY_NOTIONAL);
}

#[test]
fn duplicate_cancel_command_is_ignored() {
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

    let cancel_id = Uuid::new_v4();
    let cancel = Command::CancelOrder {
        command_id: cancel_id,
        user_id: fx.user_id,
        order_id: "1".into(),
    };

    apply_command(&mut fx.state, cancel).unwrap();

    let cancel_again = Command::CancelOrder {
        command_id: cancel_id,
        user_id: fx.user_id,
        order_id: "1".into(),
    };
    apply_command(&mut fx.state, cancel_again).unwrap();

    let usdc = fx.state.balances.get(&(fx.user_id, fx.usdc_id)).unwrap();
    assert_eq!(usdc.available, USDC_AVAILABLE);
    assert_eq!(usdc.locked, 0);
    assert!(fx.state.processed_commands.contains(&cancel_id));
}
