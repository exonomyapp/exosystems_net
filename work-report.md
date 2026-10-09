# Exosystems.net Leptos program — work report

Date: 2026-09-26. This report covers everything done to date: the
failing site and its repair, the publishing pipeline foundations, the
UI library trials (mine plus two subagent workstreams), the decision
log, and the design direction. Evidence lives in the trial bench at
`/home/exocrat/code/leptos-ui-trials/` and the decision log in
`decisions/`.

## 1. The site was down — repaired

`exosystems.net` resolved but refused connections: nginx was stopped
and never set to start on boot (leftover from the instance restart).
No config error, no crash, certificates valid. Fixed: enabled +
started nginx (survives reboots, verified 200 on `/` and
`/docs/harden.md`). Certificates: the twice-daily renew timer existed
but was disabled — enabled, plus a reload step so renewed
certificates actually load. Two boot failures (hardware-management
and crash-dump services, meaningless on a virtual machine) were
disabled; boot is clean now. Hardening guide lives on the instance
at `/docs/harden.md`, also served at
`https://exosystems.net/docs/harden.md`; lockdown steps remain
proposals awaiting approval.

## 2. Publishing foundations

- Local repo `/home/exocrat/code/exosystems_net/` holds the live
  page byte-identical (verified by download + diff), fresh git repo.
- GitHub: `gh` authenticated as the project account with repo and
  workflow scope. No remote created yet — awaiting owner/repo name.
- Toolchain: Rust 1.98, WebAssembly target installed, Leptos build
  tool 0.3.9 installed and working. The Tencent box has no Rust
  toolchain (prep never ran) — and needs none: it keeps serving
  files and proxying to app services.
- Direction (owner-decided, recorded in `decisions/`): Leptos 0.8
  with server rendering. Pages will be built per signed-in user from
  synced content; static files cannot do that.

## 3. UI trials — verdicts with evidence

Rule: newest Leptos first (0.8.20), older versions only if evidence
demands it; compile trial on browser target, render trial of real
components to HTML with assertions. Full logs in the bench.

Proven (compile + render, exit 0):
- radix-leptos 0.9.0 — unstyled behavior primitives (chosen core)
- leptos-forms-rs 1.3.0 — validated forms (chosen core)
- leptos-motion, source-pinned — animation (chosen core; all
  registry releases withdrawn 2025-09-17, source only)
- lucide-leptos, floating-ui-leptos, leptos_toaster, leptos-use —
  details (chosen; active per September research)
- soma-ui 0.1.1 — broadest kit (92 component files, charts, blocks,
  screens); Button/Dialog/Input/Card/Tooltip render. Reservation:
  Portal-based overlays (Dialog, Drawer, Sheet…) are empty in server
  HTML — client-only. Held in reserve.
- singlestage 0.4.0 (source) — Button/Dialog/Input/Card render
  inline on the server too; clean browser build; per-component
  features. Stringly-typed variants. Held in reserve; safest server
  citizen of the pre-styled sets.
- leptos-shadcn components direct — button/dialog/form/input/toast
  render (one-line randomness setting required). Wrapper rejected
  (ships a dev audit tool as a required piece). In reserve.

Out, value banked (fix or revival cost recorded in `decisions/`):
- Thaw 0.5.0-beta — server build overflows the compiler's depth
  limit on the current toolchain; older-Leptos retest blocked (old
  Leptos predates the compiler).
- leptodon 1.6.0 — ships with an ungated module calling for unwired
  server pieces; tiny fix, upstream's to make.
- leptix primitives 0.2.3 — stranded on Leptos 0.6; 20
  behavior-pattern files kept as porting reference.

Notable trial mechanics worth reusing: test pages need an explicit
reactive root owner; browser-target noise from server-only
transitive crates is not a verdict; shared workspaces let crates
cover for each other — isolate before judging.

## 4. Decision log (strict ADR format)

