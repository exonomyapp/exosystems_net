# Provision the authenticated settings profile on the instance

- Status: Proposed
- Deciders: project owner
- Date: 2026-10-22
- Technical Story: The Settings page at `/settings` is the authenticated console of exosystems.net. Each of its authenticated sections reads or writes state that lives on the instance. This record states what the instance must provide for each of those sections, so that the provisioning of the console is one checklist rather than a set of arrangements made section by section.

## Context and Problem Statement

The console's sections are set out in `shape-settings-page-as-identity-and-mesh-console.md`: display, profile, sessions, delegations, sync, nodes, notifications, mesh access, operator tools, and the gate. Its data lives in three places: browser storage for the visitor's display copy, the identity's Willow namespace for the synced copy and the delegations, and the node's SQLite store for the operator policy and the admissions. The mail operations the console carries — the domain's mailboxes, the suppression list, Brevo's blocked contacts, the opt-out history, the DMARC report record — live on the same instance under `exosystems-email-services.md`.

Each of those arrangements was made separately, and the settings profile has no single statement of what the instance must hold for it. The provisioning already carried out is spread across the instance's service units, its environment file, its nginx configuration, its certificates, its mail stack, its boltstore service, and its state directory. A later reader cannot tell from the decision log which of those the console depends on.

The state the console reads on the instance is narrower than the console's design. The application reads its approved identity list from the seeded allowlist file today, and the node's identity store takes that role when the node is deployed. The Willow state directory the application writes does not exist on the instance yet. The node is not deployed there. The Brevo credential on the instance is the SMTP key alone; the API key, which the suppression and delivery views read through, is held on the operator's workstation.

## Decision

The instance carries the provisioning for the authenticated settings profile in one place, stated here and checkable on the instance.

### The state each section reads

| Section | What the instance provides | Where it lives | Standing on 2026-10-22 |
|---|---|---|---|
| Gate | The session token and the server-side capability check; the owner identity list | `OWNER_DID` in `/opt/exosystems_net/env`; the approved-identity source | The owner list is set; the identity source is the seeded `allowed_dids.json`, to be replaced by the node's identity store per pending row 7 |
| Display | The identity's synced display copy | The identity's Willow namespace | The namespace is the application's Willow store at `exostorage/willow/`, to be the node's store |
| Profile | Display name and avatar; the Profile Bundle export and import | The identity's Willow namespace | The namespace is provisioned as above; the bundle is unbuilt |
| Sessions | The identity's live browser sessions | The node's store | The node is not deployed |
| Delegations | Per-namespace grants with expiry and revocation; the pending approvals | The identity's Willow namespace; the node's identity store | The store is the application's capabilities file at `exostorage/capabilities.json`; the node's store is not deployed |
| Sync | Per-namespace sync state with the last reconciled point | The node's store | The node is not deployed |
| Nodes | The joined Conscia node set with each node's address | The node's discovery endpoint | The node is not deployed |
| Notifications | Badge state for the session; Brevo delivery state per identity | The session; the send log in the node's store | The badge is built; the send log waits on the node |
| Mesh access | The mesh-wide flag over admitting identities | The site's Willow store at `exostorage/willow/site-flags.json` | Built and live |
| Operator tools | The pending approval queue, the admitted identity list with delegation detail and revocation, the namespace publication toggles, and the retention, retry, indexing and search screens | The node's store | The queue, the list, the grants and the flags are built against the seeded allowlist and the application's store; the node's store is not deployed |
| Mail operations | The domain's mailboxes with their Maildir storage; the Roundcube user and identity for each; the send log, suppression list, opt-out history, DMARC report record, authorized-source registry and published-policy history; the Brevo webhook endpoint | The mail stack on the instance; the node's store; the node's endpoint | The mailboxes, the Roundcube users, the certificate and the webmail are in place; the records and the webhook wait on the node |
| Models | The hosted `models` database and its read key | `MODELS_STORE` and `MODELS_DASH_KEY` in `/opt/exosystems_net/env`; boltstore on `127.0.0.1:8080` | Built and live |

