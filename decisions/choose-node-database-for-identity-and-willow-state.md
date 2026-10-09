# Choose the node database for identity state and Willow content

- Status: Accepted
- Deciders: project owner
- Date: 2026-09-28
- Technical Story: the instance node needs one database story covering conscired identities, capability delegations, binding proofs, the publishable metadata index, and Willow content on a 2 GB host with no database server.

## Context and Problem Statement

Persistence reaches the disk through four stacked layers. Application
code calls `sqlx`, the Rust database library. The `sqlx` SQLite
driver calls `libsqlite3-sys`, the bindings crate. That crate
compiles a bundled SQLite engine into the binary. The engine reads
and writes the database file. Each layer carries its own version,
and the engine version arrives with the locked dependency tree
rather than by direct selection: `sqlx` 0.8.3 pulls
`libsqlite3-sys` 0.30.1, which bundles SQLite 3.46.0. That chain is
why the engine stands at 3.46.0 while SQLite stable stands at
3.53.4.

Engine 3.46.0 predates a 2026 defect fix. SQLite engines 3.7.0
through 3.51.2 carry a data-race defect in multi-reader mode that
can corrupt a database when two connections write or checkpoint at
the same instant, which matches daemon access. The fix ships in
3.51.3 and later. Current releases, read 2026-09-28: `sqlx` 0.9.0,
`libsqlite3-sys` 0.38.x bundling SQLite 3.53.2, SQLite stable
3.53.4, libSQL crate 0.9.30 stable. libSQL is SQLite's sibling with
built-in sync for offline-readable shared-state copies, named in
the spec for this profile (Spec 41; `ops/satellite.md`).

## Decision Drivers

- The engine must live inside the daemon as a file; no server fits
  the host.
- The metadata index needs full-text search; concurrent readers
  must not block writers; multi-reader mode admits one writer and
  needs periodic checkpoints with reader gaps.
- The reference Willow builders persist content on a key-value
  layer (memory, server-side KV, and browser-storage drivers, with
  small payloads in the store and large payloads as files) and
  build the three-dimensional index in code, because no
  ready-made database supplies Willow's path-ordered range queries
  or fingerprint math.

## Considered Options

### A. Locked tree: `sqlx` 0.8.3 with engine 3.46.0

### B. Newest tree: `sqlx` 0.9.0 with engine 3.53.x

Release 0.9.0 (May 2026) refreshes security dependencies, fixes
one SQLite crash, and converts the underlying SQLite library into
a version range so the engine advances on routine updates.

### C. libSQL with embedded replicas

Same SQL dialect with sync: local copies of shared state readable
offline.

## Decision Outcome

Option B. The engine leaves the defect range, the library stays on
its maintained line, and the dependency tree advances to current
releases throughout.

## Pros and Cons of the Options

### A. Locked tree

- Good: no dependency changes.
- Bad: engine 3.46.0 remains inside the defect range under daemon
  access.

### B. Newest tree

- Good: engine 3.53.x carries the defect fix; maintained library
  line; full-text search and multi-reader mode confirmed at build
  and recorded.
- Bad: dependency bump with regression testing; minimum Rust
  moves; two retired security options removed.

### C. libSQL with embedded replicas

- Good: spec-named; offline-readable shared-state copies without
  a second server.
- Bad: unevaluated second database product for a single host with
  no cross-node sharing yet; re-evaluated when nodes share state.

## Addendum 2026-09-28 — step blocked in dependency resolution

The step to 0.9.0 was attempted in `conscia/Cargo.toml` and
reverted. Version 0.9.0 requires stable `sha2` 0.11.0; the locked
`iroh-base` 0.98.0 pins `sha2` to pre-release 0.11.0-rc.5 exactly,
and the two pins admit no joint resolution. The tree was restored
to 0.8.3 and verified building. The step proceeds when the locked
`iroh` line drops the pre-release pin. Pending row 8 tracks the
retry.

## Links

- Spec 41, Satellite operations note, Spec 11 §11.4, identity
  foundation §§7–8
- willow-js drivers, willowprotocol.org 3D storage note,
  sqlite.org multi-reader documentation, sqlx 0.9.0 changelog
