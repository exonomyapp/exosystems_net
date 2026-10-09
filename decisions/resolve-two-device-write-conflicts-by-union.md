# Resolve two-device write conflicts by union

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: two devices can write while apart, and the reunion must not lose either.

## Context and Problem Statement

Conversation and document content in this system is append-only: devices add entries, and each entry is addressed by its content, so two devices writing while apart produce two distinct sets of entries. Any rule that picks a winner deletes authored content, which contradicts the purpose of synchronisation. The union rule fits the data shape because distinct entries interleave.

## Decision

When two devices reconcile, the resulting namespace is the union of both entry sets, ordered by content fingerprint with timestamp as the display tiebreak. No entry is overwritten or discarded. Retraction is the only removal path and propagates as an explicit withdrawal that both devices apply. The per-namespace reconciled marker advances past the union point only after both sides acknowledge it.

## Consequences

No authorship is lost at reunion. Editing of a single value, as opposed to appending entries, stays out of scope until a value-editing surface exists that could produce a conflict.
