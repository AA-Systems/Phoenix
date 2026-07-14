INSERT INTO assets (symbol, name, decimals)
VALUES ($1, $2, $3)
RETURNING id, symbol, name, decimals, status, created_at;