# Build fresh boot and sign-in on the node store

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: boot and sign-in are defined as construction from an empty store.

## Context and Problem Statement

The initial state is an empty store with the fixed schema and a conscia identity for the node. No migration step exists.

## Decision

Boot performs exactly: create the five schema tables when absent; generate the node's conscia identity when absent and store it as the single conscia-kind row; start the conscire endpoint, the relay, and the synchronisation listener. The static allowlist file is deleted; boot and sign-in read the store.

Sign-in performs exactly: accept the presented identity code; complete the challenge-response round against the identity secret; look up the identity and its delegations in the store; admit when a valid delegation covers the request and refuse otherwise. Unknown identities are not admitted by sign-in; they enter only through the conscire endpoint with owner approval, which writes the identity, delegation, and binding rows that later sign-ins read.

## Consequences

The first boot yields an empty gate that admits nobody until the owner approves the first joins. Every later sign-in exercises the same path as all earlier ones, so the proof accounts exercise the same path as every subsequent sign-in.

## Addendum 2026-10-23

This record governs the Conscia node's own boot and sign-in: the node's identity store, its conscia identity, and the verification of a presented identity against that store. It is distinct from the application's ExoTalk user sign-in, which establishes the user's ExoTalk context and unlocks the user's Willow store (record: `exosystems-exoauth.md`). The node's sign-in verification is the node checking a presented `did:peer` and its secret-ownership proof against the node's store; it does not, by itself, sign the user in to the application, and the application's sign-in does not, by itself, admit the user to the node. The application bridges the two: after the user's ExoTalk context exists, the application presents the user's identity to the node's conscire endpoint and the node's owner approves under the node's own policy.