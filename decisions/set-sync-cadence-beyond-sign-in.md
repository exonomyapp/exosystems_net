# Set synchronisation cadence beyond sign-in and reconnect

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: synchronisation on sign-in and on reconnect proves the mechanism, but live use needs fresher state than session boundaries provide.

## Context and Problem Statement

Content written on one device stays invisible on the user's other endpoint until the next sign-in or reconnect, which breaks conversation continuity for an application used across a day. A fixed polling interval chosen without usage data either overloads the node or leaves stale data visible. The cadence is therefore set from measured volumes after the verification run.

## Decision

Synchronisation triggers at sign-in and on node reconnect from the first boot. Interval and background triggers are added after the proof run, sized from observed write volume and reconnect frequency: the interval must sit well below the typical time between user-visible writes while keeping node load within the retention and storage budgets. Unreconciled writes remain marked pending until acknowledged, so no cadence choice can silently drop content.

## Consequences

The first proof exercises only the session-bound triggers. The interval addition lands as a dated addendum with the measured volumes that sized it.
