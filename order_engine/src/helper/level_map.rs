use std::collections::HashMap;

use types::orderbook::{BookSide, OrderBookDepth};

pub fn level_map(book: &OrderBookDepth) -> HashMap<(BookSide, i64), (i64, u32)> {
    let mut map = HashMap::new();
    for level in &book.bids {
        map.insert(
            (BookSide::Bid, level.price),
            (level.quantity, level.order_count),
        );
    }
    for level in &book.asks {
        map.insert(
            (BookSide::Ask, level.price),
            (level.quantity, level.order_count),
        );
    }
    map
}
