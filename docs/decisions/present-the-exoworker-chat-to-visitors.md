# Present the ExoWorker chat to visitors

## Status

Proposed.

## Context

The exosystems.net home page serves people who hold no account on the site. The instance runs its own oc-worker (`run-oc-worker-on-the-operational-instance.md`, oc-worker repository), so the site already has a local assistant runtime. A visitor conversation carries session-scoped memory: it starts fresh in each browser session, and the site holds nothing about the person beyond that session.

AOTA governs how options are offered: all available options are the offer, unless the offer serves the visitor better with fewer.

## Decision

1. The home page shows a chat box for **ExoWorker**, the Exosystems-branded assistant. The box sends the visitor's prompt to the instance's oc-worker and renders the answer. The box is built for ExoWorker's features.

2. The conversation holds session-scoped memory. A new browser session starts a new conversation.

3. AOTA applies to persistence. The chat offers the visitor every available way to persist the interaction — cookie-extended memory and any further mechanism the deployment provides. Session scope is what the visitor gets when no option is chosen.

4. ExoWorker works inside the constraints of the unauthenticated context. The session and the offered persistence options are its whole surface; the identity-bound services open when the visitor signs in.

5. When the site is conscired to a Conscia node, the chat surface gains a switch to that node's Conscierge app in visitor mode. Visitor mode needs no token, and it shows the community's globally public page: the content the node publishes for open reading. Spec 67 (exotalk repository) defines the Conscierge app and its visitor mode.

## Consequences

- Every visitor can talk to the site's assistant without an account, and the site holds nothing about the visitor beyond the session and the persistence option the visitor chose.
- The persistence offers follow AOTA, so the list grows as mechanisms become available.
- The switch to the Conscierge app appears only when a conscired node exists (`install-the-conscierge-plugin-and-become-node-aware.md`).

## Links

- `run-oc-worker-on-the-operational-instance.md` (oc-worker repository)
- `install-the-conscierge-plugin-and-become-node-aware.md`
- `adapt-the-home-page-to-the-signed-in-identity.md`
- Spec 67, `docs/spec/67_conscia_web_conscierge_specification.md` (exotalk repository)
