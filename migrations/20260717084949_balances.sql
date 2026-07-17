CREATE TABLE balances (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id),
    available BIGINT NOT NULL DEFAULT 0 CHECK (available >= 0),
    locked BIGINT NOT NULL DEFAULT 0 CHECK (locked >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, asset_id)
);

CREATE TYPE ledger_entry_type AS ENUM (
    'deposit',
    'withdrawal',
    'lock',
    'unlock',
    'trade',
    'fee',
    'adjustment'
);

CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    asset_id UUID NOT NULL REFERENCES assets(id),
    entry_type ledger_entry_type NOT NULL,
    available_delta BIGINT NOT NULL,
    locked_delta BIGINT NOT NULL,
    available_after BIGINT NOT NULL,
    locked_after BIGINT NOT NULL,
    -- link to order/trade/deposit later; nullable for now
    reference_id UUID,
    reference_type TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (available_delta <> 0 OR locked_delta <> 0)
);

CREATE INDEX ledger_entries_user_asset_created_idx
    ON ledger_entries (user_id, asset_id, created_at DESC);