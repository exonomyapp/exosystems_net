# Define the plain-node scope with discovery

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: the first instance implements a bounded subset of the full helper so that a local application exercises sign-in, sharing, synchronisation, and discovery against it.

## Context and Problem Statement

The full-node record lists every conscia capability. The first instance implements the subset the local application exercises: identity, approval, relay, storage, synchronisation, discovery, verification, and the management interface. Discovery belongs in this subset because shared documents are located through publishable metadata.

## Decision

The plain node implements exactly: own conscia identity (`did:peer`); the conscire endpoint running the join flow; hold-and-forward relay for conscired identities; the SQLite identity store defined in the schema record; Willow namespaces with synchronisation on sign-in and on reconnect; publishable-metadata indexing behind the operator toggle with local search and federated live query; sign-in verification against the store; and the operator management interface defined in the policy record.

Explicitly deferred: high-availability mirroring, geographic routing, internationalisation, the public dashboard and chat interfaces, synchronisation cadence beyond sign-in and reconnect, cross-device conflict merging, and the ExoMetro and ExoSocial flavor extras. Each deferred item is decided separately.

## Consequences

Provisioning has a fixed checklist, and the proof run exercises every implemented capability. The proof run publishes metadata for a shared document and retrieves it through local search.
