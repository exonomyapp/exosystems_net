# Exosystems.net and the Conscia Node: Direction History

A living record of the direction decisions behind exosystems.net and the Conscia node that will supplement its node operations, told through the lens of how each part was born with one technological approach, expanded into other directions, and was sometimes replaced. It cites the primary sources — decision records, git commits, plans, and reports — and the sessions stored in the exokb knowledge base that record the reasoning at the time.

> **How to read this document.** This document maps direction: which technological approach each part started with, what it grew into, and what replaced it. The recurring theme is a set of "survivors" — long-lived decisions that kept reappearing across the shifting plans and proved to have more continuity than the surrounding churn. The companion document at `exotalk/docs/history.md` covers the application side of the same system: the ExoTalk app, the ExoSoul authoring tool, the Conscia daemon in the monorepo, and the ExoApps they host. Read together, the two show how the site and the node relate to the app they will converse with.

## 0. The cast of parts

The site is one Leptos application with server rendering, version 0.1.0, in the repository at `/home/exocrat/code/exosystems_net/`. Its source is five files — `app.rs`, `auth.rs`, `lib.rs`, `main.rs`, `store.rs` — and it serves both `exosystems.net` and `models.exosystems.net` from port 3000 behind nginx. The front door is nginx, whose two configuration files, `deploy/nginx/exosystems.conf` and `deploy/nginx/models.conf`, hold transport security, caching, and the rewrite from the models subdomain to the models route. The database is a hosted SQLite endpoint served by boltstore and read over its HTTP interface.

The node is a Conscia daemon planned to run on the same instance as a plain node: a conscia identity, a Conscire endpoint, a hold-and-forward relay, a sign-in verification, a management interface, publishable-metadata indexing with local search, and Willow synchronisation, with federation and the flavour extras deferred. Its state lives in one SQLite file inside the daemon, holding identities, capability delegations, binding proofs, namespaces, and per-namespace synchronisation state.

The identity of a visitor is an Ed25519 `did:peer` keypair, with challenge-response sign-in: the site issues a challenge, the visitor answers with the secret, and the answer is checked against the identity list in the visitor's own Willow namespace. The node's own identity is separate, and the node acts on the owner's authority through Meadowcap capability delegations.

The work is recorded in forty-one decision records under `docs/decisions/`, each with a status and a date, indexed newest first in `docs/decisions/README.md`, with the open work listed in `docs/decisions/pending.md`.

---

## 1. Origins: the domain, the free instance, the static page (2026-09-17 to 2026-09-20)

The site began as a question about ownership. On 2026-09-17 the owner asked where the exosystems.net website stood and who controlled it (`ses_f4f42fc57ffe3uPb6ZtVV447iv`, 94 turns). The answer was that the name was held at Namecheap and the hosting at Hostinger. The owner handed over a Hostinger API key, now at `~/.hostinger.yaml`, and asked for free hosting. The session compared Tencent Lighthouse, Alibaba, Arvan, and Yandex free tiers, and settled the framework question by dropping Nuxt explicitly — "forget about nuxt3" — and testing Dioxus against serverless functions.

The instance came on 2026-09-18: a Tencent Lighthouse instance `lhins-093xx2ht`, OpenCloudOS 9, region eu-frankfurt, 2 vCPU, 2 GB, 40 GB SSD, public address 43.157.89.31. The Hostinger zone was rewritten as a fresh professional profile with `@` and `www` pointing at it, and the previous virtual-server records at 153.92.208.222 were dropped as dead (hermes `20260918_173644_609c9d`).

The page itself was born static. On 2026-09-19 and 2026-09-20 the home page was written as a single HTML document, laid out with CSS Grid: a carousel masthead, a vertical menu, and eight product cards for ExoTalk, ExoSoul, Conscia, Exonomy, Exocracy, RepubLet, ThreeSteps, and InterNovel. ThreeSteps and InterNovel joined the family that day, and the owner named Dioxus as the intended framework for both domains.

---

## 2. The site consolidation: static page to Leptos with server rendering (2026-09-26)

The site went down on 2026-09-26 after an instance restart, and the cause was configuration that had never been made durable: nginx was stopped and never set to start at boot, and the twice-daily certificate renewal timer was disabled (`ses_f2236e95affe1URgV9VxMIjbXM`). The fix restored the service and re-enabled the timer with a reload step so renewed certificates actually load.

