use types::{order::OrderType, orderbook::OrderBook};

use crate::commands::apply_command::ApplyError;

pub fn remove_from_book(
    book: &mut OrderBook,
    order_type: OrderType,
    price: i64,
    order_id: &str,
) -> Result<(), ApplyError> {
    let levels = match order_type {
        OrderType::Buy => &mut book.bids,
        OrderType::Sell => &mut book.asks,
    };

    let queue = levels.get_mut(&price).ok_or(ApplyError::OrderNotFound)?;
    let initial_len = queue.len();
    queue.retain(|book_order| book_order.order_id != order_id);

    if queue.len() == initial_len {
        return Err(ApplyError::OrderNotFound);
    }

    if queue.is_empty() {
        levels.remove(&price);
    }

    Ok(())
}
