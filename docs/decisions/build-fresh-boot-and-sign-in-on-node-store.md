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
