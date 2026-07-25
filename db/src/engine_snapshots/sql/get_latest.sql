SELECT
    id,
    created_at,
    next_order_id,
    order_commands_cursor,
    engine_commands_cursor,
    state
FROM order_engine_snapshots
WHERE id = 1;
