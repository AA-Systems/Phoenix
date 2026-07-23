SELECT
    le.id,
    le.asset_id,
    a.symbol AS asset_symbol,
    a.decimals AS asset_decimals,
    le.entry_type,
    le.available_delta,
    le.locked_delta,
    le.available_after,
    le.locked_after,
    le.reference_id,
    le.reference_type,
    le.command_id,
    le.created_at
FROM ledger_entries le
JOIN assets a ON a.id = le.asset_id
WHERE le.user_id = $1
ORDER BY le.created_at DESC, le.id DESC
LIMIT $2;
