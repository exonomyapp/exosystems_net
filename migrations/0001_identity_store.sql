-- Identity store schema for the Exosystems Conscia node.
-- Replaces the JSON files (allowed_dids.json, users.json, capabilities.json,
-- site-flags.json, site-settings.json) that previously tracked identities,
-- profiles, delegations, flags, and settings on disk.
-- Boot and sign-in now read the SQLite store instead.

CREATE TABLE IF NOT EXISTS identities (
    did TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    created_at TEXT NOT NULL,
    binding_proof_id TEXT,
    display_name TEXT,
    avatar TEXT
);

CREATE TABLE IF NOT EXISTS delegations (
    id TEXT PRIMARY KEY,
    delegator TEXT NOT NULL,
    delegatee TEXT NOT NULL,
    namespace TEXT NOT NULL,
    permission TEXT NOT NULL,
    signature BLOB NOT NULL,
    expires_at TEXT
);

CREATE TABLE IF NOT EXISTS binding_proofs (
    id TEXT PRIMARY KEY,
    did TEXT NOT NULL,
    proof BLOB NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS namespace_sync_state (
    did TEXT NOT NULL,
    namespace TEXT NOT NULL,
    last_reconciled TEXT,
    pending INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (did, namespace)
);

CREATE TABLE IF NOT EXISTS publishable_metadata (
    namespace TEXT NOT NULL,
    field TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS petitions (
    id TEXT PRIMARY KEY,
    did TEXT NOT NULL,
    display_name TEXT,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);

CREATE TABLE IF NOT EXISTS site_flags (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS site_settings (
    did TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (did, key)
);
