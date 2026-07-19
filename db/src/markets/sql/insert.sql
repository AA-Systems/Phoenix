INSERT INTO markets (
    symbol,
    name,
    base_asset_id,
    quote_asset_id,
    price_tick_size,
    quantity_step_size,
    min_order_quantity,
    min_order_notional
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING
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
    created_at;