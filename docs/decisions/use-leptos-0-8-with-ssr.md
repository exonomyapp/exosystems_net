# Use Leptos 0.8 with server rendering

## Status

Accepted

## Context

exosystems.net assembles pages per signed-in user from that user's
locally synced Willow content; the gated docs view is the first such
page (owner direction, 2026-09-26). Leptos stable is the 0.8 line —
0.8.20 current, September 2026 — and the chosen UI stack builds on it
with the Rust 1.98 toolchain. The Tencent instance serves the site
behind nginx; nginx stays as the front door with the application
behind it.

## Decision

The site is a Leptos 0.8 application with server rendering and
hydration, behind the existing nginx reverse proxy. The build targets
the newest 0.8 patch the chosen stack builds on; the 0.8 line holds
until the library ecosystem carries 0.9.

## Consequences

Pages assemble per user at request time; sign-in, identity, and
synced-content gating live in the application. The app service is
enabled across reboots and verified after enable, like nginx. Deploys
ship a server binary plus assets through the GitHub pipeline.
Browser-only libraries prove their host builds in the trial bench at
`/home/exocrat/code/leptos-ui-trials/`.
