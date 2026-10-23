-- Session tokens for ExoAuth. Persisted so sessions survive a service restart.
CREATE TABLE IF NOT EXISTS sessions (
    token TEXT PRIMARY KEY,
    did TEXT NOT NULL,
    created_at TEXT NOT NULL
);
