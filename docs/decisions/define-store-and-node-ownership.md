# Define store and node ownership, and the meaning of OWNER_DID

## Status

Proposed

## Context

Ownership in the exosystem belongs to Willow stores and to Conscia nodes. A user identity owns its own Willow store. A Conscia node owns its own store, created by its creator identity. The two are separate: an identity's store and a node's store are different things, and owning one does not imply owning the other.

Signing in to the application verifies a `did:peer` and opens that identity's Willow store. This requires only the iroh mesh and the Willow store; ExoTalk and Conscia are applications that build on top of the verified identity and are not prerequisites for sign-in. The exosystem supports two identity kinds today: `exotalk` for users and `conscia` for non-user nodes. Applications (ExoTalk, ExoSoul, Conscia) and exoapps (Exonomy, Exocracy, RepubLet, and others) consume the verified identity and store; they are not part of sign-in.

The instance configuration carries `OWNER_DID`, the identity of the person who deployed the Conscia node on that instance. On an empty store, that identity gains operator surfaces (settings, petitions, approvals) so the instance owner can manage the node before any joins are approved. This is a property of that specific identity on that specific instance.

When ExoTalk is built, it will carry tools to define, provision, deploy, and install new Conscia nodes on instance targets. Each such node will have its own store, owned by its creator identity. The creator's own store and the node's store remain distinct.

## Decision

1. An identity owns its Willow store. Ownership is inherent to the identity and needs no separate record; the store is the identity's.
2. A Conscia node is a `conscia`-kind identity that owns its own store, and it has its own `did:peer` distinct from its creator's. The node's store is created and owned by its creator identity, which may be the same identity as the instance owner or a different one. The node operates on its creator's authority: it acts per its owner's dictates, as if it were the owner of its own store, through Meadowcap capability delegations from the creator identity to the node identity. The node's own identity is separate and does not merge with the creator's identity.

3. `OWNER_DID` names the identity of the instance owner. On an empty store, that identity gains operator surfaces. This is the instance owner's own store receiving operator access, not a user becoming an owner.
4. Signing in to the application does not make the signing-in identity an owner of anything beyond its own store. It never grants operator surfaces on a node or on another identity's store.
5. Consciring is a separate action from signing in. The application performs it on the user's behalf only after the user's Willow store context exists, presenting the user's `did:peer` to the node's conscire endpoint for the node owner's approval under the node's own policy.

The data model needs no new ownership column. Ownership is expressed by the identity kind (`exotalk` or `conscia`) and by which identity signs in matching `OWNER_DID` on an empty store. A node's store is a separate artifact owned by its creator identity, recorded when the node is provisioned.

## Consequences

The concept of "owner" is unambiguous: it refers to the identity named in `OWNER_DID` on a given instance, acting on its own store. No user becomes an owner by signing in. The bootstrap that grants operator surfaces on an empty store applies only to that identity on that instance.

The distinction between an identity's store and a node's store stays explicit in code and documentation, so provisioning tools (such as those ExoTalk will carry) can create node stores owned by their creator identity without conflating them with the creator's personal store.

## Addendum 2026-10-23 — ExoMetro session management and asynchronous transformation

ExoMetro uses three pgEdge nodes with distinct roles. `exonode` is the external gateway: all data ingress and egress for the Conscia node, primary interface for remote federation sync, and primary write target for Conscia mesh traffic. `endonode` is the internal community core: indices, curated reflections, and community-internal data, serving read queries for the local community. `nextnode` is the high-ready standby: it mirrors both exonode and endonode for failover and can assume either role on failure.

exonode and endonode do not share data. The social interface (community content consumption) works mostly with endonode; the mesh interface (iroh/Willow traffic) works mostly with exonode. Information ingested by exonode is processed and transformed into publicly consumable content, which is persisted in endonode for posterity. exonode functions as a queue for arrived content and does not retain raw ingested content after processing. endonode acts as Conscia's social store, losing content only when authors remove it or Conscia's rules do.

This is transformation, not replication. There is no need to replicate: users already hold their own raw content and can see everything their Meadowcap capabilities allow. What Conscia adds that other peers cannot is a community perspective — a distilled collected consciousness. Raw community content is transformed into publicly consumable content and persisted in endonode. This is conceptually similar to a transactional database sourcing a data warehouse, but where a data warehouse is built for analytics, endonode supports relations, associations, references, and referrals that point back to users' own Willow stores — not exonode — as their source. endonode also stashes socially relevant information inferred from Willow content across space and time, preserving Conscia's inferences for user benefit and feeding Conscia's AI agents with raw data seasoned with historically perceived and persisted understanding.

Social content (vouchers, projects, scientific reports, collaboratively composed literature, and other ExoApp-created content) streams in through Conscia's sync action, gets routed through NATS to exonode, which functions as a queue. The external worker transforms it into publicly consumable content and persists it in endonode. Since Conscia's own Willow store already persists the original content, exonode does not retain the raw ingested content after processing.

**How transformations are handled (verified against PostgreSQL 18 and Spock 6).** Spock does not transform data during replication; it replicates rows as-is with optional row filtering, and Spock provides traditional replication services for nextnode's failover readiness — keeping nextnode fresh with all rows inserted into both exonode and endonode as fast as PostgreSQL supports. Transformation follows an asynchronous, fire-and-forget pattern with transactional safety: the trigger (PL/pgSQL) performs a minimal transactional write — inserting a job record into a queue table — and issues a `NOTIFY` on a specific channel. `NOTIFY` is non-blocking and is only delivered after the transaction commits; a rollback never sends it, so the external agent is never called spuriously. An external worker — Conscierge, oc-worker, or ExoMCP, educated to participate in this trigger lifecycle — maintains a persistent connection to exonode, `LISTEN`ing on that channel. On notification it reads the queue table and executes its agentic work: transforming the raw ingested content into an SQL insert statement it fires into endonode. The worker runs in its own process, not under PostgreSQL's user ID, and Conscierge runs as a sidecar apart from Conscia so it can grab its own RAM and CPU on demand; finishing the ingestion transaction frees database resources for more ingestion while the agent transforms in its own process. For simpler micro-transformations the trigger can handle on its own, exonode's insert trigger can call a database function or procedure on endonode via `dblink_send_query()`, which dispatches the query and returns immediately without waiting for a result; this serves database queries and does not shell out to an OS-level agent.

Conscia's primary role is to share content. The database architecture and operational model support thinking and exploring, not calculating and analyzing. Conscia aspires to be a Social Operating System, extending the vision that operating systems are better founded upon databases.

Session management in ExoMetro is handled by `nextnode`. nextnode continuously reads both exonode and endonode and writes to itself to stay prepared, giving it spare resources for session management while exonode and endonode handle real-time streaming content. Sessions live on nextnode. Conscia routes session traffic to nextnode. exonode and endonode do not hold session data. When nextnode assumes exonode's or endonode's role on failure, session management continues uninterrupted.

Records that touch ownership (`exosystems-exoauth.md`, `build-fresh-boot-and-sign-in-on-node-store.md`, `provision-the-authenticated-settings-profile.md`) reference this record for the definition.
