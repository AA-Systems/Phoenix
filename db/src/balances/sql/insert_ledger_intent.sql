INSERT INTO ledger_entries (
    user_id,
    asset_id,
    entry_type,
    available_delta,
    locked_delta,
    available_after,
    locked_after,
    reference_id,
    reference_type,
    command_id
) VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10
)
ON CONFLICT (command_id) DO NOTHING
RETURNING
    id,
    user_id,
    asset_id,
    entry_type,
    available_delta,
    locked_delta,
    available_after,
    locked_after,
    reference_id,
    reference_type,
    command_id,
    created_at;