The outage also carried a decision. The owner settled that pages must be assembled per signed-in visitor from synchronised Willow content, which a static document cannot do. The framework became Leptos 0.8 with server rendering, recorded in `docs/decisions/use-leptos-0-8-with-ssr.md` with status Accepted on 2026-09-26, replacing Dioxus. The repository `/home/exocrat/code/exosystems_net/` was created that day holding the live page byte-identical, deliberately disconnected from the older `exosystems_nuxt` history (`ses_f2242a464ffeVGRGCvmsw18f0u`). The decision log adopted the upstream decision-record format the same day.

The static page did not disappear. It set the shape of the application: the CSS Grid layout, the eight product cards with their art fronts, the carousel masthead, and the proportional type rule all trace back to it, and the server-rendered version carries them forward.

---

## 3. One application and one front door: merging the second service into the site (2026-10-09 to 2026-10-22)

A second application arrived on 2026-10-07 as the Models project, a Python and SQLite round-tester for provider-hosted models with a report published through GitHub Pages. The gate it applied: only models with a context of 262144 tokens or more were admitted, serving-catalogue figures first and corroboration second, with provisional admission for figureless models published within 90 days and rechecking every round, a 60-second waiting limit, per-result commits with resume of interrupted rounds, and a rule that three failures end a model's activity.

On 2026-10-08 the project moved onto exosystems.net with boltstore installed on the host, and the owner objected to it sitting as a separate application on the domain. On 2026-10-09 two ways of joining the models project to the site were put forward. The first kept the project as a separate program in its own folder inside the repository, with its own build step, its own service to run, and its own address to keep working. The second folded the project's pages, its sign-in, and its settings into the one application, so the site becomes a single program, built once, run once, and reached under one domain name, with one sign-in and one set of styles. The owner chose the second, and commit `659caaa` records the merge together with the workflow that builds and deploys the site on every push. The `models/` folder was retired into the site.

The front door survived every replacement. `deploy/nginx/exosystems.conf` holds transport security, revalidation of `/pkg/` on every load, a day of caching for `/assets/`, `/api/` and `/dashboard/` proxied to 127.0.0.1:8080, and everything else to the Leptos server on 127.0.0.1:3000. `deploy/nginx/models.conf` carries the root rewrite for the models subdomain. One gap stood at 2026-10-22: the workflow ships the binary, the site folder, and `allowed_dids.json`, so the models rewrite never reached the host and `models.exosystems.net` still serves the home page. Both files are reference copies for the operator.

The interface work ran from 2026-10-09 to 2026-10-22: the carousel and floating gadgets, the solid navigation bar on the models, settings, sign-in, and docs pages, the flip cards, the settings console, the sign-in flow, the models registry table, and the mobile-first masthead, through commits `a38705c` to `47eae3b`. The masthead was rebuilt mobile-first with a single one-column base declared once at the foot of `style/main.scss`, and a `min-width: 52rem` measure admits the side column. Device behaviour follows from that measure alone. Two structural defects were corrected along the way: the masthead had declared itself a query container while its own rules sat inside `@container masthead`, and a `max-width` threshold expressed in rem could never fire because the fluid root moves the threshold with the viewport, so the rule now uses `min-width`.

A library trial ran alongside, with verdicts and evidence. radix-leptos 0.9.0, leptos-forms-rs 1.3.0, leptos-motion source-pinned, lucide-leptos, floating-ui-leptos, leptos_toaster, and leptos-use were chosen as core; soma-ui 0.1.1, singlestage 0.4.0, and leptos-shadcn components were held in reserve; Thaw 0.5.0-beta, leptodon 1.6.0, and leptix 0.2.3 were set aside. A defect in the radix trio was fixed by vendoring the three crates with the forced server flag removed, after the original wiring skipped live wiring and the first click crashed the application. The standing rule that came out of it: no interface crate counts as proven until a real browser loads it, clicks it, and moves between pages with it.

---

## 4. The node: scope, records, and the plain-node lane (2026-09-27 to 2026-09-29)

