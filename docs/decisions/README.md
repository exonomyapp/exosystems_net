# Decisions

Log of architecture decisions for exosystems.net. Newest first.
Each record holds its context and consequences; records are never
rewritten — new information arrives dated, replacements supersede.
The log is the record of why the site, the node, and the sync
between them are the way they are.

| Record | Status | Date | Deps |
|--------|--------|------|------|
| No. | Record | Status | Date | Deps |
|-----|--------|--------|------|------|
| 39 | [Provision the authenticated settings profile on the instance](provision-the-authenticated-settings-profile.md) | Proposed | 2026-10-22 | R23, R33 |
| 38 | [Self-host exosystems.net iroh relay for network isolation](self-host-iroh-relay.md) | Proposed | 2026-10-16 | R13 |
| 37 | [Migrate the Alibaba Singapore instance off CentOS 7](migrate-alibaba-singapore-off-centos-7.md) | Proposed | 2026-10-09 |  |
| 36 | [Deploy the ExoMetro node on the Alibaba Singapore instance](deploy-exometro-node-on-alibaba-singapore.md) | Proposed | 2026-10-09 |  |
| 35 | [Implement hardening on the Alibaba Singapore instance](implement-hardening-on-alibaba-singapore.md) | Accepted | 2026-10-09 |  |
| 34 | [Implement hardening on the Tencent Lighthouse instance](implement-hardening-on-tencent-lighthouse.md) | Accepted | 2026-10-09 |  |
| 33 | [Shape the Settings page as the identity and mesh console](shape-settings-page-as-identity-and-mesh-console.md) | Proposed | 2026-10-09 | R2, R6, R7, R10, R17, R19, R25, R27, R32 |
| 32 | [Application-layer notifications over Willow entries and Iroh gossip](application-layer-notifications-over-willow-gossip.md) | Proposed | 2026-10-08 |  |
| 31 | [Serve Model Updates data from boltstore on exosystems.net](serve-model-data-from-hosted-sqlite-endpoint.md) | Proposed | 2026-10-08 |  |
| 30 | [Deploy exo-mcp with DBOS and Postgres to Lighthouse](deploy-exo-mcp-with-dbos-and-postgres-to-lighthouse.md) | Proposed | 2026-10-02 |  |
| 29 | [Install nexo-rs on Lighthouse](install-nexo-rs-on-lighthouse.md) | Proposed | 2026-10-02 |  |
| 28 | [Integrate the exo-mcp memory gateway into the tooling workflow](integrate-exo-mcp-memory-gateway-into-tooling.md) | Proposed | 2026-09-30 |  |
| 27 | [Discover joined nodes after sign-in](discover-joined-nodes-after-sign-in.md) | Proposed | 2026-09-29 |  |
| 26 | [Verify replication on each Archon upgrade](verify-replication-on-each-archon-upgrade.md) | Proposed | 2026-09-29 |  |
| 25 | [Route tool storage by active identity](route-tool-storage-by-active-identity.md) | Proposed | 2026-09-29 |  |
| 24 | [Replicate Archon rows to the Willow store](replicate-archon-rows-to-willow-store.md) | Proposed | 2026-09-29 |  |
| 23 | [Exosystems email services](exosystems-email-services.md) | Proposed | 2026-10-21 |  |
| 22 | [Send triggered administrative emails from the node](send-triggered-administrative-emails-from-node.md) | Proposed | 2026-09-29 |  |
| 21 | [Plan repeatable deployment of the three Conscia flavors](plan-repeatable-deployment-of-three-flavors.md) | Proposed | 2026-09-29 |  |
| 20 | [Resolve two-device write conflicts by union](resolve-two-device-write-conflicts-by-union.md) | Proposed | 2026-09-29 |  |
| 19 | [Set synchronisation cadence beyond sign-in and reconnect](set-sync-cadence-beyond-sign-in.md) | Proposed | 2026-09-29 |  |
| 18 | [Run the temporary hand-provisioned node](run-temporary-hand-provisioned-node.md) | Proposed | 2026-09-29 |  |
| 17 | [Set operator policy with a management interface](set-operator-policy-and-management-ui.md) | Proposed | 2026-09-29 |  |
| 16 | [Build fresh boot and sign-in on the node store](build-fresh-boot-and-sign-in-on-node-store.md) | Proposed | 2026-09-29 |  |
| 15 | [Define the node identity store schema](define-node-identity-store-schema.md) | Proposed | 2026-09-29 |  |
| 14 | [Define the plain-node scope with discovery](define-plain-node-scope-with-discovery.md) | Proposed | 2026-09-29 |  |
| 13 | [Migrate the networking library to the 1.x series](migrate-networking-library-to-1x-line.md) | Proposed | 2026-09-28 | R12 |
| 12 | [Choose the node database for identity state and Willow content](choose-node-database-for-identity-and-willow-state.md) | Proposed | 2026-09-28 |  |
| 11 | [Conscire the local ExoTalk build with the instance node and prove sync](conscire-local-exotalk-with-instance-node.md) | Proposed | 2026-09-28 | R7 |
| 10 | [Grant document access per identity from the docs view](grant-document-access-per-identity.md) | Proposed | 2026-09-28 |  |
| 9 | [Style the site bespoke over accessible behavior primitives](style-bespoke-over-behavior-primitives.md) | Proposed | 2026-09-28 |  |
| 8 | [Require a decision record with watched code changes](require-decision-record-with-code-change.md) | Proposed | 2026-09-27 | R3 |
| 7 | [Keep a local synced Willow store per signed-in identity](keep-local-synced-willow-store.md) | Proposed | 2026-09-27 | R2, R4, R6 |
| 6 | [Recover and move ExoTalk profile across devices](recover-and-move-exotalk-profile-across-devices.md) | Proposed | 2026-09-27 |  |
| 5 | [Prove secret ownership at sign-in](prove-secret-ownership-at-sign-in.md) | Proposed | 2026-09-27 |  |
| 4 | [Run full Conscia node on exosystems.net](run-full-conscia-node-on-exosystems-net.md) | Proposed | 2026-09-27 |  |
| 3 | [Set up GitHub repository and CI pipeline](set-up-github-repo-and-ci-pipeline.md) | Proposed | 2026-09-27 |  |
| 2 | [Generate peer identity early and sign in with it](exosystems-exoauth.md) | Accepted | 2026-09-26 | R4, R5, R6, R7, R9, R11 |
| 1 | [Use Leptos 0.8 with server rendering](use-leptos-0-8-with-ssr.md) | Accepted | 2026-09-26 |  |

## Notation

The **Deps** column names the other decision records a record rests on, each
written R followed by that record's number in the No. column — R2 is record
2. A record's own number appears in the No. column.

The five significant requirements are **SR1 to SR5** and are cited that way in
the records and in the fitness checks. R followed by a number always means a
decision record; SR followed by a number always means a requirement. The two
never share a token.

## Companion documents

Not decision records. Supporting views over the log:

| Document | When to read it |
|----------|-----------------|
| [Significant requirements](significant-requirements.md) | To see the five pass/fail tests (SR1–SR5) every record must satisfy |
| [Pending decisions](pending.md) | To see the seven open decisions, what unblocks each, and where each lands |
| [System context sheet](diagrams/context.html) | To see people, site, node, and local client on one drafting sheet (C4-01) |
| [Container sheet](diagrams/container.svg) | To see the ten inner containers and their flows (C4-02) |
