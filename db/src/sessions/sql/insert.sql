INSERT INTO sessions (
    user_id,
    refresh_token_hash,
    expires_at,
    user_agent,
    ip_address
)
VALUES ($1, $2, $3, $4, $5::inet)
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