On 2026-09-27 the owner asked which session had actually got an unflavoured node running on the instance, ruled the static `allowed_dids.json` a hack that "doesn't stay", and set three standing requirements: three accounts proving the Meadowcap sharing gate, the gate resolving against a live node-side store, and the flow reaching Willow. Those became the five significant requirements in `docs/decisions/significant-requirements.md`, each with its pass measure: the sharing control exercised by an author, a reader with limited powers, and a stranger who is refused; the gate resolving against a live store; the credentials held in the visitor's own Willow namespace; the node answering on a live address; and the node persisting the signed delegations.

The records followed in three days. On 2026-09-27: the full-node scope, the local synchronised Willow store, secret ownership at sign-in, profile recovery across devices, the delivery pipeline, and the bespoke styling. On 2026-09-28: the database choice with status Accepted, and the networking-library migration. On 2026-09-29, in the session titled "C4 diagram label spacing fix" (`ses_f18652580ffeoi9bN4FfA7exeo`): the store schema, boot and sign-in on the node store, the plain-node scope with discovery, the operator policy with its management interface, the synchronisation cadence, the tool storage routing, the Archon row replication, the upgrade verification, the conflict merge by union, the discovery of joined nodes after sign-in, the triggered administrative emails, and the three-flavour deployment plan.

Three decisions in that set carry the most weight. The database is one SQLite file inside the daemon, chosen because no database server fits the 2 GB host, and because the driver in use bundles a SQLite release carrying a data-race defect fixed in a later release. The boot and sign-in record defines construction from an empty store: boot creates the tables, generates the conscia identity, and starts the Conscire endpoint, the relay, and the synchronisation listener; the static allowlist file is deleted; unknown identities enter only through the Conscire endpoint under owner approval. The plain-node scope record fixes what the first boot contains — conscia identity, Conscire endpoint, hold-and-forward relay, identity store, Willow synchronisation, publishable-metadata indexing with local search and federated live query, sign-in verification, and the management interface — and defers federation depth, indexing breadth, availability mirroring, geographic routing, internationalisation, the public interfaces, the synchronisation cadence beyond sign-in and reconnect, cross-device merging, and the flavour extras, each of which needs its own evidence.

The three-flavour deployment plan assigns repeatable deployment to the product lines only: a plain core, ExoSocial, and ExoMetro with node management as its first priority, each a versioned image with a configuration surface and a proof suite run before release.

---

## 5. The store, the sign-in gate, and the mail arrangement

The store and the gate travel together. The schema record fixes five tables: identities and delegations for sign-in and approval, binding proofs for recovery without storing secrets, the per-namespace state table driving incremental synchronisation with a pending flag so unreconciled writes are never dropped, and the metadata projection table serving discovery with retraction by deletion. Willow content itself stays outside these tables, in the key-value and file pattern, because the path-ordered index the protocol requires has no relational equivalent. The operator record joins policy to its interface: every join needs explicit approval, every grant carries an expiry, retention and storage stay capped, indexing stays disabled until it is enabled, and the interface exposes the approval queue, the identity list with revocation, the publication toggles, and the settings screens, with every action written through to the store.

The mail arrangement was specified in full on 2026-10-21 in `docs/decisions/exosystems-email-services.md`, which replaced the earlier provider-specific record. Brevo carries the outbound leg: the sending domain `exosystems.net` authenticated by DNS, the sender identities `support@exosystems.net` displayed as ExoSupport and `noreply@exosystems.net`, the SMTP key, the transactional suppression list, and the delivery events, with the marketing products and campaign services unused. The relay is `smtp-relay.brevo.com` on port 587, and the credential is held on the instance in `/opt/exosystems_net/env` with a copy in the repository's operator-held `secrets/` directory. The inbound leg runs on the instance with Postfix, Dovecot, and Roundcube, and the authentication records for sender, domain, and reporting policy sit at the domain registrar.

Notifications run at the application layer. `docs/decisions/application-layer-notifications-over-willow-gossip.md` (2026-10-08) records the reason: the Willow protocol ships no notification specification, and the gossip layer carries liveness alone. The triggered-administrative-email record covers one message per event, beginning with approval confirmation, refusal notice, revocation notice, and expiry warning, including the Conscire confirmation sent when the agreement carries an address, with delivery failure recorded and retried without reversing the completed event.

---

## 6. Deployment lanes: the Lighthouse instance, Alibaba Singapore, DBOS and Postgres

