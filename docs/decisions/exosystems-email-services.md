# Exosystems email services

- Status: Proposed
- Deciders: project owner
- Date: 2026-10-21
- Technical Story: Exosystems sends administrative mail from its own domain and receives the mail addressed to it. The Brevo relay carries the outbound path; the node carries the mailboxes, the report record and the audit trail.

## Context and Problem Statement

Exosystems sends administrative messages — approval confirmations, refusal notices, delegation revocation notices, delegation expiry warnings — to the addresses recorded in the agreement each event arises from. The sending domain is `exosystems.net`, which receiving mail servers must recognise as authentic, and each recipient must be able to answer. The domain receives mail as well: aggregate reports describing how mail sent as the domain was authenticated by the receiving servers, and any message addressed to the company.

The decision that established this arrangement was taken on 2026-09-29, when the Brevo mail service was chosen for node email. The record that made that choice is this one. It describes the whole of the Exosystems email system, with the Brevo service at its centre, and it supersedes the earlier Brevo-specific wording.

Two properties shape the arrangement. First, the sending path runs on a single relay operated by Brevo, and Brevo's transactional relay is authenticated with a credential that can submit mail and nothing else. Second, every mailbox, every record and every piece of mail Exosystems sends or receives is held on the node's own instance, and the operating account holds no contact records.

## Decision

### Outbound delivery

Every outbound message is submitted to the Brevo relay at `smtp-relay.brevo.com` on port 587 with STARTTLS, authenticated with the SMTP login `b9ff03001@smtp-brevo.com` and its SMTP key. The application owns the connection, the authentication, and the classification of each failure into a retryable condition or a permanent one. A failed delivery is recorded and retried on the retry schedule set in the record on triggered administrative emails; the event that produced the message completes first and is never reversed by a delivery failure.

The credential is held in the instance's environment file `/opt/exosystems_net/env` as `BREVO_SMTP_LOGIN` and `BREVO_SMTP_KEY`, with file permissions `600`. A local copy is held in the repository's operator-held secrets directory at `secrets/brevo-smtp-key.txt`, which the repository excludes from version control.

The sending identity is `support@exosystems.net`, displayed as `ExoSupport`. The account also holds `noreply@exosystems.net` for later use, and the address `thalperi@gmail.com` which was the first sender registered on the account.

Open and click tracking stays enabled on every message. Brevo embeds the open pixel in the message body and rewrites the links, so that an open and a click are recorded against the address that received the message, and the node subscribes to the `opened`, `uniqueOpened` and `click` events of the transactional webhook, so the reading behaviour of each recipient is held in the node's own records. The account's setting, at Transactional email settings → Tracking, holds the choice between attributing an open and a click to the individual address and counting them in aggregate, and it is set to attribute them.

### Inbound mail

The domain receives its own mail on the instance. The `exosystems.net` MX record points at `mail.exosystems.net`, which resolves to `43.157.89.31`. The instance runs the full mail stack:

| Component | Package | Repository | Role |
|---|---|---|---|
| Postfix | 3.8.5-5.oc9 | BaseOS | Accepts mail for the domain on port 25 and delivers it to Maildir |
| Dovecot | 2.3.19.1-12.oc9 | EPOL | Serves IMAP over TLS on port 993 for reading the mail |
| Roundcube | 1.6.18-1.oc9 | EPOL | Webmail at `mail.exosystems.net`, served by nginx over HTTPS |
| PHP | 8.3.31-3.oc9 | AppStream | Runtime for Roundcube |

The instance runs OpenCloudOS 9.6. Roundcube holds its own state in its own SQLite file: user accounts, preferences, contact list, sessions, and the message index it uses for search. The mail itself stays in Maildir and is read over IMAP. Roundcube is configured against SQLite, MySQL or PostgreSQL by a single configuration setting, and it keeps its own database file.

Inbound TCP port 25 is open in the Lighthouse firewall. The `mail.exosystems.net` certificate is issued by Let's Encrypt through the instance's existing certbot installation, whose renewal timer checks every twelve hours with one hour of randomised delay and renews each certificate twenty-one days before it expires. The certificates the instance already holds, for `exosystems.net`, `exocracy.org` and `models.exosystems.net`, share that schedule.

The node reads the Maildir directly for message-triggered work such as the automatic reply. IMAP carries the mail for reading it in Roundcube and for access from other devices.

### Domain authentication

The sending domain is authenticated in Brevo under domain id `6aada8b5ee7878abf5091646`, with DNS as the authentication method. The zone at Hostinger carries these records:

| Name | Type | Value |
|---|---|---|
| `@` | TXT | `v=spf1 include:spf.brevo.com a mx ip4:43.157.89.31 -all` |
| `@` | TXT | `brevo-code:f4ae223c9ae4f5994a338666ab6df66d` |
| `@` | TXT | `google-site-verification=L2fWkzGd9C6L2boDmJ6g8IbT5HixLp0xvfwewyKezdQ` |
| `brevo1._domainkey` | CNAME | `b1.exosystems-net.dkim.brevo.com.` |
| `brevo2._domainkey` | CNAME | `b2.exosystems-net.dkim.brevo.com.` |
| `_dmarc` | TXT | `v=DMARC1; p=reject; rua=mailto:dmarc@exosystems.net` |
| `@` | MX | `10 mail.exosystems.net.` |
| `mail` | A | `43.157.89.31` |
| `@` | CAA | `0 issue "letsencrypt.org"` |
| `@` | CAA | `0 issuewild "letsencrypt.org"` |

