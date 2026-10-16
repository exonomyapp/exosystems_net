# UI Library Trials — Findings and Verdicts

## Scope

Trial bench at `/home/exocrat/code/leptos-ui-trials/` evaluated UI component crates against Leptos 0.8.20. Each crate was compiled on the browser target and rendered to HTML with assertions. Full logs and render outputs live in the bench.

## Proven (chosen core)

| Crate | Version | Role | Notes |
|-------|---------|------|-------|
| radix-leptos | 0.9.0 | Unstyled behavior primitives | Chosen core. Vendored with the forced server flag removed after the browser-poison fix (see below). |
| leptos-forms-rs | 1.3.0 | Validated forms | Chosen core. |
| leptos-motion | source-pinned | Animation | Chosen core; all registry releases withdrawn 2025-09-17, source only. |

## Chosen (details, active per September research)

- lucide-leptos — icons
- floating-ui-leptos — positioning
- leptos_toaster — toasts
- leptos-use — utilities

## Reserve (not actively used, kept for reference)

| Crate | Version | Status | Reason |
|-------|---------|--------|--------|
| soma-ui | 0.1.1 | Broadest kit (92 component files, charts, blocks, screens); Button/Dialog/Input/Card/Tooltip render | Portal-based overlays (Dialog, Drawer, Sheet…) are empty in server HTML — client-only. Held in reserve. |
| singlestage | 0.4.0 | source; Button/Dialog/Input/Card render inline on server; per-component features; stringly-typed variants | Held in reserve; safest server citizen of the pre-styled sets. |
| leptos-shadcn | components direct | button/dialog/form/input/toast render | Wrapper rejected — ships a dev audit tool as a required piece. One-line randomness setting required. Held in reserve. |

## Out (value banked, fix or revival cost recorded in `docs/decisions/`)

| Crate | Version | Reason |
|-------|---------|--------|
| Thaw | 0.5.0-beta | Server build overflows the compiler's depth limit on the current toolchain; older-Leptos retest blocked (old Leptos predates the compiler). |
| leptodon | 1.6.0 | Ships with an ungated module calling for unwired server pieces; tiny fix, upstream's to make. |
| leptix primitives | 0.2.3 | Stranded on Leptos 0.6; 20 behavior-pattern files kept as porting reference. |

## Notable trial mechanics worth reusing

- Test pages need an explicit reactive root owner.
- Browser-target noise from server-only transitive crates is not a verdict.
- Shared workspaces let crates cover for each other — isolate before judging.

## Radix browser poison — fix and future rule

The chosen radix trio (radix-leptos 0.9.0 + radix-leptos-core + radix-leptos-primitives) forces the framework's server flag on every build target, browser included. With that flag on, the browser bundle skips building its live wiring — the first click or page change crashes the whole app. The bench never caught it because it only rendered server HTML, never hydrated or clicked. Fixed by vendoring the three crates with the forced flag removed (same pattern as the forms fix). Proven in a real browser via its debug protocol: full round-trip navigation, zero errors.

**Standing rule for the future:** no UI crate counts as proven until a real browser loads it, clicks it, and moves between pages with it. The vendored radix crates must stay vendored; do not update them without re-verifying in a real browser.

## Artifacts on disk that are no longer needed

The trial bench's raw intermediate artifacts — render logs (`logs-render-*`, `logs-trial-*`), render output directories (`render-forms`, `render-motion`, `render-radix`, `render-single`, `render-soma`), result aggregations (`render_results*.txt`, `isolated_results.txt`), check logs (`check.log`, `check-shadcn.log`), and the isolated test script (`isolated.sh`) — were working files used to reach the verdicts above. The verdicts are now documented here and in `docs/decisions/`. The bench itself (`/home/exocrat/code/leptos-ui-trials/`) is no longer actively used and can be removed if the team does not intend to re-run trials.