The deployment work has two lanes and one rejected shape. The first lane is the hand-provisioned Lighthouse instance, hardened in place. The hardening guide was written on the instance and served at `https://exosystems.net/docs/harden.md` on 2026-09-26, and the record `implement-hardening-on-tencent-lighthouse.md` carries status Accepted with the program recorded on 2026-10-09. The guide was then restructured into a general section plus one section per instance.

The second lane is Alibaba Cloud Singapore, instance `i-t4n5ee5yucj5f84co4s4`, `ecs.e-c1m2.large`, 47.84.81.181. The provider's own security evaluation flagged the automatically created default group as open to the internet; the owner moved the instance to a custom group `custom-main` carrying only SSH and detached the default group (`implement-hardening-on-alibaba-singapore.md`, Accepted). A further record plans the ExoMetro node on the Singapore box, and another moves the instance off CentOS 7, whose vendor support ended on 2026-06-30, onto Alibaba Cloud Linux 3, chosen by the owner on that day.

The rejected shape is the cluster. The plan `exotalk/docs/plans/conscia-to-exosystems-net.md` (2026-09-24) describes an RKE2 cluster with three pgEdge nodes on PostgreSQL 17 and ArcadeDB as the graph and vector store, marked Draft for review before provisioning. On 2026-09-29 the owner replaced it with a hand-provisioned temporary instance plus the three-flavour plan, and the cluster plan left the active set of documents.

The memory stack gained a deployment lane of its own. `docs/decisions/deploy-exo-mcp-with-dbos-and-postgres-to-lighthouse.md` puts PostgreSQL 18 on the instance with DBOS holding the scheduled review, forgetting, and curation passes, the retry queue for Cognee writes that report as pending, and the workflow history, while Memento carries working and episode knowledge, Cognee the meaning and procedure knowledge, and Graphify the code map. The companion record `integrate-exo-mcp-memory-gateway-into-tooling.md` routes knowledge by kind and places the instructions in the project instruction file and the behavior skills.

A fourth lane, a self-hosted Iroh relay, was recorded on 2026-10-16 with status proposed, and `install-nexo-rs-on-lighthouse.md` sits beside it.

The wider proposal, `docs/reports/global-deployment-proposal.md` (2026-10-19), splits public artefacts from private data into two planes, treats a relay fleet as first-order infrastructure, distinguishes two classes of lifeline, and recommends Cloudflare with Amazon Web Services, naming Alibaba with Tencent Cloud as the alternative pairing.

---

## 7. The model registry and the identity surfaces

The models route on the site reads the hosted boltstore database over its HTTP interface, and the seed figures carried forward from the retired folder are 87 rows: 78 `alibaba_maas` with 72 active, and 9 `nvidia` with 3 active, each with context figures, sources, failure durations, and strike counts. The table offers search-as-keystroke, provider multi-pick, and three-state sorting, with the picker lifted from the oc-worker dashboard. Provider management sits on the settings page, and the registry is read-only against boltstore in the current build.

The decision behind the hosted endpoint is `docs/decisions/serve-model-data-from-hosted-sqlite-endpoint.md`, and the wider question of which provider catalogue is authoritative was settled with Willow holding delegations alone, on the owner's ruling of 2026-10-08 at 21:55 that the delegation path goes to Willow directly.

The visitor surfaces followed the same pattern of settling into a console. The settings console carries display, profile, sessions, mesh access, and owner grant tools, and the shape of that console is its own record, `shape-settings-page-as-identity-and-mesh-console.md`. Appearance runs system, light, and dark through a data attribute on the root element with the system preference honoured; content width runs normal and wide; visitor choices persist in browser storage and signed-in choices synchronise to the identity's server copy. Card type scales proportionally with the window from a fluid root size, and the whole shell lays out on CSS Grid with `design-reference-index.html` kept in the repository as the visual reference.

---

## 8. The decision log: from one record to forty-one

The log itself is a survivor of this period, and it grew in a shape. It began on 2026-09-26 with the Leptos framework record and the ExoAuth identity record, the latter Accepted. By 2026-09-29 it held the node records, and by 2026-10-09 it held thirty-eight indexed records with a dependency column. It holds forty-one files today, and the rule behind it is recorded in `docs/decisions/require-decision-record-with-code-change.md`: a watched code change carries a record.

Records in the log are never rewritten; new information arrives dated, and a replacement supersedes. The pending list tracks each open decision with its blocker and its landing place, and each row closes with a dated addendum or a superseding record.

