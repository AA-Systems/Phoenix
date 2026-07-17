SELECT *
FROM balances
WHERE user_id = $1
ORDER BY asset_id;