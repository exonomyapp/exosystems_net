# Migrate the networking library to the 1.x series to unblock the database library step

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-28
- Technical Story: the database library step to 0.9.0 is blocked on the locked networking line; this record evaluates the 0.9.0 step fully and proposes the sequenced migration that removes the block (pending row 8).

## Context and Problem Statement

The Conscia service declares `sqlx` 0.8.3 with the `postgres` feature enabled. The 0.9.0 release of `sqlx-postgres` requires stable `sha2` 0.11.0 as a non-optional dependency (verified against the crates registry dependency API, 2026-09-28). The locked `iroh-base` 0.98.0 pins `sha2` to pre-release 0.11.0-rc.5 exactly (verified in the workspace lockfile). The two pins admit no joint resolution, so the 0.9.0 step was attempted and reverted, and the tree was restored to 0.8.3.

The 1.x networking series removes the pin. The registry dependency API for `iroh-base` 1.1.0 lists no `sha2` dependency. The `iroh` 1.1.0 release depends on `iroh-base` 1.1.0. The 1.0 series was released 2026-06-15 and the line stands at 1.1.0 with a 1.2.0 base available under the same requirement.

## Database Library 0.9.0 Evaluation

Release 0.9.0 was published 2026-05-06 with minimum Rust 1.94.0 (verified against the project changelog). The active toolchain reports Rust 1.98.0, which satisfies the minimum.

The combined feature `runtime-tokio-rustls` used by the Conscia manifest no longer exists in 0.9.0 (verified against the crates registry feature API). The replacement is the pair `runtime-tokio` with `tls-rustls`. The remaining requested features (`postgres`, `sqlite`, `macros`, `migrate`, `chrono`, `uuid`, `json`) exist unchanged.

A scratch crate resolving `iroh` 1.1.0 jointly with `sqlx` 0.9.0 (corrected features) locks cleanly on 2026-09-28: `iroh-base` 1.2.0, `sha2` 0.10.9 alongside stable `sha2` 0.11.0, `libsqlite3-sys` 0.37.0, zero pre-release entries in the lockfile. The conflict is removed at resolution level. No repository manifest was modified for this evaluation.

Breaking changes arriving with 0.9.0, from the project changelog: query functions accept only static strings or explicitly marked safe strings; the `Migrate` trait changes shape; the SQLite system library is now a version range advancing on routine updates; SQLite extension loading becomes an unsafe operation behind non-default features; `SqliteValue` becomes non-`Sync`, requiring clones or locks at shared call sites. Each Conscia call site needs review against these points during the step.

## Considered Options

### A. Remain on the locked tree

`iroh` 0.98.x with `sqlx` 0.8.3 and engine 3.46.0. No dependency changes. The engine remains inside the defect range documented in the database record, and the 0.9.0 line stays unreachable.

### B. Migrate the networking line first, then step the database library

Move `exocore` to `iroh` 1.1, verify the build and the sync behaviour, then step `sqlx` to 0.9.0 with the corrected TLS features and run regression tests. This is the only ordering that resolves: the database step cannot precede the networking step.

### C. Step the database library without moving the networking line

Excluded. Joint resolution admits no solution while `iroh-base` 0.98.0 pins the pre-release hash component.

## Decision Outcome

Option B, sequenced. The networking migration proceeds first, gated on companion-crate verification and a green build with sync evidence. The database step follows under the database record, gated on call-site review and regression tests.

## Open Scope

The companion crates `iroh-blobs` (latest 0.103.0) and `iroh-gossip` (latest 0.101.0) remain on the 0.x series with no 1.x releases published (verified against the registry, 2026-09-28). Their compatibility with `iroh` 1.x is unverified and forms the first verification gate. The `iroh` 1.x breaking changes (address lookup registry rework, non-exhaustive public types, per the 0.98.0 through 1.0 release notes) require measurement of the `exocore` call-site changes before effort estimates are stated.

## Pros and Cons of the Options

### A. Remain on the locked tree

- Good: no dependency changes, no call-site review.
- Bad: engine remains inside the defect range; maintained database line stays unreachable.

### B. Networking first, then database step

- Good: removes the version conflict at resolution level; engine leaves the defect range; both dependencies return to maintained lines.
- Bad: two sequenced migrations with call-site review on both; companion-crate compatibility unverified; sync behaviour must be re-evidenced.

## Links

- Pending row 8 (retry of the `sqlx` step)
- `choose-node-database-for-identity-and-willow-state.md` (database record and defect range)
- crates registry: `iroh` 1.1.0, `iroh-base` 1.1.0, `sqlx` 0.9.0, `sqlx-postgres` 0.9.0 dependency and feature APIs
- `sqlx` changelog 0.9.0 (2026-05-06)
- `iroh` releases 0.98.0 through 1.1.0
