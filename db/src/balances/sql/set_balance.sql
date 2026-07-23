INSERT INTO balances (user_id, asset_id, available, locked, updated_at)
VALUES ($1, $2, $3, $4, NOW())
ON CONFLICT (user_id, asset_id)
DO UPDATE SET
    available = EXCLUDED.available,
    locked = EXCLUDED.locked,
    updated_at = NOW()
RETURNING
    user_id,
    asset_id,
    available,
    locked,
    updated_at;
