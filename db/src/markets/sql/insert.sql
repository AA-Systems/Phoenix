INSERT INTO markets (symbol, name, base_asset_id, quote_asset_id)
VALUES ($1, $2, $3, $4)
RETURNING id, symbol, name, base_asset_id, quote_asset_id, status, created_at;