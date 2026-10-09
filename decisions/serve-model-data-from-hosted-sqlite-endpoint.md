# Serve Model Updates data from boltstore on exosystems.net

- Status: proposed
- Deciders: project owner, with build execution by the coding agent
- Date: 2026-10-08
- Technical Story: the Model Updates interface at `https://models.exosystems.net/` reads and writes round data through boltstore on exosystems.net. The interface is logically part of Exosystems.net: sign-in runs only on the main page, and the models route recognizes the main-page session instead of running its own sign-in. The local system consumes results from the hosted address.

## Context and Problem Statement

Verification rounds write results, registry standing, and round history. The site application reads the same data for live filtering, sorting, provider management, and operations that keep the owner updated. The exploratory arrangement is a local SQLite file plus a committed JSON snapshot rebuilt after each round, feeding a static report site on GitHub Pages through Python scripts. The snapshot duplicates the database, lags the rounds, and cannot serve provider management writes. The Python scripts and the Pages report path retire with this record. Boltstore is a self-hostable SQLite platform: one process serves isolated SQLite databases over an HTTP REST API, with per-database API keys, an admin dashboard at `/dashboard`, a raw SQL endpoint, and Bun or Docker deployment. Running boltstore on exosystems.net beside the Leptos site application gives rounds and dashboard one shared store on one host.

## Decision Drivers

- Rounds and dashboard read and write the same tables with no lag between them.
- Provider keys stay out of the site repository in every form.
- The finished site carries the verified round behavior: context admission at 262144 tokens with serving-catalogue figures first and corroboration second, provisional admission for figureless models published within 90 days with rechecking every round, the 3-failure activity rule, per-result commits with resume of interrupted rounds, and transitions-only alerts.
- Migration carries the current 87 registry rows with their verified context figures and failure evidence intact.
- Signed-in user settings travel in the user's Willow namespace and sync to every device where the identity signs in. Unauthenticated visitor settings stay in browser storage only.
- Delegation rows live in the boltstore delegation database for website gate checks and ride the Willow capabilities namespace for peer sync, announced over Iroh gossip. Notification behavior runs at the application layer, watching new delegation entries the way round alerts watch round results.

## Considered Options

1. Boltstore on exosystems.net, with the Dioxus site application hosted on the same machine.
2. Local SQLite file plus committed snapshot feed with Pages report (exploratory arrangement).
3. Third-party hosted database account (managed Postgres or similar).
4. Repository-committed database file synced by push.

## Decision Outcome

Option 1: boltstore runs on exosystems.net holding the model-updates database, and the Leptos interface serves from the `models` route on the same host against the boltstore HTTP API as its primary store. The models page carries its own top navigation bar with the gadgetry and no carousel; the home page keeps the carousel with the same gadgetry overlaid on top. Sign-in runs only on the main page; sessions live in the Willow store, validated directly by both routes, and unsigned visitors to the models route receive a sign-in prompt pointing at the main page with a return address. No sessions table lives in boltstore. Delegations live in the Willow capabilities namespace only, with no boltstore copy: the gate reads the Willow store directly after sign-in. The local system consumes results from the hosted address. The model updates menu item renders only for a signed-in identity holding the model-updates meadowcap delegation; unauthenticated visitors never see the item. Approvals live in the Willow capabilities namespace, replacing the static allowlist file. Willow and Iroh carry peer sync for the node layer per the node records.

### Positive Consequences

- Rounds publish the moment they commit; the dashboard shows the latest round.
- The snapshot exporter, the report generator, and the per-round commit cycle retire.
- Provider management reads and writes live data from the settings page.
- Schema changes apply once, in the hosted database, for all consumers.
- The site and the store share one host, one TLS story, and one access log.

### Negative Consequences

- Rounds require network reachability to exosystems.net.
- Credential handling becomes load-bearing: the boltstore admin account, per-database API keys, and key rotation live with the operator.
- The data API ships without built-in rate limiting; a reverse proxy or equivalent stands in front of the server where throttling matters.

## Pros and Cons of the Options

- Option 1 carries live sharing and single-schema authority on operator-run infrastructure, at the cost of hosting and credential operations.
- Option 2 carries zero new infrastructure and full local operation, at the cost of permanent lag, duplication, and no writable settings page.
- Option 3 carries managed backups and mature tooling, at the cost of a new vendor account, SQL dialect differences against the existing schema, and a second credential estate.
- Option 4 carries simplicity, at the cost of merge conflicts on every round, unbounded repository growth, and public exposure of operational data.

## Links

- Boltstore: `https://github.com/boltstore/boltstore`
- Host: `https://exosystems.net`
- Seed store: `/home/exocrat/code/model-updates/models.db`
- Checkout directory: `/home/exocrat/code/model-updates/dashboard/`
- Provider picker port source: `/home/exocrat/code/bonjourno/oc-worker/dashboard/app.js` (`useMultiSelect`, `probeChecked`, `{ col, dir }` sort state)
- Appearance and width port source: `/home/exocrat/code/exo-mcp/mcp-ui/`
- Future work ordering: `/home/exocrat/code/model-updates/docs/reports/session_transfer_report.md`

## Implementation checklist

1. Install boltstore on exosystems.net from Docker or the Bun process with a data directory, a chosen port, nginx passing the service path, and the admin account created through the first-run setup flow. Landed 2026-10-08: self-contained binary at `/opt/boltstore/dist/boltstore`, enabled `boltstore.service` unit, nginx `/api/` and `/dashboard/` paths behind the existing TLS, admin account created, placeholder credential removed from the unit.
2. Create the model-updates database with the `model_tests`, `model_registry`, `test_run_history`, and `providers` tables, and issue one API key for round writes plus one for dashboard reads and provider-management writes. Delegations stay out of boltstore: approvals and delegation rows live in the Willow capabilities namespace, read directly by the website gate after sign-in.
3. Seed the registry from the verified round data: 87 rows (78 `alibaba_maas`, 9 `nvidia`) with context figures, sources, failure durations, and strike counts.
4. Build the Leptos site application with live operations against the boltstore HTTP API: provider multi-pick, model checkboxes, three-state sorting, search-as-keystroke, Runs above Models stacked layout, date-only Last tested, ported appearance and width controls, and the settings page with provider management. Menu visibility follows meadowcap delegations: the model updates item renders for signed-in identities holding the delegation and stays hidden from unauthenticated visitors.
5. Host the built site on exosystems.net beside boltstore and confirm the local system reads results from the hosted address.
6. Retire the Python round scripts, the snapshot exporter, the report generator, and the Pages deployment workflow once the hosted site serves round data.
7. Run one full round against the hosted database and confirm registry counts, history rows, and dashboard display.
8. Delegations persist through the capability layer's own file at `exostorage/capabilities.json`, read by the website gate and by the node code alike. No second working copy exists. Landed 2026-10-09: the `site-delegations.json` copy is removed and the gate reads the in-process capability store directly.
