# Set up the Brevo mail service for node email

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: triggered node email is delivered through the Brevo relay.

## Context and Problem Statement

The node composes administrative messages and requires a delivery relay with authentication, sender verification, and a transactional sending path. Brevo provides an SMTP relay for transactional mail at host `smtp-relay.brevo.com` on port 587 with STARTTLS, port 465 with SSL/TLS, or port 2525 as fallback (verified against Brevo developer documentation, 2026-09-29). Authentication uses an SMTP key generated in the Brevo account, which is distinct from the API key. Sending addresses require a verified sender or verified domain.

## Decision

The operator creates the Brevo account, verifies the sending domain for exosystems.net with SPF and DKIM records, and generates a dedicated SMTP key for the node. Conscia is configured with the relay host `smtp-relay.brevo.com`, port 587 with STARTTLS, the SMTP username, the SMTP key as password, and the verified sending address. The SMTP key is stored as a secret and is accepted on update without ever being returned by the administration interface.

## Consequences

Delivery availability follows the Brevo account standing and domain verification state. Key rotation is a settings update through the management interface.