Brevo confirms the SPF record, both DKIM records, the ownership code and the DMARC record as present in the zone. Every record in the zone carries a five-minute time to live.

The DMARC policy is `reject`. A receiving server refuses a message claiming to be from `exosystems.net` when neither SPF nor DKIM passes with alignment, and the refusal returns to us as a hard bounce rather than reaching the recipient's spam folder. Both mechanisms align on every message we send: SPF through `include:spf.brevo.com`, and DKIM through the `brevo2` selector, whose signing domain is `exosystems.net`.

The report address is `dmarc@exosystems.net`, held as a mailbox on the instance. A second report address, held on a second instance operated by another provider, is added when that instance exists; that domain must publish `exosystems.net._report._dmarc.<that domain>` as a TXT record with the value `v=DMARC1`, as RFC 7489 §7.1 requires for a report destination in another domain.

The domain is registered in Google Postmaster Tools, which publishes Gmail's own view of the domain's reputation, its spam rate, its authentication results, and the set of Gmail sending addresses to admit in the SPF record. The registration is held by the account `exonomyapp@gmail.com` as its owner, and the verification record Google issued for it, `google-site-verification=L2fWkzGd9C6L2boDmJ6g8IbT5HixLp0xvfwewyKezdQ`, is published at the domain root. The registration was verified on 2026-10-21. The dashboards cover mail addressed to personal Gmail accounts, and the domain's health reads "Not enough data" until the daily volume rises past the threshold below which Google withholds the data.

### Suppression, opt-out and complaint handling

Administrative mail reaches an address only when that address was provided in the agreement the event arose from. Each message carries a `List-Unsubscribe` header pointing at the node's own endpoint, so the opt-out is handled on our infrastructure. An address that opts out is recorded in the node's suppression list and receives no further administrative mail.

Brevo maintains a transactional suppression list alongside ours, holding only the address strings it has blocked, and it is reachable through the API at `GET /v3/smtp/blockedContacts` and `DELETE /v3/smtp/blockedContacts/{email}`. The node reads that list and clears entries there when an address is restored.

The node subscribes to Brevo's full transactional webhook event set — `sent`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` — and records every event against the message it sent. The webhook endpoint is served by the node over HTTPS at `exosystems.net`, and Brevo authenticates its calls to that endpoint with the token Brevo issues for it. The `spam` and `unsubscribed` events place the address on the suppression list.

### What the node records

The Conscia node holds the business records in its SQLite database, reached through the `sqlx` driver, as decided in the record on the node database.

Aggregate reports arrive as compressed XML attachments in the report mailbox. The node parses them itself, in Rust, following parsedmarc's data model, which is the reference implementation for the aggregate report format and the de facto standard for the field names. The parser reads the report from the mailbox, unpacks the gzip or zip attachment, and writes the fields into the node's SQLite tables, so the parsed report data and the rest of the Exosystems records live in one database.

The model is parsedmarc's. A report carries `report_metadata` — the reporting organisation, the report identifier, the beginning and end of the reporting window, the errors and the generator — and `policy_published`, the policy that was in force for the domain, with its `p`, `sp`, `adkim`, `aspf`, `pct`, `fo`, `np` and `testing` values. Each record carries the window it covers, the source with its address and reverse DNS, the message count, the alignment verdicts for SPF, DKIM and DMARC, the policy that was evaluated with its disposition and any override reasons, the identifiers — the header domain, the envelope sender and the envelope recipient — and the authentication results, with the DKIM domain, selector and result and the SPF domain, scope and result.

The parser is ours, and it carries eight additions to parsedmarc's model:

- A registry of the sources authorized to send for the domain, holding the Brevo relay addresses and the instance's own address with the date each was first seen. A report row naming a source outside that registry identifies an unauthorized sender.
- The join with the send log. A report row names a source and a window but no message and no recipient, while the send log holds the message identifier, the trigger and the recipient. Joining them on the source and the window places the receiving server's verdict on the exact messages Exosystems sent.
- The history of the domain's own published policy, with the date each version took effect, so a report is read against the policy in force when it was generated.
- Alignment-change detection. A source that passed SPF and DKIM alignment in an earlier report and fails in a later one is recorded as a change event, and the change is surfaced in the report view.
- Sending-path attribution. The source's reverse DNS is resolved against the registry of sending paths, so a row names the path that owns the address rather than a hostname.
- Geolocation. The country and the network operator behind a source address, from the address lookup service parsedmarc queries.
- Public suffix and rebrand data. The organisational domain behind a source address, and the detection of known mail-forwarding and rebrand patterns, from the lists parsedmarc embeds.
- Sender reputation lookups. The organisation, country and city registered against a source address's domain, from the sender reputation records parsedmarc queries.

