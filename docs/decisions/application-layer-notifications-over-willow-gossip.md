# Application-layer notifications over Willow entries and Iroh gossip

## Status

Proposed

## Context

Delegation grants, round results, and petition approvals must reach the affected identities without polling pages by hand. The official Willow family (Data Model, Meadowcap, Confidential Sync, Drop Format, Transfer Protocol, Willow’25) holds no notifications specification, verified against willowprotocol.org. Iroh ships gossip broadcast (`iroh-gossip`, epidemic broadcast trees per HyParView and PlumTree, per docs.iroh.computer): topic broadcast with eventual delivery, no delivery guarantees, no per-identity inbox, no offline delivery. Gossip therefore carries liveness only. Email delivery runs through Brevo, with the `exosystems.net` sender domain authenticated (DKIM records and DMARC policy live in DNS) and the API key held server-side. The website navbar already gates menu items per signed-in identity.

## Decision

Notifications run at the application layer above Willow and gossip, in three parts.

1. Entry watchers. The node watches the capabilities Willow namespace for new delegation entries, tombstone revocations, and rescue capabilities, and watches the models result feed for new round entries. Every watched entry carries author, timestamp, and affected identity. The website gate reads delegations from the Willow store directly after sign-in; no boltstore copy exists.
2. Gossip announcements. Each new watched entry triggers an Iroh gossip announcement on the governance topic carrying the entry hash and the affected identity, so online peers learn within the sync round instead of discovering the entry later.
3. Delivery channels. Signed-in sessions receive a menu badge through the existing gate functions in `src/app.rs`. Every affected identity receives a Brevo email carrying the actor, the action, and the address of the relevant page, whether signed in or not. The audit log records every notification with its entry hash and channel.

## Consequences

Delegates learn about grants without polling. Revocations propagate with their tombstones. The website menu badge and the WhatsApp bridge share one watcher implementation. Gossip announcement volume grows with entry volume; the announcement carries hashes only, with payloads fetched through Confidential Sync by entitled peers.
