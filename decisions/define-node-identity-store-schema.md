# Define the node identity store schema

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: sign-in, approval, synchronisation, and discovery all read one store; the table shapes must be fixed before any of that code is written.

## Context and Problem Statement

Four flows touch the same database: sign-in checks identity and delegations, the owner approves pending joins, synchronisation records per-namespace progress, and discovery searches publishable metadata. Without fixed table shapes, each flow would define its own layout and the flows would stop agreeing. The schema below fixes the shared layout on SQLite, the storage selected in the database record, using full-text search for the metadata index and multi-reader mode for concurrent readers.

## Decision

Five tables, all keyed by explicit identity or namespace strings, none by row alias:

- `identities(did TEXT PRIMARY KEY, kind TEXT, created_at TEXT, binding_proof_id TEXT)`: one row per conscired person or helper identity. The kind distinguishes person identities from the node's own helper identity. The proof reference links to the binding table.
- `delegations(id TEXT PRIMARY KEY, delegator TEXT, delegatee TEXT, namespace TEXT, permission TEXT, signature BLOB, expires_at TEXT)`: one row per granted power. The signature is the Ed25519 delegation envelope; expiry bounds every grant so revocation has a ceiling even before explicit withdrawal.
- `binding_proofs(id TEXT PRIMARY KEY, did TEXT, proof BLOB, created_at TEXT)`: one row per ownership proof. The proof binds the identity to its recovery material without storing secrets.
- `namespace_sync_state(did TEXT, namespace TEXT, last_reconciled TEXT, pending INTEGER, PRIMARY KEY(did, namespace))`: one row per identity per namespace. The reconciled marker drives incremental sync; the pending flag marks unreconciled local writes, so every local write remains marked pending until acknowledged.
- `publishable_metadata(namespace TEXT, field TEXT, value TEXT, updated_at TEXT)`: projection rows for discovery, populated only from namespaces whose owner enabled publication, with a full-text index over value. Retraction deletes the rows, so withdrawal propagates through synchronisation.

These tables hold relational state. Willow content resides outside them: small payloads are stored in the key-value layer and large payloads are stored as files, following the reference builder pattern; the three-dimensional Willow index is built in code above that layer.

## Consequences

Boot creates these tables, sign-in reads the first two, approval writes the first three, synchronisation advances the fourth, and discovery searches the fifth. Indexing stays disabled by default and writes nothing until the operator enables it.
