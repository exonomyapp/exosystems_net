# Session Transfer Report — Model Updates

> From session: ses_ee119dc06ffeNbghUhosjUM776

## What Was Shipped

The Exosystems.net application lives in a single repository at `github.com/exonomyapp/exosystems_net` with push-to-deploy through `.github/workflows/deploy.yml`: prebuilt cargo-leptos install, cached Rust dependencies, release build of the server binary plus the browser bundle, artifact pack of the binary and the site folder, and deploy to the host with a service restart, landing in about three minutes warm. Both `https://exosystems.net/` and `https://models.exosystems.net/` serve this one Leptos 0.8 application from port 3000 behind nginx, with `/pkg/` revalidated on every load and long-lived asset caching; the standalone models service and its separate deploy are retired.

The application carries the home page with the masthead carousel under a single full-cover screen and transparent floating gadgetry, eight flip cards with art fronts and full-text backs with scroll fade and jump controls, a solid top navigation bar on the models, settings, sign-in, and docs pages carrying the same gadgetry with no self-pointing menu item, challenge-response ExoAuth sign-in with on-the-spot identity creation against the local Willow user list, a models registry table with search-as-keystroke, provider multi-pick, and three-state sorting reading the hosted boltstore `model-updates` database over its HTTP API, and a settings console with display, profile, sessions, mesh access, and owner grant tools. Appearance runs system, light, and dark through `data-appearance` on the root element with the OS preference honored for system mode, content width runs normal and wide through `data-width`, visitor choices persist in browser storage and signed-in choices sync to the identity's server copy, card type scales proportionally with the window from a fluid root size, and the whole shell lays out on CSS Grid with `design-reference-index.html` kept in the repo as the visual reference.

Seed figures carried forward from the retired folder: `model_registry` holds 87 rows (78 `alibaba_maas` with 72 active, 9 `nvidia` with 3 active) with context figures, sources, failure durations, and strike counts, and the round behavior stands as recorded — context admission at 262144 tokens with serving-catalogue figures first and corroboration second, provisional admission for figureless models published within 90 days with rechecking every round, 60-second waiting limit, per-result commits with resume of interrupted rounds, the 3-failure activity rule, transitions-only alerts, Hermes-owned runs, and provider admission by locally published free names. The decisions log holds the notification, sync, operator, recovery, and settings-console records, and the working tree also carries the hardening pass uncommitted: three node-harding decision records, the `docs/harden.md` guide with its local variant, and the `secrets/` gitignore protection over the operator-held cloud keys, all awaiting separate ratification.

## What Changed

The Model Updates feature moved into the main application and the `model-updates/` folder retired to trash: the models interface is a route of Exosystems.net with the shared main-page session, the delegation gate, and the shared gadgetry, and the retired folder's scaffold, seed database, scripts, and reports survive only as the figures and rules carried above.

## What's Next

Owner onboarding first: the owner creates an identity on the main sign-in page, receives the first Admin delegation through the bootstrap path, and grants from the owner tools, which unlocks the gated models route for real use. Next the provider admission work: free-name filter plus live endpoint testing against the hosted database, then a full proving round with the registry and history counts refreshed into the README. The UI pass continues alongside against the reference file: gadgetry stays real icon controls, cards keep flipping with scroll-correct backs, type stays proportional, and the dark palette keeps its contrast. The settings console's three open proposals (notification channel preferences, active-identity indicator, storage readout) await ratification. The trash holds the retired folder plus the earlier build caches and still awaits the word to empty.

## Where Things Live

- `/home/exocrat/code/exosystems_net/src/app.rs` — routes, FloatingGadgets, ModelsNavbar, FlipCard, DisplayGadgets with ThemeDrop and WidthToggle, sign-in, settings, models table, docs.
- `/home/exocrat/code/exosystems_net/src/auth.rs` — challenge-response ExoAuth, Willow-backed sessions, delegation gate, owner grants.
- `/home/exocrat/code/exosystems_net/src/store.rs` — registry reads from the hosted boltstore over HTTP.
- `/home/exocrat/code/exosystems_net/src/lib.rs` — the hydrate export that boots the browser application.
- `/home/exocrat/code/exosystems_net/style/main.scss` — the full theme: grid shell, carousel, flip cards, gadgets, dark palettes.
- `/home/exocrat/code/exosystems_net/design-reference-index.html` — the visual reference the pages build toward.
- `/home/exocrat/code/exosystems_net/docs/decisions/` — the decision log, including the settings-console record and the uncommitted hardening records.
