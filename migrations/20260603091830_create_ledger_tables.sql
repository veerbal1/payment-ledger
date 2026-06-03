CREATE TABLE transfers (
    id TEXT PRIMARY KEY
);

CREATE TABLE ledger_entries (
    id TEXT PRIMARY KEY,
    transfer_id TEXT NOT NULL REFERENCES transfers(id),
    account_id TEXT NOT NULL REFERENCES accounts(id),
    amount BIGINT NOT NULL CHECK (amount >= 0),
    direction TEXT NOT NULL CHECK (direction IN ('Credit', 'Debit'))
);