-- Single-row latest snapshot of matching engine state (orders, books, trades, cursors).
CREATE TABLE order_engine_snapshots (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    next_order_id BIGINT NOT NULL,
    order_commands_cursor TEXT NOT NULL DEFAULT '0-0',
    engine_commands_cursor TEXT NOT NULL DEFAULT '0-0',
    state JSONB NOT NULL
);
