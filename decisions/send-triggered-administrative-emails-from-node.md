# Send triggered administrative emails from the node

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: administrative events on the node reach the people affected by them through email.

## Context and Problem Statement

Approvals, admissions, revocations, and expiries change what a person may do on the node, and the affected person learns of the change only on the next sign-in. A confirmation sent at the moment of the event closes that gap. The first trigger is the conscire confirmation: when the conscire agreement includes an email address, the node sends a confirmation message on approval and on refusal.

## Decision

The node sends one message per trigger event to the address recorded for that event. The trigger set at launch: approval confirmation, refusal notice, delegation revocation notice, and delegation expiry warning. Each trigger has a fixed template stating the event, the affected identity, and the resulting access. Sending runs beside the event: the event completes first, and delivery failure is recorded and retried on the retry schedule without reversing the event.

An address receives mail only when it was provided in the corresponding agreement. No address is ever added from another source, and no message type outside the trigger set is sent through this path.

## Consequences

The proof accounts receive approval confirmations at the recorded addresses. Template additions land as dated addenda with the template text. Delivery depends on the configured mail service in the setup record.