The diagrams were held to the same standard. The two architecture sheets for the site and the node were redrawn as measured vector after the first rendering left the action labels crammed between the boxes and the flows crossing operational boundaries. The finished sheets are embedded in their records, the container sheet in `docs/decisions/keep-local-synced-willow-store.md` and the context sheet in `docs/decisions/conscire-local-exotalk-with-instance-node.md`, with the drafting sheet at `docs/decisions/diagrams/context.html`. The title block of each sheet states what its drawing designation means, names the elements shown, enumerates the flows, and gives the notation for boundaries, units, and arrows, followed by the version, the date, and the status.

---

## 9. Cross-cutting survival table (the "survivors")

| Surviving idea | First recorded | Why it persisted | Eventually replaced by / absorbed into |
|---|---|---|---|
| **nginx as the front door** | 2026-09-18 | one place holds transport security, caching, and routing | Absorbed the static page, the Leptos rewrite, and the models subdomain |
| **CSS Grid as the layout philosophy** | 2026-09-20 | the grid carries the page shell and the card layout alike | Carried into the flip cards, the masthead, and the settings console |
| **The eight-card product grid with art fronts** | 2026-09-20 | it is the map of the product family | Gains a carousel, floating gadgets, and proportional type; the set stays |
| **Proportional, zoom-safe type** | 2026-09-26 | a fixed measurement cannot survive a fluid root | Carried into the masthead rebuild and the card art (2026-10-13/14) |
| **Leptos with server rendering** | 2026-09-26 | pages must be assembled per signed-in visitor | The merged build absorbed the second application into it; the framework held |
| **ExoAuth `did:peer` with challenge-response sign-in** | 2026-09-26 | possession of the secret is the gate | Carried into every later record; the node gate resolves against a live store |
| **Willow as the store and the sync layer** | 2026-09-27 | the node holds the visitor's content, and delegations live there | Carried into the schema record, the delegation path, and the flavour plan |
| **Brevo as the mail provider** | 2026-09-18 | the expected administrative volume sits far below the free tier | Expanded into the full inbound and outbound arrangement (2026-10-21) |
| **The decision-record discipline** | 2026-09-26 | a watched code change carries a record | Grew from one record to forty-one, with a pending list and dated addenda |
| **One store, one owner** | 2026-09-29 | embedded databases are single-owner; two processes on one file lie outside the contract | Settled for the node store, and settled for the ExoSim relay on the same ground |
| **Plain-core first, extras deferred** | 2026-09-29 | each deferred capability needs its own evidence | Assigned to the three flavour images with a proof suite before release |

---

## 10. Timeline of key direction decisions (at a glance)

