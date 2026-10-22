# Generate peer identity early and sign in with it

## Status

Accepted

## Context

ExoAuth is the identity system of the Exosystem family: it creates,
holds, and recovers a self-made online identity. Identities are of two
kinds, person and helper-node, both `did:peer` — a self-created
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
boot from `allowed_dids.json`. Secret ownership is proven at sign-in
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

## Addendum 2026-10-22 — the owner identity

The two identities seeded on 2026-09-26 were written into
`allowed_dids.json` and the instance's credential file, and their
secrets were never kept. Neither identity could sign in, and the
owner's tools were unreachable on the live site, because the
application's own calls were answered by a routing error on the way
(see `ship-the-nginx-configuration-with-the-deployment.md`).

The owner identity is now `did:peer:2.VzwrWZ1eCZd48Y6TgRCzseHkxhVydYXUPQDP4h14e4zsq`,
created through the sign-in page on 2026-10-22, with its secret in the
owner's hands. The instance's credential file names it in `OWNER_DID`,
and it holds the operator surfaces while the delegation store is empty.

The two seeded identities stay in `allowed_dids.json` because the
fitness check holds the approved list to two entries and no second
live identity exists yet. They name accounts that no one can reach.
The owner's identity is the third entry, and it can sign in.
