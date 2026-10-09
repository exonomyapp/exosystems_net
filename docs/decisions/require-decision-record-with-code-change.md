# Require a decision record with watched code changes

## Status

Proposed

## Context

Code and paperwork drift apart silently: a change lands in `src/`,
`Cargo.toml`, or `scripts/` and no record explains why, so the next
builder inherits behavior without rationale. The pipeline record
(`set-up-github-repo-and-ci-pipeline.md`) validates builds before
merge but checks no paperwork. The upstream ADR source describes a
guardrail: an automatic check that refuses a proposed change touching
watched paths unless the change carries a new or updated decision
record, or an explicit exemption line stating why none applies.

## Decision

When the pipeline exists, every proposed change touching `src/`,
`Cargo.toml`, `scripts/`, or `allowed_dids.json` must include a new
or updated record in `decisions/`, or a single exemption line naming
the reason (trivial fix, revert, generated output). The check runs
beside `scripts/fitness.sh` in the same workflow and stops the run
on failure. The watched-path list lives in this record and changes
only by addendum here.

## Consequences

Rationale keeps pace with code by construction; silent drift becomes
a stopped run instead of a quiet merge. Trivial changes stay cheap
through the exemption line. The watched list requires upkeep as the
project grows; an over-broad list blocks harmless changes, so
additions to it arrive as addenda with reasons. Activation waits on
row 2 of `pending.md` (pipeline live); until then this record is
intent, not enforcement.
