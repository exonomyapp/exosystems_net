# Petition email templates for conscire approval flows

## Status

Proposed

## Context

When an unknown `did:peer` proves secret ownership against this Conscia node
but is not yet known to it, the sign-in flow asks the person to petition the
node owner for access. The owner must approve or refuse the petition before
sign-in is complete. Petitions can reach the owner through several paths:
in-app owner tools, in-app notifications, and email.

Each Conscia node installer chooses their own mail handler. OpenCloudOS runs
Postfix + Roundcube today, and Brevo carries the transactional outbound leg
for `exosystems.net`, but neither is mandatory. Installers may use Brevo,
Fastmail, a self-hosted SMTP relay, or any other mail provider. Therefore
petition email templates cannot be tied to Brevo or any single mail handler.

## Decision

Petition, approval, and refusal emails are rendered from templates owned by
the Exosystems application, not by the mail handler.

- **Format:** Professional HTML built on CSS Grid for reliable, device-
  independent layout across mail clients. Plain-text fallback included.
- **Ownership:** Templates live in the Exosystems repository
  (`assets/templates/`) and ship with the application, independent of the
  mail handler the node operator configures.
- **Content:** Each petition email names the petitioning `did:peer`, its
  display name, whether mesh-reach was detected (and from which node), and
  a direct link to approve or refuse in the node's owner tools. Approval
  and refusal emails confirm the outcome to the petitioner.
- **Transport:** The node's configured mail handler delivers the message.
  The application supplies the rendered subject and body; the handler
  supplies the transport. Brevo is one supported transport among others,
  not the assumed default.

## Consequences

Mail handler agnostic: any Conscia node installer can use their preferred
transport without changing petition email content or layout. Template
updates ship with application releases. CSS Grid keeps the layout stable
across mail clients and screen sizes. The petitioner always knows who is
petitioning, from where, and what the owner decided.
