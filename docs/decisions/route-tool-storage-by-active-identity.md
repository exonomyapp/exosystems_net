# Route tool storage by active identity

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: signing in with an identity code points both tools at that identity's Willow namespace.

## Context and Problem Statement

Two tools writing to one shared location mix authorship beyond repair, while a separate location per identity keeps each author's content distinct. Two independent selectors would allow the tools to point at different stores at the same time, producing split state that is difficult to diagnose.

## Decision

A single identity selector lives in BMAD's team configuration surface. The selector writes the active identity code into a shared configuration value that Archon reads from its project configuration file on startup. The replication publisher and the BMAD output placement both resolve the target Willow namespace from that value. Signing in with a different identity code repoints both tools at the new identity's namespace. The permission system governs which identities may read which namespaces, and the tools remain unaware of that layer.

## Consequences

Each identity owns a complete and separate history across both tools. Switching identities switches the visible history in both tools together.
