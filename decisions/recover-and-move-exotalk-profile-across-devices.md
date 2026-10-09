# Recover and move an ExoTalk profile across devices

## Status

Proposed

## Context

NextGraph supports wallet export and import via QR code, text code, or file. The wallet holds identities and keys and serves as the root of trust.

In the Exosystem, the ExoTalk profile serves as the root of trust. The profile holds the `did:peer` identity, Ed25519 secret, display name, avatar URL, verified links, OAuth links, name history, and Conscia node configurations. After Exonomy subscription, it also holds vouchers and transaction history.

Spec 10 (Identity Vault) defines the identity backup model. The secret is the root of identity. The secret is exposed for manual backup and encrypted export as a Profile Bundle.

Spec 11 (Meadowcap Capabilities) defines delegation and revocation. Transferring a profile to a new device re-establishes capabilities on the destination device.

## Decision

The ExoTalk Profile Bundle is the recovery and multi-device export mechanism.

**Bundle format:** Encrypted JSON containing the identity record (`did`, `secret`, `display_name`, `avatar_url`, `proof_string`, `verified_links`, `oauth_links`, `name_history`) and Conscia node configurations. Encryption uses a user-chosen passphrase.

**Export:** The profile page provides an export function. The user provides a passphrase. The system encrypts the bundle and offers it as a file download, QR code, or text code split into chunks for manual transfer.

**Import:** The profile page provides an import function. The user provides the file, scanned QR code, or pasted text code and the passphrase. The system decrypts and restores the identity. The destination device conscires afresh with the user's Conscia nodes to re-establish capabilities.

**Passphrase recovery:** The manual secret backup from Spec 10 serves as the recovery path.

**Multi-device:** The same bundle is imported on multiple devices. Each device conscires in its own right. The source device is revoked manually from the profile page.

## Consequences

Users migrate to a new device while preserving identity. Multiple devices operate the same profile. The user is responsible for passphrase safekeeping. Each device holds its own conscire relationship.
