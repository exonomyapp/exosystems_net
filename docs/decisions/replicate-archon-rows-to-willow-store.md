# Replicate Archon rows to the Willow store

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: Archon workflow content survives the machine it runs on by replicating row by row into identity-scoped Willow namespaces.

## Context and Problem Statement

Workflow runs, messages, approvals, and events live in Archon's embedded SQLite database, which exists on one machine only. A new machine signing in with the same identity finds none of that history. File exports taken on a schedule leave gaps and cannot prove completeness. The trigger language of the embedded database permits only row reads and writes inside the transaction, so the database cannot reach outside itself.

## Decision

Each content table carries triggers that copy every inserted, updated, and deleted row into a dedicated outbox table within the same atomic transaction. A delivery script reads undelivered outbox rows in order, writes each one into the owning identity's Willow namespace, and marks the row delivered. The script runs as an ordinary Archon workflow started by a schedule binding firing every sixty seconds, with a named resource and queueing overlap so a new delivery run starts only after the previous one completes. Within one run the script drains in passes two seconds apart and exits on the first empty pass, so sustained activity replicates without waiting for the next firing. Sixty seconds bounds the wait for the first row after idleness; each idle firing costs one indexed query plus one run record. Every firing leaves a durable receipt for inspection.

## Consequences

The Willow store holds a complete row-level replica of Archon content per identity. A new machine with the same signed-in identity synchronises the full history. Trigger definitions are versioned beside the workflow definitions and reviewed on every Archon upgrade under the verification record.
