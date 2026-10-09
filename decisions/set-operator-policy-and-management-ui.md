# Set operator policy with a management interface

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: the conscire endpoint admits identities under owner approval; the approval rules, their defaults, and the interface that applies them are stated here.

## Context and Problem Statement

Each join request carries storage and visibility consequences. Stated defaults keep individual decisions consistent. An interface presents pending requests with approve and refuse actions, because approval through shell access delays decisions beyond the applicant's waiting period.

## Decision

Policy defaults: every join requires explicit owner approval; no identity is admitted automatically. Delegations carry expiry so every grant has a ceiling. Retention follows capped time and storage budgets with oldest-first eviction. Metadata indexing stays disabled until explicitly enabled, and enabling it indexes only locally authored publishable metadata unless the operator opts into federated or global sources. Search runs local query plus federated live query. Retry follows a schedule with increasing delays for unreachable peers.

The management interface exposes exactly: the pending approval queue with approve and refuse actions; the admitted identity list with delegation detail and revocation; the namespace publication toggles per project; and the settings screens for retention, retry, indexing, and search mode. Every action writes through to the store and the settings index.

## Consequences

The owner admits the proof accounts through the interface, which exercises the same path all later admissions use. Configuration changes are made through the interface, so the running configuration always matches the recorded policy.
