# Model Verification System

Automated verification of language models served by external providers. Each round sends a fixed diagnostic prompt to every admitted model, records the outcome, and maintains a registry of model standing. The finished interface serves at `https://models.exosystems.net/` as a full page of Exosystems.net without carousel. Sign-in runs only on the main page; the models route recognizes the main-page session. The local system consumes results from the hosted address.

## Structure

| Part | Role |
|------|------|
| Leptos interface at `models.exosystems.net` | Dashboard with live filtering, sorting, provider management, and round operations. Full page without carousel, logically part of Exosystems.net. The model updates menu item renders only for signed-in identities holding the model-updates meadowcap delegation. |
| Boltstore database `model-updates` | Hosted store. Holds test results, the model registry, round history, and provider records. Reached over the HTTP API. |
| Willow capabilities namespace | Delegation home. Holds approvals and meadowcap delegation rows for sign-in gating and the model updates menu item. The website gate reads this store directly after sign-in. |
| Local `models.db` | Seed figures for the hosted database, carried from verified rounds. |

## Tables

`model_tests` — one row per test attempt. Columns: model identifier, provider, test prompt, success flag, response text, answer time in milliseconds, tokens used, error text, test time, batch number.

`model_registry` — one row per model. Columns: model identifier, provider, first seen, last tested, last success, consecutive failure count, activity flag, total tests, successful tests, last batch, typical and bad-day answer times (`latency_p50`, `latency_p95`), confirmed context window (`context_window`) with its source (`context_source`), and the last three failure durations (`failure_latency_1_ms`, `failure_latency_2_ms`, `failure_latency_3_ms`, most recent first).

`test_run_history` — one row per provider round. Columns: round identifier, start and completion times, provider, batch number, models tested, successes, failures, newly added models, newly flagged models, total and average answer times, round typical and bad-day times (`p50_latency_ms`, `p95_latency_ms`), error summary, notes. A row with empty completion time marks a running round; the next invocation resumes it.

`providers` — one row per provider. Columns: provider identifier, display name, endpoint address, key reference, activity flag, management notes for the settings page.

## Round Flow

1. Catalogue fetch per provider.
2. Context check. Admission requires a confirmed window of at least 262144 tokens. Figures come from the serving catalogue first and the corroborating catalogue second. Models published within the last 90 days without a published figure enter provisionally with the release date recorded, and face rechecking every round. Older figureless models stay out. Held-out models are logged with standing and figure.
3. Test set assembly from active registry models plus newly seen catalogue models, minus models already tested in the current batch.
4. Testing with parallel workers. Each result commits immediately, so an interrupted round resumes on the next invocation.
5. Registry refresh per result: failure count moves, activity flag follows the 3-failure rule, failure durations shift, timing and context figures update.
6. Round close-out in `test_run_history`, followed by the messaging alert.

## Failure Policy

`consecutive_failures` rises on each failed round and returns to zero on any success. `is_active` reads 0 once the count reaches 3. The three failure-duration columns preserve the evidence: durations near the 60-second waiting limit on all three positions indicate a model that hangs every time; shrinking durations indicate recovery; fast mixed durations indicate quick rejection. Read together with the stored error texts, these separate dead endpoints from congestion (fast HTTP 429) from overload hangs (limit timeouts).

## Providers

Alibaba Model-as-a-Service through its OpenAI-compatible endpoint. Serving catalogue at the workspace `/api/v1/models` address supplies context windows and publication dates.

NVIDIA through its integration endpoint. Context figures corroborated through the open catalogue.

OpenCode Desktop coverage reads the local OpenCode configuration. Live testing against its endpoints replaces the placeholder function as part of the free-name filter work.

## Schedule

Rounds run from the hosted site on exosystems.net. The Monday and Thursday Hermes entries retire as round execution moves to the hosted site.

## Dashboard

The dashboard serves provider multi-pick filtering, model checkboxes, three-state sorting with up and down triangles, search-as-keystroke across provider and model name, the Runs above Models stacked layout, date-only Last tested, appearance and width controls with light, dark, and system modes, and a settings page with provider management reading and writing the hosted database. Signed-in user settings travel in the user's Willow namespace and sync to every device where the identity signs in; unauthenticated visitor settings stay in browser storage only. Menu visibility follows meadowcap delegations: signed-in identities holding the model-updates delegation see the model updates item, and unauthenticated visitors never see it. The provider picker mechanism ports the oc-worker multi-select approach; appearance and width controls port the shared UI work.

## Current State

Seed figures from verified rounds: registry holds 87 models (78 Alibaba with 72 active, 9 NVIDIA with 3 active). Test history holds 558 attempts across 14 recorded rounds.
