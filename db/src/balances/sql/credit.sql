INSERT INTO balances (user_id, asset_id, available, locked, updated_at)
VALUES ($1, $2, $3, 0, NOW())
ON CONFLICT (user_id, asset_id)
DO UPDATE SET
    available = balances.available + EXCLUDED.available,
    updated_at = NOW()
RETURNING
    user_id,
    asset_id,
    available,
    locked,
    updated_at;
