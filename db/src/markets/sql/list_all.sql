SELECT
    id,
    symbol,
    name,
    base_asset_id,
    quote_asset_id,
    status,
    price_tick_size,
    quantity_step_size,
    min_order_quantity,
    min_order_notional,
    created_at
FROM markets
ORDER BY symbol;
