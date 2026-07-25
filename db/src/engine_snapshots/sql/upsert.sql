INSERT INTO order_engine_snapshots (
    id,
    created_at,
    next_order_id,
    order_commands_cursor,
    engine_commands_cursor,
    state
) VALUES (1, now(), $1, $2, $3, $4)
ON CONFLICT (id) DO UPDATE SET
    created_at = excluded.created_at,
    next_order_id = excluded.next_order_id,
    order_commands_cursor = excluded.order_commands_cursor,
    engine_commands_cursor = excluded.engine_commands_cursor,
    state = excluded.state;
