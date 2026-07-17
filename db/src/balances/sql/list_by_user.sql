SELECT
    b.asset_id,
    a.symbol,
    a.name,
    a.decimals,
    b.available,
    b.locked,
    b.updated_at
FROM balances b
JOIN assets a ON a.id = b.asset_id
WHERE b.user_id = $1
ORDER BY a.symbol;