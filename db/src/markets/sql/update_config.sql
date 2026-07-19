UPDATE markets
SET
    price_tick_size = $2,
    quantity_step_size = $3,
    min_order_quantity = $4,
    min_order_notional = $5
WHERE id = $1
  AND status = 'halted'
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
