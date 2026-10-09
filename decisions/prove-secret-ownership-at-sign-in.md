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
