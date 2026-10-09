# Session Transfer Report — Model Updates

> From session: ses_ee6fc2c33ffeLVrIQc04RgQ3qI

## State

- `https://models.exosystems.net/` is the route serving the full-page Model Updates interface. The page carries no carousel and renders its own top navigation bar carrying the gadgetry: brand, Home, Models, Settings, sign state, appearance System/Light/Dark, and content width Normal/Wide. The home page keeps the carousel with the same gadgetry overlaid on top of it instead of a separate bar. Logically the interface belongs to Exosystems.net: sign-in runs only on the main page at `https://exosystems.net/signin` through the challenge-response ExoAuth flow, and the models route recognizes the main-page session instead of running its own sign-in.
- Session sharing runs through the Willow store: sign-in on the main page writes a session entry, and the models route validates that entry directly instead of running its own sign-in. An unsigned visitor to the models route lands on a sign-in prompt pointing at the main page; the main page returns the visitor with a one-time ticket that the models server exchanges for its session cookie. The models menu item renders only for sessions holding the model-updates delegation, and every models path rechecks the gate server-side. No sessions table lives in boltstore.
- Delegations live in the Willow capabilities namespace only, with no boltstore copy: the gate reads the Willow store directly after sign-in. The `site-delegations.json` working copy retires when the Conscia node runs on the host with the capabilities namespace in its Willow store, as exosystems.net moves toward its ExoMetro flavor.
- Signed-in user settings travel in the user's Willow namespace and sync to every device where the identity signs in; unauthenticated visitor settings stay in browser storage only. Notifications pair a menu badge for signed-in sessions with Brevo email to every affected identity, signed in or not.
- Seed figures from verified rounds: `model_registry` holds 87 rows (78 `alibaba_maas` with 72 active, 9 `nvidia` with 3 active) with context figures, sources, failure durations, and strike counts. Round behavior carried forward: context admission at 262144 tokens with serving-catalogue figures first and corroboration second, provisional admission for figureless models published within 90 days with rechecking every round, 60-second waiting limit, per-result commits with resume of interrupted rounds, the 3-failure activity rule, transitions-only alerts.
- Boltstore 1.0.3 runs on exosystems.net from the self-contained binary at `/opt/boltstore/dist/boltstore`, managed by the enabled `boltstore.service` unit, behind TLS through nginx `/api/` and `/dashboard/` paths. The Leptos site serves from `/opt/model-updates/` (binary plus site assets), managed by the enabled `model-updates.service` unit, behind its own TLS certificate, with DNS `models` A record in the zone. Server values live in `/opt/model-updates/env` on the host. The `chat` DNS record was deleted. The `dashboard/` checkout in git history holds the retired Python snapshot site; the working tree holds the Leptos application.
- `README.md` is the technical document for the finished interface: route, store tables, round flow, failure policy, providers, schedule, dashboard operations, current state. No code excerpts.
- Agreed rules in force: the Leptos interface and the hosted database share the store; OpenCode and Hermes app providers admit only locally published free names (NVIDIA always passing, Alibaba under the context rule); round alerts fire from inside Hermes-owned runs only; no Supabase project record exists anywhere searched. The Flutter scaffolds and bridge crates left the exotalk tree through trash; the native direction follows the adopted UniFFI specification.

## What Was Shipped

- Boltstore install and database on exosystems.net: Bun runtime, self-contained binary, enabled service unit, TLS proxy paths, admin account, `model-updates` database imported from the local seed with matching counts plus the empty `providers` table, round-writer and dashboard API keys, placeholder credential removed.
- Leptos application scaffold at `model-updates/dashboard/`: home, sign-in with browser-side challenge signing, models table behind the delegation gate, settings page with display controls for every visitor plus provider management and owner grant/revoke tools behind Admin, challenge-response server functions, delegation snapshot persistence, registry reads over the dashboard key path. Server, client, and hydrate targets compile clean with zero clippy warnings.
- Public route `https://models.exosystems.net/` with DNS record, dedicated TLS certificate, nginx proxy, enabled service unit, Exosystems grid theme with dark appearance and wide content options, all four routes answering 200.
- Repository hygiene: Flutter plugin scaffold, Dart sources, SDK copy, and all five bridge crates trashed from exotalk; workspace manifest repaired and resolving; `target/` cache folders of five projects trashed, sitting in trash pending emptying.

## What's Next

1. **Single sign-on from the main page.** Sign-in stays on `exosystems.net/signin`; the models route redirects unsigned visitors there with a return address and accepts the one-time ticket back. Sessions live in the Willow store, validated directly by both routes. Gating item for owner onboarding.
2. **Navigation and gadgetry.** The home page keeps the carousel with the gadgetry overlaid on top. The models page renders its own top navigation bar carrying brand, Home, Models, Settings, sign state, appearance, and width gadgets with identical behavior.
3. **Owner onboarding.** The owner creates an identity on the main sign-in page and receives the first Admin delegation through the bootstrap path, then grants from the owner tools.
4. **Free-name filter plus OpenCode live testing.** App-provider admission by locally published free names; real endpoint testing replacing the placeholder function.
5. **Proving round and README refresh.** One full round against the hosted database confirming registry counts, history rows, and dashboard display; current-state numbers refreshed after items 1 through 4 land.

## Relevant Files

- `/home/exocrat/code/model-updates/dashboard/src/auth.rs` — challenge-response sign-in, sessions, delegation gate, owner grant and revoke, settings copies.
- `/home/exocrat/code/model-updates/dashboard/src/app.rs` — routes, navigation bars, sign-in flow, models table, settings and owner tools views.
- `/home/exocrat/code/model-updates/dashboard/src/store.rs` — registry reads from the boltstore database.
- `/home/exocrat/code/model-updates/models.db` — seed figures for the hosted database.
- `/home/exocrat/code/model-updates/README.md` — technical document for the finished interface.
- `/home/exocrat/code/model-updates/docs/decisions/serve-model-data-from-hosted-sqlite-endpoint.md` — architecture record for the hosted store.
- `/home/exocrat/code/exosystems_net/decisions/application-layer-notifications-over-willow-gossip.md` — notification design with Brevo delivery.
- `/home/exocrat/code/exosystems_net/src/auth.rs` — main-page ExoAuth reference (format checks, allowlist gate, token sessions).
- `/home/exocrat/code/exosystems_net/src/app.rs` — main-page navigation and gate pattern reference.
- `/home/exocrat/.hermes/skills/cloud-server-setup/references/brevo-api.md` — Brevo sender authentication reference.
