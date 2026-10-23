# Run the temporary hand-provisioned node

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: the local application is verified against a live node that exists only for the verification period.

## Context and Problem Statement

The local application exercises sign-in, sharing, synchronisation, and discovery against a live node. This instance serves verification only and is provisioned once, so operational investment in it is limited to keeping the process alive for the verification period.

## Decision

Provision by hand on the instance exactly once: install the daemon build, start it to generate the conscia identity and the empty store, admit the proof accounts through the management interface, and verify the proof run. No supervision, backup, or upgrade procedure is established for this instance beyond keeping the process alive for the proof window.

Durable node management belongs to the ExoMetro implementation, whose first priority is managing Conscia nodes. Repeatable deployment belongs to the flavors deployment record. This instance is retired or rebuilt from those lines once they land.

## Consequences

The verification run has a live counterpart. Product deployment follows the flavors deployment record.