### The provisioning the sections share

- **The credential file.** `/opt/exosystems_net/env`, mode `600`, is the single file of instance credentials, read by the site's service unit through `EnvironmentFile`. It holds the Leptos site settings, the models address and key, the owner and node identities, the node secret, and the Brevo SMTP login and key.
- **The service unit.** The site runs from `/opt/exosystems_net` as root, with the working directory setting the paths the application resolves its state against. A server function therefore reaches the mail server's own accounts, its alias file and the Roundcube state on the same machine.
- **The state directory.** `exostorage/` holds the capabilities file, the site settings file and the Willow store. The application creates it on first write.
- **The services the console's data comes through.** nginx terminating TLS for `exosystems.net` and `mail.exosystems.net`, with the site's certificates renewed by the instance's certbot timer on its twelve-hourly schedule; the site on `127.0.0.1:3000`; boltstore on `127.0.0.1:8080`; Postfix on port 25 and Dovecot on port 993 for the mail sections.
- **The mail server's accounts.** Each mailbox is a system account on the instance with a Maildir under `/home/<local part>/Maildir`, named in the instance's alias file where the name is one the alias file owns, and paired with a Roundcube user and identity.
- **The Brevo credentials.** The SMTP login and key submit mail. The API key reads the transactional blocked contacts, the delivery events and the webhook subscription, and it belongs in the credential file beside the SMTP pair so that the console's mail sections are served from the instance.

### What the console may do to the mail server

The mail operations section creates a mailbox by creating the system account, setting its password, creating the Maildir, writing the alias where the name requires one, and adding the Roundcube user and identity. Removing a mailbox withdraws the account and its Roundcube user, and retains the Maildir on the instance as the archive of what the address received, so that the record of a mailbox that existed survives its removal.

## Consequences

The console's provisioning becomes checkable in one place: the credential file, the state directory, the service unit, the four services, the mail accounts and the two Brevo credentials. A section that reads state the instance does not hold shows the section as unavailable rather than failing at the request.

The mail operations section gains a boundary worth stating. The console's authority is the owner's identity and the server-side capability check, and the operations it performs reach the mail server's accounts as root on the instance. The record that grants the owner the console's tools is the operator policy record; the mail account operations belong to the same authority.

The sections that read the node's store stay unavailable until the node is deployed on the instance. The credential file gains a Brevo API key at that point, and the API key's own home on the operator's workstation is unchanged.

## Not yet carried out

- The Brevo API key in the instance's credential file, beside the SMTP pair.
- The node's deployment on the instance, which carries the send log, the suppression list, the opt-out history, the DMARC report record, the authorized-source registry, the published-policy history, the webhook endpoint, the sessions, the sync state and the operator tools.
- The application reading the approved identity list from the node's identity store, retiring the seeded allowlist per pending row 7.
- The console's mail operations section: the mailbox list with its create and remove actions.

## Addendum 2026-10-23 — avatar upload on identity creation

The "No identity yet" card now carries a file field that reads the chosen image as a data URL in the browser (190 KB client limit, 256 KB server limit) and passes it through `create_identity` into the identity store, so a new identity can carry an avatar from the moment it is created. The identity chip and the settings list render the avatar, falling back to the initial letter when none is present. This realises the Profile section's display name and avatar entry in the table above.

## Addendum 2026-10-22 — the Brevo API key and the mailbox section

The Brevo API key stands in the instance's credential file beside the SMTP login and key, and the file keeps owner-only permissions. The console's mail-operations section is built: the Settings page carries a Mailboxes section that lists the domain's mailboxes with their addresses and sending names, creates a mailbox with its account, password, mail store, alias and webmail user, and withdraws one, which leaves the mail store on the instance as the archive. Its record is `manage-mailboxes-from-the-settings-console.md`.

The gate the console reads now recognises the bootstrap owner: an identity named in `OWNER_DID` holds the operator surfaces while the delegation store is empty, so the owner reaches them on the first sign-in.
