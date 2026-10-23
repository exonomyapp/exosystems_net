# Run a full Conscia node on exosystems.net

## Status

Proposed

## Context

Conscia nodes are utility peers in the Exosystem mesh providing opt-in helper services: relay, indexing, discovery, persistence, authentication, observability, and federation. Flavor-specific features (ExoMetro, ExoSocial) are out of scope; all core Conscia functionality is in scope.

SQLite is the database for all node state, per Spec 67 (Conscia Web Application Suite).

Willow is the content-addressed sync layer. User content persists and syncs through Willow namespaces. The node holds encrypted content for conscired users and forwards it on connection.

## Decision

The node implements the full Conscia feature set.

**Identity and Access:**
- Own did:peer (conscia identity)
- Meadowcap capability system: Ed25519-signed delegations `{delegator, delegatee, namespace, permission, signature}` exchanged over Iroh gossip and HTTP
- Operator approval via ConSoul admin interface

**Storage:**
- Willow namespaces for all user content (encrypted, content-addressed, synced)
- SQLite for conscired identities, capability records, operator settings, publishable metadata index, routing events, and orchestration telemetry

**Networking:**
- Iroh for peer connections (encrypted, hole-punching, relay fallback)
- Full federation protocol: gossip topics for heartbeat, catalog, index sync, governance, geo updates, health
- Geographic and latency-aware routing
- HTTP API for outside apps (conscire endpoint; runs the join flow in Spec 11 §11.4)

**Services:**
- Relay: hold and forward encrypted content for conscired users
- Indexing: blind indexing of publishable metadata
- Discovery: search local and federated metadata, merge results from live queries
- Persistence: Willow-backed always-present mirror for whitelisted users
- Authentication: verify did:peer ownership via challenge-response, issue session tokens
- Observability: Prometheus metrics, health telemetry, federated health exchange
- Federation: mutual peering, capability exchange, gossip channels

**Database Services (Spec 41):**
- Blind indexing: publishable metadata only
- Curation: signature verification, schema validation, normalization before indexing
- Operator controls: indexing toggles, local/federated/global metadata acceptance, retention limits, allow/deny lists
- Privacy: metadata minimization, separate publish-to-peers and publish-globally toggles
- Updates/retractions: publishers withdraw metadata, nodes remove after sync

**High Availability (Spec 6):**
- Lifeline service for whitelisted users (always-present mirror)
- Disaster recovery posture
- Federated load-balancing via health telemetry

**Traffic Control (Spec 14):**
- Rate limiting
- Namespace-level access control

**Mesh Delivery Guarantees (Spec 16):**
- Content delivery across the mesh
- Offline tolerance
- Sync reconciliation

**Internationalization (Spec 15):**
- Language routing
- Locale-aware metadata

**Solid Front Door (Spec 17):**
- Node landing page
- Public identity
- Available services listing

**Web Applications (Spec 67):**
- conscia_web: public dashboard
- conscierge_web: chat interface
- consoul_web: admin interface

## Consequences

Full Conscia functionality on a single instance. Users receive relay, indexing, discovery, persistence, and federation. Outside apps conscire for capabilities under the owner's policies. The node participates as a full peer in the mesh.

Implementation surface includes Willow, Iroh, SQLite, the federation protocol, all services, and all web applications. The node runs as a daemon and requires operator management of conscire requests and settings.