`decisions/` follows the upstream ADR repository's prescriptions
verbatim: Nygard shape (Title, Status, Context, Decision,
Consequences) by default, MADR shape (header fields, drivers,
options list, outcome with positive/negative consequences, per-option
pros/cons, links) when options matter. Filenames are imperative verb
phrases; new decisions supersede via new files, never rewrites;
new facts arrive as dated addenda. The `adr` agent skill was
rewritten to the same standard. Template survey (all eleven
skeletons reviewed): Nygard + MADR cover our needs; the lean
executive template is reserved for future narrow picks.

## 5. Design direction (owner-decided)

Keep from the current site: the logo and the product descriptions
only. New design takes its concepts from the ADR reference site:
generous user theming (persisted choice applied before first paint),
language routing, user font-size scaling via one variable, and
backend-free share links. Mechanisms documented; implementation is
the scaffold stage.

## 6. Next steps

1. ~~Scaffold the Leptos 0.8 server app from the live page with the
   chosen core wired in.~~ Done.
2. Create the GitHub repo; pipeline builds and ships on every
   commit; instance serves behind nginx.
3. ~~ExoAuth signup/signin as the first upgrade (forms library).~~
   Done — sign-in page wired to engine identity shape, gated docs
   proven in a real browser with two test identities.
4. Minimal Conscia node on the instance by hand; local synced store;
   gated docs as the satisfaction test. Gate proven; node and store
   remain.
5. ~~Re-add motion as client-side only (animate after page load).~~
   Proven that motion parts crash page serving; browser-only variant
   not yet re-added to hero/cards. Standing rule from the radix
   incident: no UI crate counts as proven until a real browser loads
   it, clicks it, and moves between pages with it.

## 7. Radix browser poison — fixed

The chosen radix trio (radix-leptos 0.9.0 + radix-leptos-core +
radix-leptos-primitives) forces the framework's server flag on every
build target, browser included. With that flag on, the browser bundle
skips building its live wiring — the first click or page change
crashes the whole app. The bench never caught it because it only
rendered server HTML, never hydrated or clicked. Fixed by vendoring
the three crates with the forced flag removed (same pattern as the
forms fix). Proven in a real browser via its debug protocol: full
round-trip navigation, zero errors. See the decision record addendum
for the full account.

## 8. Conscia node lane — plain node on the instance for local ExoTalk sync (tracked with website + ExoAuth)

A Conscia node is an always-on helper computer that holds encrypted user content and passes it on when the user reconnects. To conscire means to join a person's identity to such a helper so it may hold and forward for them. Willow is the shared sync layer that keeps copies of content matched across devices. ExoTalk is the local-first chat app from which the smaller family apps are issued; the local ExoTalk build is the client that will conscire with the instance node and sync. ExoAuth is the single identity system behind all of this: each person and each helper node carries a self-made identity code (a did:peer) as the root of trust. The Tencent Lighthouse instance is the small cloud server (2 GB memory, address 43.157.89.31) that hosts exosystems.net behind nginx. A plain (unflavored) node means the full core helper with no ExoMetro or ExoSocial extras: own helper identity, permission slips (Meadowcap), Willow namespaces for content, SQLite for node state, Iroh peer links, relay, blind indexing of publishable metadata, discovery, persistence mirror, sign-in checks, metrics, and federation.

Finding from the knowledge store: no recent session has a plain node running soundly on the instance yet. The work stands at specified, not deployed. The node decision record is Proposed; the site gate is proven in a real browser while the node and the local synced store remain open (see §6 item 4). No session shows an active node service or passing node health check on the instance.

Sessions that carried this lane, newest first, with what each settled:

