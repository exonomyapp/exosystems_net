# Shape the Settings page as the identity and mesh console

## Status

Proposed

## Context

The site's records assign control surfaces to one authenticated page. ExoAuth places custodial control on the user's profile page (`exosystems-exoauth.md`). The synced Willow store surfaces per-namespace sync state on the profile page and marks pending writes (`keep-local-synced-willow-store.md`). The Profile Bundle export and import with passphrase recovery run from the profile page (`recover-and-move-exotalk-profile-across-devices.md`). The operator policy record specifies a management interface: the pending approval queue, the admitted identity list with delegation detail and revocation, the namespace publication toggles, and the screens for retention, retry, indexing, and search mode (`set-operator-policy-and-management-ui.md`). Notifications reach signed-in sessions through a menu badge and every affected identity through Brevo email (`application-layer-notifications-over-willow-gossip.md`). The joined Conscia node set is presented for inspection (`discover-joined-nodes-after-sign-in.md`). Delegations live in each user's settings under Willow. Display settings keep a browser-storage copy for unauthenticated visitors and a syncing copy for authenticated users; the owner holds a mesh-wide pause flag over admitting identities.

One page assembles these surfaces so each arriving record lands in its section instead of a new screen. The page exists today with the display controls, the profile editor, the session list, the mesh access state, and the owner's grant tools.

## Decision

The Settings page at `/settings` is the identity and mesh console, organized in sections:

1. **Display.** Appearance (system, light, dark) and content width (fit margins, full width). The visitor copy persists in browser storage; the signed-in copy syncs to the identity's Willow namespace through meadowcap persistence.
2. **Profile.** Display name and avatar that travel with the identity, collected at manual creation or supplied by a linked SSO provider, plus the Profile Bundle export and import with passphrase, offered as file download, QR code, or chunked text.
3. **Sessions.** Every live browser session for the identity, with switching and sign out. The identity chip's dropdown reads the same user list from the local Willow store.
4. **Delegations.** The identity's per-namespace grants with expiry and revocation, read from and written to the user's Willow namespace — Willow dataland. Document-level sharing stays in the docs view per `grant-document-access-per-identity.md`.
5. **Sync.** Per-namespace sync state with the last reconciled point and pending-write markers. Cadence controls join this section after the measured volumes size them (`set-sync-cadence-beyond-sign-in.md`).
6. **Nodes.** The joined Conscia node set for inspection, with the address of each node the identity conscired with.
7. **Notifications.** Badge state for the signed-in session and Brevo email delivery state for the identity.
8. **Mesh access.** The owner's pause flag for admitting identities, default open, applying mesh-wide to every subscriber of the site.
9. **Operator tools.** The pending approval queue with approve and refuse, the admitted identity list with delegation detail and revocation, the namespace publication toggles, and the retention, retry, indexing, and search mode screens.
10. **Gate.** The unauthenticated view carries the Display section. Every other section requires the signed-in session; the Operator tools require the owner identity.

Settings data lives in three places: browser storage for the visitor display copy, the identity's Willow namespace for the synced display copy, delegations, and notification preferences, and the node's SQLite store for operator policy and admissions. Boot and sign-in read the node store as the identity record of authority, retiring the seeded allowlist per pending row 7.

Open proposals, each awaiting owner approval before implementation: notification channel preferences per identity (badge and email toggles), the active-identity indicator for tool storage routing (`route-tool-storage-by-active-identity.md`), and a storage budget readout from the retention caps.

## Consequences

Each record's implementation lands its section on this page, so the console fills in as the sync, notification, and operator records go from proposed to built. The gate keeps unauthenticated visitors at display-only, matching the browser-storage copy. The delegation and operator sections depend on the Willow store and the node store going live, so they arrive with those records. The page grows into the single surface an identity visits to answer "who am I here, what can I do, and what do I subscribe to," and the ADR index gains its replacement entry when a future record supersedes this shape.
