# Prove secret ownership at sign-in

## Status

Proposed

## Context

Sign-in in `src/auth.rs` validates the `did:peer` format and checks the allowlist. The Ed25519 signing secret backs the identity.

The engine at `exotalk/exoauth/rust/src/api/identity.rs` holds the Ed25519 signing secret. The `generate_verification_proof` function creates a signed claim from the secret. The web app has access to the same key material through `ed25519-dalek`.

## Decision

Add a challenge-response step to sign-in, executed before the allowlist check.

1. Server generates a random 32-byte nonce, stores it with a five-minute expiry, and returns it to the caller with the presented `did`.
2. Client signs the nonce with the Ed25519 secret for that `did` and returns `{did, nonce, signature}`.
3. Server recovers the public key from the `did` by base58-decoding the key portion after `did:peer:2.Vz`, verifies the signature against the nonce, then checks the allowlist.

The nonce prevents replay attacks. The expiry prevents reuse of stolen challenge responses.

The `create_identity` path generates the secret locally and signs in immediately.

## Consequences

Sign-in requires possession of the private key corresponding to the presented `did:peer`. The sign-in flow includes an additional round trip and a nonce store. The user presents the secret when signing in.

## Addendum 2026-10-23

The save-secret option now persists its own checkbox state per card in browser storage, under `exo_save_secret_signin` and `exo_save_secret_create`, through `store_flag`/`read_flag` helpers in `src/auth.rs`. Each card remembers its own choice across reloads instead of sharing one flag. Sign-in now performs a full reload after success so the signed-in state is visible from first paint. The secret itself continues to be stored in browser storage via `store_saved_secret` when the checkbox is set.

## Addendum 2026-10-23 — scope of the proof

The challenge-response proof establishes secret ownership for the application's ExoTalk user context. The same proof is presented again when the user later conscires with a Conscia node, but that is a separate action governed by the node's own policy and store (record: `build-fresh-boot-and-sign-in-on-node-store.md`). Proving ownership to the application does not admit the user to any node, and the application's sign-in flow does not perform conscire.