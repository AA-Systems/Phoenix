UPDATE sessions
SET
    revoked_at = NOW(),
    last_used_at = NOW()
WHERE refresh_token_hash = $1
  AND revoked_at IS NULL
RETURNING
    id,
    user_id,
    refresh_token_hash,
    expires_at,
    revoked_at,
    created_at,
    last_used_at,
    user_agent,
    host(ip_address) AS ip_address;
