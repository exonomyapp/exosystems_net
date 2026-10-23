# Generate peer identity early and sign in with it

## Status

Accepted

## Context

ExoAuth is the identity system of the Exosystem family: it creates,
holds, and recovers a self-made online identity. Identities are of two
kinds, exotalk and conscia, both `did:peer` — a self-created
identity code that lives with the user as the root of trust. Identity
and permission checks anchor in the Rust engine layer (Exocore) so
every client opens the same identity; the portable identity logic
lives in `exotalk/exoauth/rust/`, and `src/auth.rs` carries the same
engine entry (`did:peer:2.Vz` + base58 Ed25519 public key). The
sign-in flow emulates the NextGraph shape: identity is created locally
first, then a session ticket gates per-user pages.

## Decision

The sign-in page offers two paths side by side: present an existing
`did:peer` and continue, or generate a new `did:peer` on the spot and
continue signed in with it. Creation runs the engine logic locally and
shows the recovery secret once. Sign-in opens a session token that
the application passes with every data call. Sessions resolve the
three local stores — public, shared-by-permission, private — so pages
assemble per signed-in user from synced content; the gated docs view
is the first such page. Capability requests start empty and grow by
owner approval. Node linking follows the Conscire steps: pick the
node, send identity plus permission slip, the node checks and
confirms, the client adds it to the relay list. Custodial control
lives in the user's profile page.

## Consequences

A newcomer signs in within seconds; a returning user presents an
existing identity. The application holds session tokens and permission
slips. The Conscia node carries content blind. The sign-in page and
the gated docs view are browser-proven on the vendored radix stack
(record: `style-bespoke-over-behavior-primitives.md`): full navigation
round trip, identity generation, and gate checks with zero console
errors. Sessions are bearer tokens; the approved list seeds at server
boot from the SQLite identity store. Secret ownership is proven at sign-in
(record: `prove-secret-ownership-at-sign-in.md`); recovery and
multi-device export run through the Profile Bundle (record:
`recover-and-move-exotalk-profile-across-devices.md`).

## Addendum 2026-09-27 — accepted on browser evidence

The sign-in page and the gated docs view passed the satisfaction test
in a real browser driven via its debug protocol: identity generation
on the spot, present-existing-identity continuation, session ticket
issue, and docs gating for two approved identities, with zero console
errors across a full navigation round trip. `allowed_dids.json` seeds
the approved list with the owner identity and the second approved
identity. Status advances to Accepted on this evidence. Open follow-ons
keep their own records: challenge-response secret ownership
(`prove-secret-ownership-at-sign-in.md`, secret challenge unbuilt per
`src/auth.rs:117`) and the Profile Bundle export/import
(`recover-and-move-exotalk-profile-across-devices.md`). Consciring
and the synced store that complete the gate are tracked in
`run-full-conscia-node-on-exosystems-net.md`,
`keep-local-synced-willow-store.md`, and
`conscire-local-exotalk-with-instance-node.md`.

## Addendum 2026-09-28 — language

Where the Decision above says "node linking", it means conscire:
the Conscire verb governs the relationship throughout these
records. Approval is the node owner's policy decision; nothing in
the flow forces the node.

## Addendum 2026-10-23 — two distinct sign-in contexts

Signing in to the application establishes the ExoTalk user context and unlocks the user's Willow store. That is the first action and it happens through the application's own sign-in flow. Consciring with the Conscia node is a separate, later action that the application performs on the user's behalf once that context exists: the application presents the user's `did:peer` to the node's conscire endpoint, the node's owner approves under the node's own policy, and the node records the capability. The application then shows which nodes the user conscires with, including the node that happens to run on the same instance. The application is both an ExoTalk-context application and a host for a Conscia node, and the two are distinct: signing in to the application does not sign the user in to any Conscia node, and the user's Willow store must be unlocked before any node relation can be known or displayed. The same secret-ownership proof (record: `prove-secret-ownership-at-sign-in.md`) is reused for both actions, but the actions remain separate.

## Addendum 2026-10-23 — identity model and the meaning of "owner"

A sign-in verifies a `did:peer` and opens that identity's Willow store. This does not require ExoTalk or Conscia to be installed or running; the iroh mesh and the Willow store alone are enough to verify who the user is. ExoTalk and Conscia are applications that build on top of that verified identity and store, not prerequisites for it. The exosystem supports two kinds of identity today: `exotalk` for users and `conscia` for non-user nodes. Applications (ExoTalk, ExoSoul, Conscia) and exoapps (Exonomy, Exocracy, RepubLet, and others) are consumers of the verified identity and store, not part of sign-in.

Ownership in this system belongs to Willow stores and to Conscia nodes, not to users of an application. The "owner" that gains operator surfaces on an empty store is the identity named in `OWNER_DID` on the instance — the person who deployed that Conscia node. It is not an ExoTalk user who signed in, and no user becomes an owner by signing in. The bootstrap exists so the instance owner can reach the operator surfaces on first sign-in before any delegation records or joins exist; it is a property of that specific identity on that specific instance, not a general "first user" rule.
