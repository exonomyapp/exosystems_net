# Significant requirements

Requirements that shape the architecture, each with its pass/fail
measure. They are the standing tests behind the decision records;
a record is accepted only when its requirements pass.

These are SR1 to SR5. The five requirements are cited that way
throughout the log and in the fitness checks. A decision record is
numbered in the log's index and a record's dependencies are written R
followed by that record's number, so the two never read alike.

## SR1 — Three accounts prove the gate, the sharing control, and the permission system

Pass: in a real browser, the docs view carries a sharing control per
document, working like sharing on mainstream docs: the author names
an identity, grants whichever Meadowcap powers the document needs,
and removes names to revoke. Three signed-in accounts exercise it
independently — the docs author with full powers, an enabled reader
holding limited powers on some documents and wider powers on others,
and a stranger who is refused wherever the author's publish settings
deny them. Whether the stranger sees a listing follows the author's
per-document setting, never a software default. The stranger case is
what proves the permission system (Meadowcap) enforces as well as grants. Records: `exosystems-exoauth.md`,
`grant-document-access-per-identity.md`.

## SR2 — The gate resolves against the node's live SQLite identity store

Pass: sign-in checks a live node-side store of conscired identities
and capability delegations, built the spec way: consciring through
the node's conscire endpoint (join flow in Spec 11 §11.4) → approval
by the node's owner under the node's own policies → signed
capability delegation (Spec 11); OAuth binding proof stored beside
the identity for recovery; the user's encrypted credential material
kept in the user's own Willow namespace and synced to conscired
nodes, never decrypted by the node (identity foundation §§7–8). No
static allowlist anywhere: `allowed_dids.json` is deleted and boot
plus sign-in read the store (pending row 7). Manual setup mirrors a
fresh install: generate the node's helper identity, conscire each SR1
account through the endpoint, and persist the resulting delegations,
binding proofs, and namespaces — the same shape a fresh ExoTalk
install plus its own Lighthouse Conscia node produces on its own.

Store backend, verified 2026-09-28: the spec names libSQL/SQLite for
Satellite (Spec 41; `ops/satellite.md`), and the daemon already
depends on `sqlx 0.8.3` with its SQLite driver
(`conscia/Cargo.toml:35`), so no new database product enters. The
build verifies the bundled SQLite carries FTS5 for the metadata
index plus WAL mode for concurrent readers, and records the versions.
Willow content sits on the same substrate the way the reference
Willow builders do it (`willow-js`: key-value drivers — memory, Deno
KV, IndexedDB — with small payloads in the store and large payloads
on the filesystem; the protocol authors note no off-the-shelf
database supplies path-ordered 3D range queries or fingerprint
aggregation, so the 3D index is built in code above the key-value
layer). Node relational state — identities, delegations, binding
proofs, publishable metadata — lives in SQLite tables beside it.

## SR3 — No UI code counts as proven until a real browser clicks it

Pass: the crate loads in a real browser that navigates between pages
and activates its controls with zero console errors. Server HTML
rendering alone passes nothing. Styled and unstyled sources
compete on equal footing; neither is mandated or barred. Record:
`style-bespoke-over-behavior-primitives.md`.

## SR4 — Node, store, and client prove the gate as one

Pass: the local build conscires to the instance node, publishes a
test entry locally, syncs, reads it back through the gated docs view
for the author and reader accounts while the stranger stays refused,
then revokes and confirms sync stops. Records:
`run-full-conscia-node-on-exosystems-net.md`,
`keep-local-synced-willow-store.md`,
`conscire-local-exotalk-with-instance-node.md`.

## SR5 — Secret ownership at sign-in

Pass: sign-in completes a challenge-response round against the
identity secret before the store check. Unbuilt; tracked in
`prove-secret-ownership-at-sign-in.md`.