| Date (local) | Decision | Evidence |
|---|---|---|
| 2026-09-17 | Hosting moved to a free tier; Nuxt dropped; Dioxus named as the framework | `ses_f4f42fc57ffe3uPb6ZtVV447iv` |
| 2026-09-18 | Tencent Lighthouse instance `lhins-093xx2ht` chosen in eu-frankfurt; the zone rewritten; Brevo account opened | hermes `20260918_173644_609c9d`, `20260918_222521_1cf99cdb` |
| 2026-09-19/20 | Home page written as a single static document with a carousel masthead and eight product cards; ThreeSteps and InterNovel join the family | hermes `20260920_001520_b3f964` |
| 2026-09-21 | ThreeSteps and InterNovel first-class; the static source preserved | `20260921_010250_e1e214` |
| 2026-09-26 | nginx restart failure diagnosed; Leptos 0.8 with server rendering accepted, replacing Dioxus; the repository created; ExoAuth identity record accepted | `ses_f2236e95affe1URgV9VxMIjbXM`, `ses_f2242a464ffeVGRGCvmsw18f0u` |
| 2026-09-27 | The three significant requirements set; the node records begin | `ses_f206c711dffeiH4Y1puy58Q9Fl` |
| 2026-09-28 | The node database choice accepted; the networking-library migration recorded | `docs/decisions/` |
| 2026-09-29 | The node record set written: schema, boot and sign-in, plain-node scope, operator policy, cadence, storage routing, replication, upgrade verification, conflict merge, discovery, emails, flavours | `ses_f18652580ffeoi9bN4FfA7exeo` |
| 2026-09-29 | The RKE2 cluster plan replaced by the hand-provisioned instance and the flavour plan | — |
| 2026-10-02 | DBOS and PostgreSQL deployment to the instance recorded | `docs/decisions/deploy-exo-mcp-with-dbos-and-postgres-to-lighthouse.md` |
| 2026-10-06/07 | The exosystems gateway registration removed; the Alibaba provider registered in Hermes | hermes `20261006_222922_81c4b3`, `20261007_153537_287cb2` |
| 2026-10-08 | The Models project moved onto the site with boltstore; the delegation path settled to Willow direct; the application-layer notification record written | `ses_ee20aa2c6ffeP7QDG5vc5nCztl`, `docs/decisions/application-layer-notifications-over-willow-gossip.md` |
| 2026-10-09 | The models project merged into the one application: one build, one service, one sign-in; the push-to-deploy workflow created; the Alibaba hardening accepted; the Lighthouse hardening accepted | commit `659caaa`; `docs/decisions/implement-hardening-on-*.md` |
| 2026-10-16 | The self-hosted Iroh relay record written; the work report retired into the transfer report | commit `212c6cd`; `docs/decisions/self-host-iroh-relay.md` |
| 2026-10-17 | The models subdomain rewrite recorded; the container-query masthead | commits `9957ab3`, `64214bb` |
| 2026-10-19 | The global deployment proposal written | `docs/reports/global-deployment-proposal.md` |
| 2026-10-21 | The mail arrangement specified in full | `docs/decisions/exosystems-email-services.md` |
| 2026-10-22 | The interface library trials recorded with verdicts; the display bootstrap rendered | commit `47eae3b`; `docs/reports/session_transfer_report.md` |

---

## 11. Continuity threads that replaced the most

**The site's own surface replaced its own framework, and kept its layout.** A single static document became a server-rendered application, the second application merged into the first, and the models service became a route behind a subdomain rewrite. Through all of it the grid layout, the eight cards, the carousel, and the nginx front door stayed, because each of them answered a need that the replacement carried forward.

**The node's shape went from cluster to single instance and back to a plan of three.** The RKE2 cluster with three pgEdge nodes gave way to one hand-provisioned box, because the local application needed a live counterpart and nothing else was needed for that. The same window then produced the plan for the three product lines with node management as the first priority, which is where the cluster's intent was carried.

**The identity thread runs unbroken through both sides of the system.** The Ed25519 `did:peer` keypair, the challenge-response gate, the delegation model, and the store that holds the delegations all trace to the same decisions, and the same vocabulary carries into the ExoTalk app, whose native plan names Conscire to this node as one of its features. That is the join between this document and the application history: the site and the node exist so that the ExoTalk app has a live counterpart to converse with, and the ExoApps hosted inside ExoTalk obtain their content by subscribing to the Willow namespaces that ExoSoul publishes.

---

## 12. Epistemic note / how this was sourced

The record was assembled by reading the exokb sessions named in the tables above, the decision records in `docs/decisions/`, the commit history of `/home/exocrat/code/exosystems_net/` with `git log --oneline`, the working tree itself, and the transfer report at `docs/reports/session_transfer_report.md` dated 2026-10-22, which is the most recent statement of the shipped state. Dates are given in local time. Session identifiers are the exokb `source_session_id` values, confirmed present through the exokb connection; where a claim rests on a file or a commit, that path or hash is given instead.

The claims about the current tree — forty-one decision records, Leptos 0.8.20, the five source files, the two nginx configurations, the two reports, and the commit head of 2026-10-22 — were read directly from the repository. The claim about the plain-node lane being specified and undeployed rests on the transfer report of 2026-10-22, which states that no node daemon, Conscire endpoint, or synchronisation check runs on the instance. Four items remain unverified at the time of writing: whether the exo-mcp gateway with DBOS and PostgreSQL reached the Lighthouse instance, for which the record is Proposed and no deployment claim appears; whether the Singapore instance completed its move off CentOS 7; whether the self-hosted Iroh relay was deployed, its record being proposed with no deployment found; and whether the models subdomain rewrite reached the host, the transfer report recording that the two nginx files are reference copies and the workflow does not ship them. The birth of the site before 2026-09-19 is likewise untraced: the earlier Nuxt repository's last commits are from 2024, and the live page becomes traceable only from the static document of 2026-09-20.
