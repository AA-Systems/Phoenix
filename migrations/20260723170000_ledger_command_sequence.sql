-- Allow multiple ledger rows per command (lock + trades, etc.).
ALTER TABLE ledger_entries DROP CONSTRAINT IF EXISTS ledger_entries_command_id_key;

ALTER TABLE ledger_entries
    ADD COLUMN IF NOT EXISTS sequence INTEGER NOT NULL DEFAULT 0;

CREATE UNIQUE INDEX IF NOT EXISTS ledger_entries_command_id_sequence_uidx
    ON ledger_entries (command_id, sequence);
