CREATE TABLE idempotency_records (
    id TEXT PRIMARY KEY,
    transfer_id TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)