The parser handles the three report types in the standard. Aggregate reports are the daily summary per source per receiving organisation, delivered to the `rua` address. Forensic reports are one report per failed message, carrying that message's headers, delivered to the `ruf` address. SMTP TLS reporting records the TLS negotiation between a sending source and a receiving server, delivered to the `tls-rpt` address. Publishing the forensic and TLS report addresses is part of the DMARC record and the separate TLS record the domain carries.

The reports are read in two ways. The Settings page presents them to the operator. A job on the instance reads them as well and acts on them, so that a finding reaches the operator without waiting for the page to be opened.

The records are:

- The send log: message id, trigger, recipient, timestamp, and the outcome of each attempt.
- The suppression list: the addresses that receive no further administrative mail, with the reason and the date.
- The opt-out and complaint history: each webhook event of type `unsubscribed` or `spam`, with the address and the date.
- The DMARC report record: for each aggregate report received, the reporting organisation, the date received, the raw report retained, and one row per sending source carrying the source address, the sending host, the message count, the SPF result and domain, the DKIM result and domain, the DMARC result and disposition, and the header domain.
- The authorized source registry: each source address permitted to send for the domain, with the sending path that owns it and the date it was first seen.
- The published policy history: each version of the domain's own DMARC record with the date it took effect.

The DMARC report record is read by joining the receiving servers' own verdicts with the send log: the report states that a receiving server authenticated and accepted a message, and the send log states that we sent it, to whom, and on what trigger.

### The Settings page

The Settings page of exosystems.net, at the `/settings` route of the Leptos application, is the surface for these records. It carries the identity and mesh console, and its Notifications item already names Brevo email delivery state for the identity. It gains a mail-operations section holding:

- The suppression list, with the addition and removal of an address.
- Brevo's transactional blocked contacts, with the removal of an entry.
- The opt-out and complaint history, as a dated log.
- The DMARC report view: the sending sources, their alignment verdicts, the changes in alignment, the enriched fields, and the authorized-source registry.

The removal of an address from the suppression list restores it to receiving administrative mail, and the same removal clears the address in Brevo's transactional list. Both lists are read from the node.

### Verification

A message sent from `support@exosystems.net` to a recipient at a receiving server is verified by reading that server's own `Authentication-Results` header. The message sent on 2026-10-21 to `thalperi@gmail.com` returned `dkim=pass header.i=@exosystems.net header.s=brevo2`, `spf=pass` on `77.32.148.25`, and `dmarc=pass (p=QUARANTINE sp=QUARANTINE dis=NONE) header.from=exosystems.net`. The message identifier was `<202610211726.84771262764@smtp-relay.mailin.fr>`.

The sending record is read back from Brevo at `GET /v3/smtp/statistics/events` and `GET /v3/smtp/emails`, and the delivery state of each message is confirmed by the webhook.

### What the Brevo account holds

The account holds the sending domain, the sender identities, the SMTP key, the transactional suppression list of address strings, and the delivery events. It holds no contact records. The account's marketing products and campaign services are not used, and the marketing plan vertical and marketing automation are held unused on the account.

The account's authorized address list, at Settings → Security → Authorized IPs, holds the addresses permitted to call the API and the SMTP relay, and the same list governs both credential types. The instance's address `43.157.89.31` is on it, along with the address the operator's workstation presents while it is on a mobile hotspot, which changes when the carrier reassigns it. An SMTP key carries no learning period, so a host added later has its address placed on the list before it sends.

### Carried forward

The automatic reply from `support@exosystems.net` is a separate piece of work: the node reads the Maildir of that mailbox and answers each message it receives.

## Consequences

Delivery follows the standing of the Brevo account and of the sending address within it. Every administrative message leaves through one relay under one domain with one sender identity, which keeps the sending pattern consistent for the receiving servers that score it.

The instance carries the mailboxes, the webmail, the suppression list, the send log and the report record, so a reader of the Exosystems mail system finds all of it on one machine and in one database. The relay account holds a credential that can submit mail and nothing further.

The `reject` policy converts an unauthenticated send into a refusal we can see. A sending path added later without alignment fails on its first message and is recorded, rather than reaching a recipient's spam folder unnoticed.

The report record depends on the instance being reachable. While the instance is down, the reports generated in that window are not received, and the second report address on the second instance holds them once that instance exists.

The free Brevo plan allows 300 transactional sends per day, with up to 1,000 further transactional messages held in a retry queue when the daily allowance is reached.

The mail stack adds two listening services to the instance, a mail transfer agent on port 25 and an IMAP service on port 993. The instance hardening review follows the installation, and `docs/harden.md` is updated to cover the new services.

## Not yet carried out

The following are specified above and are not yet in place as of 2026-10-21:

- The Postfix, Dovecot and Roundcube installation on the instance, and the inbound TCP port 25 rule in the Lighthouse firewall.
- The `mail.exosystems.net` certificate and the nginx virtual host that serves Roundcube.
- The sending path in the node, the webhook subscription, the suppression list and the report record.
- The Rust aggregate-report parser and the authorized source registry it maintains.
- The suppression and opt-out section of the Settings page.
