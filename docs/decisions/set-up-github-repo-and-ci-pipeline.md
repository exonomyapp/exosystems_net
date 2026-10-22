# Set up GitHub repository and CI pipeline

## Status

Proposed

## Context

The project resides at `/home/exocrat/code/exosystems_net/` as a git repository. The `gh` CLI is authenticated as the project account with repo and workflow scope. The site is a Leptos 0.8 SSR application that builds with `cargo leptos build` and serves behind nginx on the Tencent instance.

The build produces a server binary, a browser bundle, and static assets under `target/site/`.

## Decision

Create the GitHub repository and a GitHub Actions workflow. On push to main, the workflow builds the Leptos app with `cargo leptos build --release`, runs tests, and deploys to the Tencent instance via SSH. On pull request, the workflow builds and tests.

The workflow uses `actions/checkout@v4`, `dtolnay/rust-toolchain@stable` with the wasm target, `cargo-leptos` via `cargo binstall`, and SSH deployment with a deploy key stored in GitHub Secrets. Deployment copies the server binary and `target/site/` via rsync, restarts the app service, and verifies HTTP 200.

## Consequences

Every push to main is automatically built, tested, and deployed. Pull requests are validated before merge. The workflow and deploy key require maintenance. The instance is reachable from GitHub Actions runners.

## Addendum 2026-09-27 — fitness script runs in CI

`scripts/fitness.sh` (executable, passing 4/4) runs in the workflow
on pull request and on push to main, before build: F1 motion-free
server sources, F2 two-identity allowlist seed, F3 decision index
completeness, F4 sign-in and gated-docs routes present. A failing
check stops the run so broken decisions never deploy. The live
browser proofs (SR1–SR3, SR5 in `significant-requirements.md`) stay
manual until the pipeline owns a browser runner; wiring that runner
is a follow-on to row 2 of `pending.md`.

## Addendum 2026-10-22 — the repository and the pipeline are live

The repository is `exonomyapp/exosystems_net` and the workflow `.github/workflows/deploy.yml` runs on every push to `main`: the fitness checks, a release build of the server binary and the browser bundle, an artifact of the binary, the site folder and `allowed_dids.json`, and a deployment to the instance that restarts the service and verifies the response. The first green deployment ran on 2026-10-21, and the run of 2026-10-22 built commit `47eae3b` and restarted the service on the instance. The deployment ships the binary, the site folder and the allowlist file, so the nginx configuration held in the repository's `deploy/nginx/` folder stays reference material on the operator's machine and does not reach the host.
