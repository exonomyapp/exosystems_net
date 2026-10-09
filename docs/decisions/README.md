# Decisions

Log of architecture decisions for exosystems.net. Newest first.
Each record holds its context and consequences; records are never
rewritten — new information arrives dated, replacements supersede.
The log is the record of why the site, the node, and the sync
between them are the way they are.

| Record | Status | Date |
|--------|--------|------|
| Record | Status | Date | Deps |
|--------|--------|------|------|
| [Migrate the Alibaba Singapore instance off CentOS 7](migrate-alibaba-singapore-off-centos-7.md) | Proposed | 2026-10-09 |  |
| [Deploy the ExoMetro node on the Alibaba Singapore instance](deploy-exometro-node-on-alibaba-singapore.md) | Proposed | 2026-10-09 |  |
| [Implement hardening on the Alibaba Singapore instance](implement-hardening-on-alibaba-singapore.md) | Accepted | 2026-10-09 |  |
| [Implement hardening on the Tencent Lighthouse instance](implement-hardening-on-tencent-lighthouse.md) | Accepted | 2026-10-09 |  |
| [Shape the Settings page as the identity and mesh console](shape-settings-page-as-identity-and-mesh-console.md) | Proposed | 2026-10-09 |  |
| [Application-layer notifications over Willow entries and Iroh gossip](application-layer-notifications-over-willow-gossip.md) | Proposed | 2026-10-08 |  |
| [Serve Model Updates data from boltstore on exosystems.net](serve-model-data-from-hosted-sqlite-endpoint.md) | Proposed | 2026-10-08 |  |
| [Deploy exo-mcp with DBOS and Postgres to Lighthouse](deploy-exo-mcp-with-dbos-and-postgres-to-lighthouse.md) | Proposed | 2026-10-02 |  |
| [Install nexo-rs on Lighthouse](install-nexo-rs-on-lighthouse.md) | Proposed | 2026-10-02 |  |
| [Integrate the exo-mcp memory gateway into the tooling workflow](integrate-exo-mcp-memory-gateway-into-tooling.md) | Proposed | 2026-09-30 |  |
| [Discover joined nodes after sign-in](discover-joined-nodes-after-sign-in.md) | Proposed | 2026-09-29 |  |
| [Verify replication on each Archon upgrade](verify-replication-on-each-archon-upgrade.md) | Proposed | 2026-09-29 |  |
| [Route tool storage by active identity](route-tool-storage-by-active-identity.md) | Proposed | 2026-09-29 |  |
| [Replicate Archon rows to the Willow store](replicate-archon-rows-to-willow-store.md) | Proposed | 2026-09-29 |  |
| [Set up the Brevo mail service for node email](set-up-brevo-mail-service.md) | Proposed | 2026-09-29 |  |
| [Send triggered administrative emails from the node](send-triggered-administrative-emails-from-node.md) | Proposed | 2026-09-29 |  |
| [Plan repeatable deployment of the three Conscia flavors](plan-repeatable-deployment-of-three-flavors.md) | Proposed | 2026-09-29 |  |
| [Resolve two-device write conflicts by union](resolve-two-device-write-conflicts-by-union.md) | Proposed | 2026-09-29 |  |
| [Set synchronisation cadence beyond sign-in and reconnect](set-sync-cadence-beyond-sign-in.md) | Proposed | 2026-09-29 |  |
| [Run the temporary hand-provisioned node](run-temporary-hand-provisioned-node.md) | Proposed | 2026-09-29 |  |
| [Set operator policy with a management interface](set-operator-policy-and-management-ui.md) | Proposed | 2026-09-29 |  |
| [Build fresh boot and sign-in on the node store](build-fresh-boot-and-sign-in-on-node-store.md) | Proposed | 2026-09-29 |  |
| [Define the node identity store schema](define-node-identity-store-schema.md) | Proposed | 2026-09-29 |  |
| [Define the plain-node scope with discovery](define-plain-node-scope-with-discovery.md) | Proposed | 2026-09-29 |  |
| [Migrate the networking library to the 1.x series](migrate-networking-library-to-1x-line.md) | Proposed | 2026-09-28 |  |
| [Choose the node database for identity state and Willow content](choose-node-database-for-identity-and-willow-state.md) | Proposed | 2026-09-28 |  |
| [Conscire the local ExoTalk build with the instance node and prove sync](conscire-local-exotalk-with-instance-node.md) | Proposed | 2026-09-28 | R4 |
| [Grant document access per identity from the docs view](grant-document-access-per-identity.md) | Proposed | 2026-09-28 | R1 |
| [Style the site bespoke over accessible behavior primitives](style-bespoke-over-behavior-primitives.md) | Proposed | 2026-09-28 | R3 |
| [Require a decision record with watched code changes](require-decision-record-with-code-change.md) | Proposed | 2026-09-27 |  |
| [Keep a local synced Willow store per signed-in identity](keep-local-synced-willow-store.md) | Proposed | 2026-09-27 | R4 |
| [Recover and move ExoTalk profile across devices](recover-and-move-exotalk-profile-across-devices.md) | Proposed | 2026-09-27 |  |
| [Prove secret ownership at sign-in](prove-secret-ownership-at-sign-in.md) | Proposed | 2026-09-27 | R5 |
| [Run full Conscia node on exosystems.net](run-full-conscia-node-on-exosystems-net.md) | Proposed | 2026-09-27 | R4 |
| [Set up GitHub repository and CI pipeline](set-up-github-repo-and-ci-pipeline.md) | Proposed | 2026-09-27 |  |
| [Generate peer identity early and sign in with it](exosystems-exoauth.md) | Accepted | 2026-09-26 | R1 |
| [Use Leptos 0.8 with server rendering](use-leptos-0-8-with-ssr.md) | Accepted | 2026-09-26 |  |
## Companion documents

Not decision records. Supporting views over the log:

| Document | When to read it |
|----------|-----------------|
| [Significant requirements](significant-requirements.md) | To see the five pass/fail tests (R1–R5) every record must satisfy |
| [Pending decisions](pending.md) | To see the seven open decisions, what unblocks each, and where each lands |
| [System context sheet](diagrams/context.html) | To see people, site, node, and local client on one drafting sheet (C4-01) |
| [Container sheet](diagrams/container.svg) | To see the ten inner containers and their flows (C4-02) |
