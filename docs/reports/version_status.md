# Version Status Register

The working versions of the current stack against the currently published stable of each, with the date each fact was verified and the record that tracks the upgrade. A behind row carries the pointer to its tracking record. Refresh a row when a component moves or when a deployment pins a version.

## Behind the published stable

| Component | Working version | Published stable (verified 2026-10-26) | Tracked in |
|---|---|---|---|
| iroh — exotalk workspace (`Cargo.lock`) | 0.98.2 | 1.4.1 | `migrate-networking-library-to-1x-line.md`; pending row 8 |
| iroh-gossip — exotalk workspace (`Cargo.lock`) | 0.98.0 | 0.101.0 | `migrate-networking-library-to-1x-line.md`; pending row 8 |
| sqlx — Conscia daemon (`conscia/Cargo.toml`) | 0.8.3 | 0.9.0 | `migrate-networking-library-to-1x-line.md`; pending row 8 |
| Rust toolchain — Exocracy | 1.98.0 | 1.100.0 (released 2026-08-27) | Operator toolchain update |
| Node.js — Exocracy | 24.4.1 | 24.22.1 (the 24 LTS line; the newest line is 26.12.1) | — |
| cargo-leptos — Exocracy | 0.3.9 | 0.3.11 | — |
| Docker Engine — Exocracy | 28.3.3 | 29.9.2 (released 2026-10-20) | — |
| Conscia web suite — existing code | Dioxus 0.7 | — | Superseded by the Leptos 0.8.x line; Spec 67 §2.5 (exotalk repo) |

## Current

| Component | Working version | Published stable (verified 2026-10-26) |
|---|---|---|
| iroh — exosystems.net (`Cargo.lock`) | 1.4.1 | 1.4.1 |
| iroh-blobs — exosystems.net (`Cargo.lock`) | 0.103.1 | 0.103.1 |
| iroh-gossip — exosystems.net (`Cargo.lock`) | 0.101.0 | 0.101.0 |
| sqlx — exosystems.net (`Cargo.toml`) | 0.9 | 0.9.0 |
| Leptos — exosystems.net app (`Cargo.lock`) | 0.8.22 | 0.8.22 (0.9.0 at release candidate) |
| PostgreSQL — Lighthouse instance | 18 | 18 is the current stable major; 19 is at release candidate |
| Prometheus — Exocracy containers | 3.15.1 | 3.15.1 |
| Grafana — Exocracy containers | 13.2.4 | 13.2.4 |
| OpenCode Desktop — Exocracy | 1.19.4 | 1.19.4 (current v1 line; the v2 line is available) |
| oc-worker | 0.3.1 | 0.3.1 (published) |

## Pinned at deployment

| Component | Pin source (verified 2026-10-26) |
|---|---|
| RKE2 — Lighthouse deployment | Stable channel v1.36.5+rke2r1; latest channel v1.37.1+rke2r1 |

## Sources

- crates registry API: `iroh`, `iroh-blobs`, `iroh-gossip`, `leptos`, `sqlx`, `dioxus`, `cargo-leptos` (2026-10-26)
- `static.rust-lang.org` stable channel (2026-10-26)
- `nodejs.org/dist` release index (2026-10-26)
- `api.github.com/repos/moby/moby` releases (2026-10-26)
- `update.rke2.io/v1-release/channels` (2026-10-26)
- `postgresql.org/support/versioning` (2026-10-26)
- `prometheus.io/download`, `grafana.com` download page (2026-10-26)
- Local: `exosystems_net/Cargo.lock`, `exosystems_net/Cargo.toml`, `exosystems_net/vendor/exocore/Cargo.toml`, exotalk workspace `Cargo.lock`, `conscia/Cargo.toml`, `conscia/conscierge_web/Cargo.toml`, `conscia/conscia_web_core/Cargo.toml`, running binaries on Exocracy (`rustc`, `node`, `docker`, `cargo-leptos`), OpenCode Desktop `app.asar`
