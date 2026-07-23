use order_engine::commands::apply_command::apply_command;
use order_engine::queries::answer_query::{
    answer_query, balances_for_user, open_orders_for_user, order_book_depth,
};
use types::{
    command::Command,
    order::OrderType,
    query::{EngineQuery, EngineReply},
};
use uuid::Uuid;

use crate::common::{MARKET, PRICE, QTY, USDC_AVAILABLE, fixture};

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
        other => panic!("unexpected reply: {other:?}"),
    }
}

#[test]
fn open_orders_and_book_after_resting_buy() {
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

    let open = open_orders_for_user(&fx.state, fx.user_id);
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].id, "1");
    assert_eq!(open[0].remaining, QTY);

    let book = order_book_depth(&fx.state, MARKET).expect("market exists");
    assert_eq!(book.market_symbol, MARKET);
    assert_eq!(book.bids.len(), 1);
    assert_eq!(book.bids[0].price, PRICE);
    assert_eq!(book.bids[0].quantity, QTY);
    assert_eq!(book.bids[0].order_count, 1);
    assert!(book.asks.is_empty());

    let request_id = Uuid::new_v4();
    match answer_query(
        &fx.state,
        EngineQuery::GetOpenOrders {
            request_id,
            user_id: fx.user_id,
        },
    ) {
        EngineReply::GetOpenOrders { orders, .. } => assert_eq!(orders.len(), 1),
        other => panic!("unexpected reply: {other:?}"),
    }

    match answer_query(
        &fx.state,
        EngineQuery::GetOrderBook {
            request_id: Uuid::new_v4(),
            market_symbol: MARKET.into(),
        },
    ) {
        EngineReply::GetOrderBook {
            book: Some(book), ..
        } => {
            assert_eq!(book.bids[0].price, PRICE);
        }
        other => panic!("unexpected reply: {other:?}"),
    }

    assert!(order_book_depth(&fx.state, "NOPE").is_none());
}
