# Verify replication on each Archon upgrade

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: replication keeps working across Archon releases because every upgrade passes the same verification.

## Context and Problem Statement

Replication triggers live inside a database whose table shapes are managed by Archon. A release that adds, renames, or removes a column silently breaks any trigger that references the old shape, and the breakage surfaces only as missing history. Trust in the replica therefore rests on proof produced after every upgrade.

## Decision

No Archon upgrade is accepted until three checks pass in order: the trigger definitions apply cleanly against the new table shapes; a verification run writes test rows through every covered table and confirms each row arrives in the Willow namespace; and the schedule binding fires the delivery workflow with a clean receipt. Failures are repaired in the versioned trigger and script files, and the verification run repeats. The passing run is recorded as a dated addendum on this record naming the Archon version.

## Consequences

Upgrades proceed on evidence. The replica carries a dated proof of correctness per Archon version, and a broken upgrade is caught before it serves real work.
