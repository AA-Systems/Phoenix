-- Postgres UNIQUE allows multiple NULLs, so legacy rows without command_id are fine.
ALTER TABLE ledger_entries
    ADD COLUMN command_id UUID UNIQUE;
