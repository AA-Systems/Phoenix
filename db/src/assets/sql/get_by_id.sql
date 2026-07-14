SELECT id, symbol, name, decimals, status, created_at
FROM assets
WHERE id = $1;