use std::collections::HashSet;

use types::orderbook::{BookSide, OrderBookDepth, OrderBookLevelDelta};

use crate::helper::level_map::level_map;

pub fn diff_book_levels(
    before: Option<&OrderBookDepth>,
    after: Option<&OrderBookDepth>,
) -> Vec<OrderBookLevelDelta> {
    let empty = OrderBookDepth {
        market_symbol: String::new(),
        bids: Vec::new(),
        asks: Vec::new(),
    };
    let before_map = level_map(before.unwrap_or(&empty));
    let after_map = level_map(after.unwrap_or(&empty));

    let mut keys: HashSet<(BookSide, i64)> = before_map.keys().copied().collect();
    keys.extend(after_map.keys().copied());

    let mut updates = Vec::new();
    for key in keys {
        let before_level = before_map.get(&key).copied();
        let after_level = after_map.get(&key).copied();
        if before_level == after_level {
            continue;
        }

        let (side, price) = key;
        let (quantity, order_count) = after_level.unwrap_or((0, 0));
        updates.push(OrderBookLevelDelta {
            side,
            price,
            quantity,
            order_count,
        });
    }

    updates.sort_by(|a, b| a.side.cmp(&b.side).then_with(|| a.price.cmp(&b.price)));
    updates
}