- `opencode/ses_f20f06203ffegA6kLlCbX4l4tZ` (2026-09-26 to 2026-09-27): wrote the node decision records in `decisions/`. `run-full-conscia-node-on-exosystems-net.md` (Proposed) supersedes the earlier minimal draft: SQLite for all node state, full Willow service for persistence and sync, Iroh links, full federation protocol, a conscire endpoint for outside apps, plus indexing, discovery, persistence, sign-in checks, metrics, and federation behavior. Owner correction recorded in history: the node keeps the full Willow service for holding and syncing persisted data; limits on the node's scope are not set by the implementer. Companion records: `exosystems-exoauth.md` (sign-in offers present-an-existing-identity or generate-a-new-one, session ticket gates per-user pages, consciring follows the present-identity / owner-approves / node-records-capability / client-keeps-relay steps), `prove-secret-ownership-at-sign-in.md` (challenge-response with the identity secret before allowlist check), `recover-and-move-exotalk-profile-across-devices.md` (encrypted profile bundle carrying identity plus node configurations; fresh consciring per device).
- `opencode/ses_f20ecff6cffeqKhSSQiEz8RfB1` plus the `ses_f208620*` cluster (2026-09-26): grounded the ExoAuth definitions from history (single identity system; ExoTalk as the app that creates apps; Conscia as helper node; consciring as the cryptographic link) and re-verified the satisfaction test: sign-in gates the docs view through identity, node, and synced storage, owner identity first, then a second approved identity.
- `opencode/ses_f2236e95affe1URgV9VxMIjbXM` with `ses_f222501f9ffewJo8lwkhMnIe2B`, `ses_f22266e1dffedpjLL29LL6YcHm`, `ses_f2224aa48ffeTDG5Ni2o5igDmL`, `ses_f222484eeffePtXkjScqZbgMtq` (2026-09-26): owner direction that starts this lane. The full ExoMetro suite (multi-machine database cluster with three distinct database roles, event bus, tunnel, operator console) does not fit the 2 GB instance, so the plain node goes first by hand: helper daemon plus its synced data folder (Willow store), with the gated docs view as the satisfaction test. Same sessions confirm the Leptos 0.8 server-rendered site as the host and ExoAuth signup/signin as the first site upgrade.
- `opencode/ses_f3bc9aaeeffeQ4XmPIr7F2htq9` (2026-09-22 to 2026-09-24): wrote `exotalk/docs/plans/lighthouse.md` (instance provisioning: base OS, Rust/WebAssembly toolchain, Node, Python, OpenCode, Hermes, exo-mcp resources, persistence pointed at the future database cluster) and `exotalk/docs/plans/conscia-to-exosystems-net.md` (Draft full ExoMetro deployment: orchestration cluster, three-role database topology, event bus, dormant tunnel, daemon at `/opt/exo/conscia/conscia` as `conscia.service`, operator console, identity binding, phased rollout with health and conscire checks). Same session brought `https://chat.exosystems.net/login` to 200 through a fresh certificate with the Hermes dashboard behind nginx.
- `hermes/20260920_001520_b3f964`, `hermes/20260920_111212_f90580`, `hermes/20260920_221713_975494`, `hermes/20260920_154000_84bb8f`, `hermes/20260921_010250_e1e214` (2026-09-20 to 2026-09-21): site construction plus the settled vocabulary used ever since (ExoTalk creates apps; smaller apps grow out of it; Conscia helper with Satellite/ExoSocial/ExoMetro flavors; ExoAuth single identity; conscire as the joining of user identity to helper identity).
- `hermes/20260918_173644_609c9d` (2026-09-18 to 2026-09-19): instance foundation. Located and prepared the Lighthouse instance, pointed exosystems.net DNS at it (TTL tightened for fast moves), opened web ports, enabled nginx across reboots, issued certificates with renewal plus reload, and published the family landing pages for exosystems.net and exocracy.org.

Current state: specified, not running. Decision records Proposed; no node daemon, conscire endpoint, or sync check passes on the instance. The website and ExoAuth lanes travel with this lane to the same finish line: site per-user pages, working sign-in flow, plain node on the instance, local synced store, local ExoTalk conscired and syncing, gated docs as proof.

Continue, in order: provision the plain node on the instance by hand per `decisions/run-full-conscia-node-on-exosystems-net.md` (helper identity, Willow store, SQLite state, Iroh links, conscire endpoint); stand up the local synced store; conscire the local ExoTalk build with the instance node and prove sync; run the gated-docs satisfaction test for the owner identity, then a second approved identity. Sources: exokb history (queries above, knowledge store at `/home/exocrat/.local/share/exokb/exokb.db`), `exotalk/docs/plans/lighthouse.md`, `exotalk/docs/plans/conscia-to-exosystems-net.md`, `exotalk/docs/plans/conscia_delivery.md`, `decisions/`.
