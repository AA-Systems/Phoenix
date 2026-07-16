-- Add migration script here

-- Users table 
CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Assets type
CREATE TYPE assets_status AS ENUM ('active', 'archived');


-- assets table 
CREATE TABLE assets (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    symbol VARCHAR(16) UNIQUE,
    name VARCHAR(64) NOT NULL,
    decimals INTEGER NOT NULL DEFAULT 0,
    status assets_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TYPE market_status AS ENUM ('trading', 'halted', 'archived');

-- markets table 
CREATE TABLE markets (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    symbol VARCHAR(32) UNIQUE,
    name VARCHAR(128) NOT NULL,
    base_asset_id UUID NOT NULL REFERENCES assets(id),
    quote_asset_id UUID NOT NULL REFERENCES assets(id),
    status market_status NOT NULL DEFAULT 'trading',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (base_asset_id <> quote_asset_id),
    UNIQUE (base_asset_id, quote_asset_id)
);