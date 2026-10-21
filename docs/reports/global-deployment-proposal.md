# Report: Global Deployment Proposal — ExoSystem

- Status: Proposal
- Deciders: project owner
- Date: 2026-10-19

## 1. Executive Summary

The Exosystem comprises three deliverables. ExoSoul is the authoring tool: it produces blueprint sets that define the interface elements and behaviour of ExoApps. ExoTalk is the application environment in which subscribed-to ExoApps are unfolded and in which the user works. Conscia is a peer technology that supplies the service roles the ecosystem requires — relay, persistence, discovery, indexing, notification, blueprint distribution, and the public and administrative web surfaces of its own operation. ExoSoul, ExoTalk and Conscia are the product line's core deliverables; the ExoApps they carry are configured artefacts of that line.

The three share one substrate. A node holds a `did:peer` identity, reaches its peers over iroh, and stores and synchronises its data in Willow. Identity, authorisation and content remain peer-held throughout. No component consults a central authority to establish identity or to grant access. The design intent recorded in the specification corpus is that the system continues to operate when operated infrastructure is unavailable.

This proposal addresses deployment at scale. It introduces a two-plane model separating the distribution of public, verifiable artefacts from the flow of private data, and a role-based deployment model in which one service artifact is deployed many times with differing role assignments. Three consequences follow.

**A global seed and a regional mesh.** Public artefacts — blueprint sets, application version manifests, the relay map, the lifeline discovery list, and native application packages — are distributed from edge delivery networks and object storage under write-once retention. Private data, identity and authorisation remain inside the mesh. The planes meet at one point: the first fetch a node performs, after which all further synchronisation proceeds peer to peer.

**A relay fleet as first-order infrastructure.** Native nodes reach each other directly wherever the network permits. A browser node cannot open a raw socket to an arbitrary address, so every browser connection to the mesh traverses a relay. A relay fleet is therefore the precondition for a browser peer, and a browser peer is the precondition for an installation-free ExoTalk. Relay placement also determines the path length of every connection that fails to punch through, and therefore the latency a user experiences on the hardest networks.

**Two classes of lifeline, with independent failure modes.** An operated lifeline is a Conscia peer in continuous service, publicly discoverable, selected by clients using the existing peer-selection criteria of region, locality, connectivity profile and measured latency. A participant lifeline is an ExoTalk node acting on behalf of a peer that cannot reach an operated lifeline. The two classes cover each other's failure.

The model is stated in vendor-neutral terms in §2, with a derived requirement set in §2.9. Eight vendor annexes in §3 assess one operator each against that requirement set. §4 proposes a recommended pairing. §5 consolidates the evidence.

## 2. Deployment Model

### 2.1 Roles

Conscia is a peer that offers service roles to the mesh. The specification corpus records this as the Decoration Principle: services belong to no application, are offered by Conscia nodes, and are decorated onto whichever application benefits; the node operator selects which roles a node enables. The role set comprises relay, persistence and continuity (lifeline), blueprint publication, notification, discovery and indexing, and the node's own web surfaces.

Roles are a deployment-time selection on one artifact. A node's role set is its configuration. The three Conscia flavours — Satellite, ExoSocial and ExoMetro — are named bundles of roles with additional tooling, and remain bundles of roles on one artifact.

The deployment model holds one service artifact and one `did:peer` per node instance. A peer is the unit of trust, key custody, mesh reachability and discovery, and splitting roles into separate artifacts would multiply identities, key custody and mesh presence, and would leave the question of which process holds the lifeline role unresolved. The one decomposition the specification corpus already anticipates is Conscierge, which runs as an in-process role by default with a sidecar or separate-pod pattern available where a security or ownership boundary requires it.

### 2.2 The two planes

The **seed plane** is the set of artefacts a node obtains before it holds any data of its own: the baseline ExoUI blueprint set, the application version manifest, the relay map, the lifeline discovery list, and the native application packages. The seed plane is public, digest-pinned and signed, changes rarely, and is read at a scale far exceeding its write rate.

The **private plane** is everything a node holds and everything it exchanges with peers: identity, keys, messages, Willow content, delegations, and the blueprint subscriptions it operates. The private plane travels over the mesh.

The two planes are separated by function, by trust and by infrastructure. Edge delivery networks and object storage serve the seed plane. Peers serve the private plane. The public plane gains caching, because a cached artefact is verified by digest on arrival, and a stale or substituted copy fails verification and is discarded. The private plane remains peer to peer, because an intermediary would hold authority the architecture assigns to peers.

### 2.3 The seed plane

When a node starts for the first time it requires a small set of things from the network: the baseline blueprint set, the current application versions, the map of available relays, and the list of lifeline peers. It fetches these once, verifies each against its digest and signature, and retains it. Every subsequent operation — message delivery, synchronisation, blueprint subscription, discovery — proceeds between peers.

The seed plane comprises everything a node fetches before it holds anything of its own. The private plane comprises everything a node holds afterwards. Edge delivery serves the first. The mesh serves the second.

The seed plane carries a further property that determines its treatment in distribution. A blueprint is authored on behalf of an ExoApp by an author identified by a `did:peer`. The author chooses what the blueprint exposes. An author may hold blueprints in a personal Willow namespace and grant a Conscia node the capability to publish them, or may author them as a Conscia node the author controls, publishing under that node's identity without disclosing the author's own. Either way, visibility is a property of the authored artefact, and the distribution layer reproduces the author's choice and forms no policy of its own. Global caching of the seed plane is therefore compatible with authorial control of exposure, because the cached copy is the author's own signed artefact and the cache holds no additional capability over it.

### 2.4 Lifelines

Two classes of lifeline exist, and each covers the failure the other cannot.

A participant lifeline is an ExoTalk node acting on behalf of a peer that cannot reach an operated lifeline. It is available exactly when a user is present, a condition no operator can schedule and no operator can guarantee. It is the class that carries the network through the loss of operated infrastructure.

An operated lifeline is a Conscia peer in continuous service, publicly discoverable, selected by clients on region, locality, connectivity profile and measured latency. It is available exactly when an operator keeps it running, a condition no user controls. It is the class that carries the network through the absence of users, and the only class able to bootstrap a new identity.

A deployment funding only the operated class depends on continued operator presence. A deployment funding only the participant class depends on continued user presence and cannot bootstrap an identity. Funding both removes the dependence on either.

Operated lifelines are equal participants, with client-side selection and client-side failover. Selection follows the peer-selection criteria already in the specification corpus, extended with the regions in §2.8. No election, primary designation or master role is involved, and the discovery list that names them is a seed-plane artefact.

### 2.5 Relays

A relay is a small service that accepts connections from nodes unable to reach each other directly and forwards encrypted traffic between them. Relay traffic is end-to-end encrypted, and a relay cannot read what it forwards. iroh requires outbound TCP on port 443 to a relay, upgrading the connection to a WebSocket, and treats a relay as the path of last resort when a direct path cannot be established. A relay requires a public address, a resolvable name and a certificate.

Relays are the workload with the highest spatial sensitivity in the system. Native nodes that can punch through connect directly, and their traffic never reaches a relay. The connections that do reach a relay are those behind symmetric NATs, and for those the relay's distance from the user is the whole of the perceived latency.

Two properties of the relay protocol shape the deployment. A relay connection is a single long-lived TCP stream, which permits many relays to be addressed from one published name, and which makes a relay address a poor candidate for an edge delivery network, since edge networks terminate and re-establish connections. Relays are therefore addressed directly at their regional addresses, and the published relay map names each address. The second property follows from the first: because a node's relay is reached directly, the failure of one edge operator removes no part of the relay plane.

Relay fleets begin on the project's own infrastructure. The first relay is the primary domain, the second relay is the Exocracy domain, and operator-hosted relays in further regions follow the shape established by those two.

### 2.6 The browser peer

iroh compiles to WebAssembly for browser targets, and the gossip protocol supports browser builds. A browser node reaches the mesh through a relay, since the browser sandbox prevents it from opening a socket to an arbitrary address. A browser peer is therefore fully capable of participating in the mesh, and the traffic it exchanges with native nodes remains end-to-end encrypted throughout the relay.

Two modes of operation exist, differing in key custody.

In the **persistent mode** the browser holds a `did:peer` key pair in browser storage and operates as a peer in its own right, with its own identity, its own Willow namespace, and its own participation in gossip and blob transfer. The browser can author, receive private messages, and hold the full capability of a native node.

In the **delegated mode** the browser holds no key material and renders the copy that a Conscia node has synchronised on the user's behalf, reaching that node over the mesh as a peer the browser does not itself implement. The browser holds read access to the user's own data and holds no private key.

The two modes share one build and differ in custody. The persistent mode grants full capability at the cost of holding a private key in browser storage. The delegated mode grants read access with no key in the browser, at the cost of requiring a reachable Conscia node. Both modes require relay connectivity, and both are served by the same application bundle, which the seed plane delivers.

The consequence for deployment is direct. Every user can reach their own ExoTalk from a browser with nothing installed, and the installation-free path and the installed path reach the same data because both are peers of the same identity.

### 2.7 Native binary distribution

Native packages are distributed through the channels already recorded in the specification corpus: Flatpak and Flathub, AppImage and direct download, disk images with automatic update, MSIX and EXE through Winget, APK and AAB through F-Droid and Play, IPA through TestFlight and the App Store, Homebrew, npm and Bun packages, an installation script, and container images on a public registry.

These packages belong to the seed plane by the definition in §2.3, and their delivery characteristics are the same as the other seed artefacts: high read volume, low write volume, and a stable versioned address. Two properties follow. A package is verifiable by digest or signature at download, which permits indefinite caching at the edge. A package is large relative to the other seed artefacts, which places the requirement for pre-warming and for a served-expired policy on the delivery path.

### 2.8 Regions

The specification corpus records region, locality and connectivity profile as peer metadata, and gives eu-central, af-east, na-west and sea as the example values. The deployment model adopts those four regions as the operating footprint:

| Region | Purpose |
|---|---|
| Europe | Operated lifeline, relay, and web tier |
| Africa | Operated lifeline, relay, and web tier |
| North America | Operated lifeline, relay, and web tier |
| Southeast Asia | Operated lifeline, relay, and web tier |

Each region requires a compute presence, since the operated lifeline and the relay are stateful and long-running services. A region whose operator offers edge points of presence without a compute region can serve the seed plane and can accelerate a web tier hosted elsewhere, and it cannot host an operated lifeline or a relay in that region. This distinction is material to the pairing in §4, and it is assessed per operator in the annexes.

### 2.9 Derived requirements

The following requirements are stated in general technical terms and are used to assess each operator in §3.

**R1 — Edge delivery.** A global delivery network with anycast or equivalent traffic distribution, covering the four regions of §2.8, with a declarative rules engine able to express: browser and edge cache lifetimes; per-status-code cache lifetime; cache-key composition; cache bypass; artefact pre-warming; serving of expired content during an origin outage; origin selection by client geography; redirection by client geography; origin host, port and server-name override; DNS-record override for origin routing; web application firewall policy; multi-origin failover; and WebSocket passthrough to origin.

**R2 — Regional compute.** A platform able to host a long-lived HTTP server that upgrades to WebSocket, with a client connection lifetime measured in hours, a per-connection payload rate sufficient for mesh traffic, and a scale-to-zero option for stateful instances.

**R3 — Immutable storage.** Object storage with write-once retention, suitable for digest-pinned artefact sets.

**R4 — Authoritative DNS.** Geolocation-based and latency-based routing with health-check failover, at a granularity reaching individual countries.

**R5 — Regional presence.** A compute region in each of the four regions of §2.8.

**R6 — Shared configuration store.** A global key-value store, or a regional store with documented replication, for the relay map and the lifeline discovery list.

**R7 — Edge logic.** A programmable application edge runtime, or a declarative configuration surface that removes the requirement for one.

**R8 — Operability.** Programmatic control of the above, with quotas sufficient for the expected rule, origin and function counts.

## 3. Vendor Annexes

Each annex assesses one operator against R1 to R8 of §2.9, and stands alone.

### Annex A — Alibaba Cloud

**R1 — Edge delivery.** Edge Security Acceleration operates over 3,200 points of presence globally, of which more than 2,300 cover the 31 provincial regions of the Chinese mainland, with 180 Tbit/s of total bandwidth. Traffic is directed by anycast. One rules engine serves cache, redirection, compression, origin fetch and firewall policy through a shared expression syntax, and a rule consists of an expression and an action. Rule quotas by plan are 10, 25, 50 and 125 for cache, transformation, redirection, origin fetch and security rules, and 10, 25, 50 and 100 for network optimisation, which carries the WebSocket action. Browser cache lifetime, edge cache lifetime and per-status-code cache lifetime are separate rule actions, with the status-code lifetime adjustable from one second to one year. Cache-key composition admits query string, request header, cookie, device type and country or region. Cache bypass, artefact pre-warming, and serving of expired content during an origin outage are available. Origin selection by client geography is available through load-balancer geo steering, and redirection by client geography through a country or region match. Origin rules expose custom origin host, protocol and port, and origin server-name indication, and a DNS-record override routes requests for a path prefix to a different origin. Firewall policy covers managed rule sets, rate limiting, bot management and access rules. Multi-origin failover is available through load-balancer policies with per-origin weighting and health checks. WebSocket passthrough is enabled by default on ports 80 and 443, and is unavailable on the entry tier.

**R2 — Regional compute.** Function Compute offers three delivery modes. A custom runtime is a package containing an HTTP server, bound to a platform-assigned port, ready within 120 seconds, supporting keep-alive, with a request timeout of 15 minutes or longer. A custom container is an image containing an HTTP server listening on the assigned port, with a 120-second start limit and a 512 MB or 10 GB writable layer. A native WebSocket protocol is available on an HTTP-triggered function, and the connection inherits the function's execution timeout, which extends to 86,400 seconds. Instances are elastic and released when idle, with a minimum of zero giving a pay-for-usage profile, and reserved instance pools remove cold starts. Concurrency per instance is configurable from 1 to 200, and the platform is stateless, so per-connection affinity requires construction within the application.

**R3 — Immutable storage.** Object Storage Service provides bucket-level retention from 1 to 25,550 days, during which objects cannot be modified or deleted by any party including the owner. The policy locks after 24 hours and thereafter resists deletion or shortening. Versioning, prevent-overwrite rules and static website hosting are available.

**R4 — Authoritative DNS.** Global Traffic Manager provides proximity-based and latency-based access policies, health checks by ICMP, TCP, HTTP and HTTPS, failover, and two-level scheduling across address pools. Failover notifies the recursive gateway and the HTTP DNS service so that switching takes effect in seconds.

**R5 — Regional presence.** Europe: Frankfurt, London, Paris, Amsterdam. Southeast Asia: Singapore, Kuala Lumpur, Jakarta, Manila, Bangkok, Johor. North America: Virginia, Silicon Valley, Atlanta. No compute region is documented for Africa.

**R6 — Shared configuration store.** Edge KV is a globally distributed store with ten namespaces, one gigabyte per namespace, keys to 512 bytes and values to 1.8 MB, and an eventual consistency model that synchronises within seconds and guarantees within 300 seconds.

**R7 — Edge logic.** Functions and Pages provides a JavaScript runtime with a 120-second response time, a ten-second gateway wait, a four-megabyte package and four subrequests per execution. Edge Container provides serverless containers on points of presence with HTTP/3 and HTTP/2, TLS offload, canary releases and domain binding, on the enterprise plan.

**R8 — Operability.** Rules, load-balancer pools and edge containers are managed through a console and an API, with per-plan quotas on rule counts, origin pools and functions.

**Constraints.** The WebSocket action and the HTTP/2 and WebSocket exclusivity rule restrict a connection's lifetime at the edge. The documentation states that DNS rules are available on the premium and enterprise plans in one place and lists them as supported on all plans in another. Latency-based scheduling is confined to the highest edition and concentrates requests on a single address during switching.

**Assessment.** The most complete declarative rules coverage of the eight operators, and the only one of them to document origin server-name indication and DNS-record override in the same rules surface. Strong fit for the seed plane and the web tier. The absence of an African compute region prevents an operated lifeline in that region, and the edge WebSocket action is unsuited to a long-lived relay connection, which is addressed by addressing relays directly.

### Annex B — Tencent Cloud

**R1 — Edge delivery.** EdgeOne operates across more than 70 countries and regions with 3,200 acceleration nodes, 50 or more carrier access points and 400 Tbit/s of bandwidth, of which 2,300 nodes are in the Chinese mainland and 900 or more elsewhere. Traffic is directed by DNS and BGP hybrid resolution to unicast edge nodes. The rule engine provides conditional matching with nesting, where a lower rule supersedes an upper rule, and carries the node cache lifetime, browser cache lifetime, per-status-code cache lifetime, cache-key composition, host header rewrite, WebSocket duration, origin modification and redirection as actions. Rule quotas by plan are 20, 20, 50, 100 and 200, with nested conditional depth of 3, 3, 5, 10 and 20, and origin groups of 5, 20, 50, 100 and 100. Cache-key composition admits query string, cookie, request header and request protocol, the last three from the personal plan upward. Serving of expired content during an origin outage is available and enabled by default, and pre-warming is available as a console task. Origin selection by client geography and redirection by client geography are available as rule operations. Firewall policy covers custom rules, platform rate limiting, bot management and managed rule sets. Multi-origin failover is available as a rule operation covering connection failure, timeout, resolution failure and handshake failure, and as a load balancer with up to ten origin groups and health checks by ICMP, HTTP, TCP or UDP. WebSocket passthrough is available with a maximum connection duration of one to 300 seconds over HTTP/1.1, and a connection idle beyond the duration is closed.

**R2 — Regional compute.** Serverless Cloud Function provides a custom runtime for any language and container images, with a web-server image function listening on a fixed port. A native WebSocket protocol is available on an HTTP-triggered function, with an idle timeout from 10 to 7,200 seconds that the function's execution timeout must accommodate, a maximum packet of 256 KB and a per-connection rate limit of 128 KB/s. Elastic concurrency scales on arrival and provisioned concurrency pre-starts instances; a scale-to-zero statement is not documented.

**R3 — Immutable storage.** Cloud Object Storage provides object lock with a retention period from 1 to 36,500 days, extension only, and a compliance mode exposed through the API. The feature is available to allowlisted accounts and cannot be enabled for multi-availability-zone buckets or with versioning suspended.

**R4 — Authoritative DNS.** Cloud DNS Resolution operates 48 global cluster nodes and provides intelligent resolution with continent-level and per-country split zones, including several African countries, on the free plan and above. Weighted round-robin records and line type groups are available. Latency-based routing and health-check failover are not documented.

**R5 — Regional presence.** Europe: Frankfurt. North America: Virginia, Silicon Valley. Southeast Asia: Singapore, Jakarta, Bangkok. No region is documented for Africa.

**R6 — Shared configuration store.** EdgeOne KV Storage is available to allowlisted accounts, with an eventual consistency model, a default sixty-second read cache and ten namespaces per site.

**R7 — Edge logic.** Edge Functions provides a JavaScript runtime with 200 milliseconds of CPU time, 128 MB of memory, a five-megabyte package, 100 functions and 200 trigger rules per site. EdgeOne Makers provides static hosting with an edge function tier and a cloud function tier on Node.js, Python and Go.

**R8 — Operability.** Rules, origin groups, pre-warming, cache purging and function definitions are managed through a console and an API, with per-plan quotas. Traffic scheduling between EdgeOne and other providers by region is available on the enterprise plan.

**Constraints.** The 300-second WebSocket ceiling and the closure of idle connections make an edge-terminated relay connection unsuitable for long-lived mesh sessions. The edge key-value store is gated by allowlist. The object lock is allowlisted and compliance-only.

**Assessment.** Rule coverage matches the requirement set, with origin server-name indication and DNS-record override absent from the documented actions. The per-country DNS split zones reach African countries that the compute footprint does not. Fit for the seed plane and the web tier; the relay plane requires direct addressing, and an African operated lifeline requires a second operator.

### Annex C — Amazon Web Services

**R1 — Edge delivery.** CloudFront operates 750 or more points of presence in more than 100 cities across more than 50 countries, with 1,140 or more embedded points of presence and 15 regional edge caches, and serves anycast static addresses. Cache policies, origin request policies and response headers policies are declarative. Cache lifetime is expressed as minimum, default and maximum values, per-status-code error caching is available, cache-key composition admits headers, cookies and query strings, and the service honours the stale-while-revalidate and stale-if-error directives. Origin selection by client geography and redirection by client geography require a CloudFront Function. Origin groups fail over on configured status codes, timeouts and the 503 and 504 responses, and failover applies to the GET, HEAD and OPTIONS methods. Web Application Firewall provides 100 web ACLs per account per region, 5,000 web capacity units per ACL, 10 rate-based rules per ACL and 50 country codes per statement. Pre-warming and a DNS-record override are not documented.

**R2 — Regional compute.** Lambda Web Functions run an application listening on a port behind an HTTPS endpoint, scale to zero when idle, and can be deployed across regions behind one endpoint with latency-based routing. A request timeout of 30 seconds is the default, the function timeout reaches 900 seconds, and concurrent requests per environment range from 1 to 128. API Gateway provides bidirectional WebSocket APIs. Elastic Container Service with Fargate runs serverless containers with a minimum capacity of zero, and Application Load Balancers natively upgrade HTTP/1.1 connections into persistent WebSocket connections.

**R3 — Immutable storage.** Simple Storage Service provides object lock in a write-once read-many model with retention periods and legal holds, and provides versioning.

**R4 — Authoritative DNS.** Route 53 provides geolocation routing as a declarative routing policy.

**R5 — Regional presence.** Thirty-four regions. Europe: eight, including Frankfurt, Zurich, Ireland, London, Paris, Stockholm, Milan and Spain. Africa: Cape Town. North America: seven, including Virginia, Ohio, N. California, Oregon, Canada Central, Calgary and Mexico. Southeast Asia: Singapore, Jakarta, Malaysia and Thailand.

**R6 — Shared configuration store.** CloudFront KeyValueStore is a global store with 200 stores per account, a 512-byte key, a 1 KB value and 5 MB per store.

**R7 — Edge logic.** CloudFront Functions provide a JavaScript runtime for light edge work and Lambda@Edge provides a Node.js and Python runtime with a 30-second timeout, a 40 KB viewer-event response and 10,000 requests per second per region.

**R8 — Operability.** Quotas on cache policies, behaviours, origins, origin groups and functions are published, and 200 key-value stores are available per account.

**Constraints.** Geographic origin selection and geographic redirection require code, which places a JavaScript runtime in the path of a requirement the other operators meet with configuration. Pre-warming and the DNS-record override are absent from the documented feature set.

**Assessment.** The broadest regional compute footprint of the eight, and the only operator besides Oracle and Google with a compute presence in Africa as well as Europe, North America and Southeast Asia. A complete fit for the relay and lifeline planes. The edge plane carries a code requirement for geographic routing, and the global key-value store holds 1 KB values.

### Annex D — Google Cloud

**R1 — Edge delivery.** Cloud CDN caches load-balanced content at more than 100 locations, of which 23 are in North America, 23 in Europe, 16 in Asia-Pacific, 3 in Africa at Johannesburg, Lagos and Mombasa, 5 in South America, 4 in the Middle East and 2 in Oceania, and serves global anycast addresses over IPv4 and IPv6. Cache modes are declarative, with client, default and maximum lifetimes of 3,600, 3,600 and 86,400 seconds, a serve-while-stale period of 86,400 seconds, and negative caching with a separate lifetime per status code. Cache-key composition admits host, protocol, query string, headers and cookies. Origin selection by client geography is available through a Cloud DNS geolocation routing policy with health-check failover, and redirection by client geography through a firewall policy action. Origin host, port and server-name override and a DNS-record override are not documented, and pre-warming is not documented. Firewall policy permits 100 global security policies per project with 10 rate-based rules each. WebSocket passthrough requires no configuration on the HTTP and HTTPS load balancers.

**R2 — Regional compute.** Cloud Run runs containerised applications that scale to zero by default, with a configurable minimum instance count. WebSocket applications are supported without additional configuration, and the connection is bounded by the request timeout, which defaults to 300 seconds and extends to 3,600 seconds. A global external Application Load Balancer serves the service under a customer domain.

**R3 — Immutable storage.** Cloud Storage provides bucket lock, with a retention policy that can be permanently locked and never reduced or removed, and provides versioning.

**R4 — Authoritative DNS.** Cloud DNS provides geolocation routing policies with geofencing and multiple addresses per location, and failover routing policies with health checks.

**R5 — Regional presence.** Forty regions. Europe: thirteen, including Frankfurt, London, Berlin, Brussels, Amsterdam, Milan, Paris, Turin, Zurich, Stockholm, Helsinki, Madrid and Warsaw. Africa: Johannesburg. North America: twelve, including South Carolina, Ohio, Iowa, Dallas, N. California, Los Angeles, Salt Lake City, Las Vegas, Montréal, Toronto and Querétaro. Southeast Asia: Singapore, Jakarta and Bangkok.

**R6 — Shared configuration store.** Not verified as a global edge store. A regional cache service and a document database are available.

**R7 — Edge logic.** No programmable runtime is offered alongside the delivery products, which provide declarative routes only.

**R8 — Operability.** Global firewall policy and route counts are quota-governed; the firewall rule quota is published as a console reference.

**Constraints.** Geographic origin selection and redirection are met through two different services, one of them a firewall policy, which separates the delivery path from the routing path. A shared configuration store for the relay map is not available in a globally replicated form.

**Assessment.** Complete regional coverage of the four target regions, and the only operator among the three hyperscale providers to place delivery points of presence in three African cities. A fit for the relay and lifeline planes. The seed plane lacks a global configuration store and the delivery products carry no programmable runtime.

### Annex E — Microsoft Azure

**R1 — Edge delivery.** Front Door operates 192 edge locations across 109 metro cities, and the content delivery tier serves the same metro set. Traffic to the edge uses unicast for both DNS and HTTP, and the retired classic tier used anycast. A rule carries up to 10 match conditions and 5 actions, with conditions combined conjunctively and values within a condition combined disjunctively, and regular expressions supported. Per profile, rule sets number 100 or 200, rules per set 100, routes 100 or 200, endpoints 10 or 25, custom domains 100 or 500, origin groups 100 or 200, and origins 100 or 200, with 50 origins per group. Bandwidth is 75 Gbit/s and throughput 100,000 requests per second, with 3,000 concurrent WebSocket connections per profile. Cache lifetime is expressed as an edge lifetime of up to 366 days; the browser lifetime is set by the origin's cache-control header. Per-status-code cache lifetime, artefact pre-warming, a serve-expired toggle and a DNS-record override are absent from the documented actions. Cache-key composition is limited to query string. Origin selection by client geography and redirection by client geography are available through a geography match condition combined with an origin-group override and a redirect action. Origin host, port and protocol are configured on the origin, and the origin host name is used as the server name indication. Firewall policy provides 100 policies per subscription with 100 custom rules each, 10 match conditions per rule, 5 regular-expression rules, 600 address ranges and 100 exclusions. Multi-origin failover is available through origin groups with health probes. WebSocket passthrough is available on the standard and premium tiers.

**R2 — Regional compute.** Container Apps scales on KEDA with a minimum replica count of zero by default and a maximum configurable to 1,000, and supports HTTP and TCP scale rules, and its ingress supports WebSocket and gRPC. App Service enables WebSockets per application, with limits from 5 to 5 on shared tiers to approximately 50,000 per instance on the higher tiers, and supports up to 500 custom domains. Functions offers a custom handler for a long-running server, with a 230-second load balancer idle timeout on HTTP triggers, and scales to zero on the consumption plans. SignalR Service provides a WebSocket egress path.

**R3 — Immutable storage.** Blob Storage provides immutable storage in a write-once read-many state, with retention from 1 to 146,000 days, legal holds, and a policy that can only be extended. Static website hosting is available with restricted configuration, and header customisation requires the content delivery tier.

**R4 — Authoritative DNS.** Azure DNS public zones are served on a global anycast network, and Traffic Manager provides endpoint steering.

**R5 — Regional presence.** Europe: nineteen regions, including Ireland, Netherlands, France, Germany, Italy, Norway, Poland, Spain, Sweden, Switzerland and the United Kingdom. Africa: two, South Africa North and South Africa West. North America: fourteen, including Virginia, Ohio, Iowa, Texas, N. California, Washington, Montreal, Toronto and Mexico. Southeast Asia: nine, including Singapore, Hong Kong, Korea, Japan, Malaysia, Indonesia and Taiwan.

**R6 — Shared configuration store.** Not verified as a global store.

**R7 — Edge logic.** No first-party edge compute product. Edge Actions provide a constrained in-edge facility with 16 KB of code, 10 milliseconds of execution, three versions and 100 resources per subscription.

**R8 — Operability.** Rule, policy, origin and endpoint quotas are published, and 100 global firewall policies are available per subscription.

**Constraints.** Four elements of the requirement set are absent from the documented actions: per-status-code cache lifetime, artefact pre-warming, a serve-expired toggle, and a DNS-record override. Edge traffic is unicast. A shared configuration store and a programmable edge runtime are both unavailable.

**Assessment.** Complete regional coverage of the four target regions with two African regions, and a distinctive strength in a WebSocket egress service. The edge delivery rules engine covers the least of the eight, and the requirement for geographic origin selection and geographic redirection is met through the firewall service, with the delivery path carrying cache and transport duties.

### Annex F — Oracle Cloud Infrastructure

**R1 — Edge delivery.** The content delivery product is absent from the current services index, its documentation paths return errors, and the only retrievable official description is an accessibility report from January 2023 stating that the service delivers through a third party's global edge network. Oracle's own published article of January 2025 is titled as a migration guide directing content delivery clients to a third party. The operator's documented acceleration layer is Web Application Acceleration, a policy bound to a flexible load balancer applying caching and compression. The only declarative rule engine available is within the web application firewall, whose caching rules offer a cache or bypass action, a cache duration, a client cache duration and URL conditions. Traffic Management provides DNS steering policies with health-of-answer evaluation and geokeys. The footprint, addressing model, rule limits and rules expressiveness of the delivery product are not verifiable from current documentation.

**R2 — Regional compute.** Functions supports image-based functions from any container and code-only functions on Go, Java, Node.js and Python managed runtimes. A function is request-scoped, with a 6 MB request and response payload, memory to 3,072 MB, and a regional concurrent execution memory limit of 60 GB per availability domain. Execution environments are removed after a period of idleness. Container Instances run a container until stopped, with a lifecycle of create, start, stop and delete, and do not scale to zero automatically. Container Engine for Kubernetes is available. WebSocket upgrade pass-through is not verified for the load balancer, container instances or the managed Kubernetes service.

**R3 — Immutable storage.** Object Storage provides retention rules in a write-once read-many model, where locking a rule is irreversible and a 14-day delay precedes locking. Versioning is available. Native static website hosting is absent, and the documented path routes a bucket through API Gateway.

**R4 — Authoritative DNS.** Traffic Management provides geokeys and location-aware steering with health-of-answer failover.

**R5 — Regional presence.** More than 50 public cloud regions across 28 countries. Europe: eleven, including Frankfurt, Amsterdam, Paris, Marseille, Milan, Turin, Madrid, Stockholm and Zurich. Africa: two, Johannesburg and Casablanca. North America: six, including Ashburn, Phoenix, Toronto, Montreal and two in Mexico. Asia-Pacific: twelve, including Singapore, Tokyo, Osaka, Seoul, Mumbai, Hyderabad, Sydney and Melbourne.

**R6 — Shared configuration store.** Not verified as a global store. A managed memory cache with cross-region replication and a document database are available.

**R7 — Edge logic.** No edge compute product appears in the services index.

**R8 — Operability.** Not verified.

**Constraints.** The operator's edge delivery product has withdrawn from its own documentation, and the operator's published guidance directs delivery clients elsewhere. A programmable edge runtime is absent, and the declarative rule surface is confined to a firewall resource.

**Assessment.** The regional compute footprint is complete across the four target regions, with two African regions, and the regional service capability is strong. The edge plane cannot be assessed from current documentation and is treated as withdrawn. Fit for the relay and lifeline planes; the seed plane requires a different operator.

### Annex G — Cloudflare

**R1 — Edge delivery.** The network spans 348 cities, with 54 in North America, 64 in Latin America and the Caribbean, 57 in Europe, 19 in the Middle East, 71 in Asia, 36 in the Chinese mainland, 14 in Oceania and 33 in Africa, and traffic is directed by anycast. The rules surface comprises configuration rules, transform rules, origin rules, single and bulk redirects, and a cloud connector. Cache rules number 10, 25, 50 and 300 by plan. Edge and browser cache lifetimes are set independently, each with a respect-origin, override-origin or bypass mode. Per-status-code cache lifetime accepts a single code, a comparison or a range, and overrides the origin's cache-control. Cache-key composition admits headers, cookies, query strings of up to 100 parameters, host, and the device type, country and language of the request, with the header, cookie and custom-key options on the enterprise plan. Cache bypass, artefact pre-warming, and serving of expired content while revalidating are available. Origin selection by client geography steers by region, country or point of presence with a per-region failover order, and redirection by client geography is available as a bulk redirect rule matching the request's country. Origin rules provide host-header, server-name and DNS-record overrides, with the first three on the enterprise plan, and destination-port override on all plans. Firewall custom rules number 5, 20, 100 and 1,000. Multi-origin failover is available through load balancing with fallback pools. WebSocket passthrough to origin is available on all plans and inspects only the initial handshake.

**R2 — Regional compute.** Workers execute on the global network, and placement hints steer a worker toward the nearest data centre. Cloudflare Containers is the regional surface: any language or image, instances started on demand, an idle sleep that stops an instance, and an ephemeral container disk by default. Instance sizes run from 1/16 virtual CPU with 256 MB and a 2 GB disk to 4 virtual CPU with 12 GB and a 20 GB disk, with per-account limits of 1,500 concurrent virtual CPUs, 6 TB of memory, 30 TB of disk and 50 GB of image storage. A WebSocket upgrade passes through a container to a container port through a durable object, with a documented example. Durable objects provide strongly consistent per-object state on a SQLite backend with 10 GB of storage each, a 32 MB WebSocket message limit, a 30-second CPU limit that can be raised, and unlimited wall time while a client remains connected.

**R3 — Immutable storage.** R2 provides bucket locks with up to 1,000 per-prefix rules that block deletion and overwriting, and serves static assets with an entity tag derived from the asset's content hash, with immutable cache lifetimes configurable by path.

**R4 — Authoritative DNS.** Load Balancing provides health checks and steering, and the naming and resolution layer is integrated with the delivery service.

**R5 — Regional presence.** Delivery spans 33 African cities and 57 European cities, and container placement offers an Africa region alongside Europe, Asia-Pacific, the Americas, the Middle East and Oceania.

**R6 — Shared configuration store.** Workers KV provides a global store with a 512-byte key, a 25 MB value, one write per second per key, a minimum cache lifetime of 30 seconds, and 1,000 namespaces per account.

**R7 — Edge logic.** Workers accept JavaScript, TypeScript and WebAssembly, with a CPU limit rising from 10 milliseconds to 5 minutes, 128 MB of memory, a 64 MB worker size and up to 10,000 subrequests.

**R8 — Operability.** Quotas are published for rules, workers, namespaces, storage and container resources.

**Constraints.** WebSocket connections to origin are closed when idle, and the service may terminate connections when new code reaches the network, which requires client reconnection. Origin host, server-name and DNS-record overrides are confined to the enterprise plan, and a custom cache key by header or cookie is likewise enterprise-only.

**Assessment.** The strongest fit for the seed plane of the eight: complete declarative coverage including the four elements several operators lack, anycast distribution with the broadest city coverage, write-once storage with content-hash entity tags, a globally replicated configuration store, and strongly consistent per-object state with long-lived connections. The regional compute surface is a newer product with a smaller account-level quota than the hyperscale providers.

### Annex H — Tata Communications

**R1 — Edge delivery.** The content delivery network operates 28 super nodes on a tier-one internet service provider backbone, extended to 48 with a Latin American partnership, with global subsea cable routes as a fallback path. Stated performance is under 20 milliseconds for more than 80 percent of users in India, under 30 milliseconds in North America and Europe, and under 40 milliseconds across Asia. The addressing model is not verified. A declarative rules engine at the level of the individual capabilities is not verified; the documented control plane is an application programming interface for accounts, users, analytics, traffic reporting and caching, together with a customer portal, and no public rule reference is published. Per-plan rule limits are not verified. Edge compute provides runtime JavaScript engines at the content delivery edge. Firewall, denial-of-service protection and domain name service are integrated, with bot management.

**R2 — Regional compute.** Vayu Functions provides serverless execution on event and schedule triggers with per-function processor and memory control, web-hook and stream triggers, and scheduled invocations. Vayu Edge provides container orchestration on Kubernetes across more than 100 geographic locations with sandboxes in the United States, the United Kingdom and India, and managed database services. Vayu Cloud provides virtual machines, Kubernetes, processor acceleration and databases with automatic instance creation and deletion, and object storage with an S3-compatible protocol. WebSocket pass-through, scale-to-zero behaviour, minimum instance and concurrency constraints, and service-level region identifiers are not verified.

**R3 — Immutable storage.** Object storage with multiple tiers is available; write-once retention is not verified.

**R4 — Authoritative DNS.** Not verified.

**R5 — Regional presence.** Forty-four data centres worldwide are stated. The cloud and edge compute services are stated to be available in India, with the content delivery network global.

**R6 — Shared configuration store.** Not verified.

**R7 — Edge logic.** Runtime JavaScript engines at the content delivery edge, with execution limits not verified.

**R8 — Operability.** An application programming interface, a customer portal and self-service tools are available for accounts, users, analytics and caching. Per-plan quotas are not verified.

**Constraints.** The compute footprint is confined to India while the delivery network is global, which confines operated lifelines and relays to one region. The declarative rules surface, the addressing model, the immutable storage capability and the shared configuration store are all unverified, and the edge runtime's execution limits are unpublished.

**Assessment.** A substantial global delivery network with the strongest stated latency into India, and a cloud platform confined to India. The delivery plane alone is a candidate for the seed plane, subject to verification of the rules surface. The compute plane contributes a single region.

## 4. Recommended Pairing

Two operators are proposed for the operated planes, on the basis that the browser peer has exactly one transport path to the mesh, and that the mesh's own design assigns no single point of failure any authority over a node's access.

**Cloudflare and Amazon Web Services.**

Cloudflare carries the seed plane: anycast distribution across 348 cities including 33 in Africa, a declarative rules surface covering every element of R1, write-once bucket locks for digest-pinned artefact sets, a globally replicated key-value store for the relay map and the discovery list, and strongly consistent per-object state with a connection lifetime bounded only by the client's presence. Its content-hash entity tags and immutable cache lifetimes fit a digest-pinned artefact set exactly.

Amazon Web Services carries the regional durable plane: Elastic Container Service with Fargate at a minimum capacity of zero and native WebSocket upgrade in the Application Load Balancer, Lambda Web Functions scaling to zero behind a single endpoint, write-once object lock, declarative geolocation routing in Route 53, and thirty-four regions including Frankfurt, Cape Town, Singapore and seven in North America.

The synergy is that the delivery plane and the durable plane rest on separate operators. A failure, a policy action or a routing error at one leaves the other serving, and the client-side failover that already governs lifeline selection extends to the seed plane: a node unable to reach one edge reaches the other. The relay plane is reached directly at its regional addresses and rests on neither operator's edge, which keeps the single transport path of the browser peer independent of edge delivery behaviour, including the connection closures and idle timeouts that edge networks apply.

The cost is two control planes, two credential sets, split observability, and two configurations to keep aligned.

Two alternatives merit comparison. Google Cloud with Cloudflare reaches the same property with a stronger African compute presence and a media delivery product, at the cost of a shared configuration store that is not globally replicated. Alibaba Cloud with Tencent Cloud places two operators with dense Asian delivery presence, at the cost of neither holding an African compute region.

A single-operator deployment remains viable and is simpler. It is recommended for the first operated regions, where the requirement is presence and an operator's regional reach, and for the relay fleet, which begins on the project's own infrastructure at the primary domain and the Exocracy domain.

## 5. Evidence Appendix

The load-bearing claims behind §2, §3 and §4, each with its source and the page's last-updated date where shown.

### 5.1 Substrate

- iroh compiles to WebAssembly for browser targets as of the 0.33 release; browser peers reach other nodes through a relay, and relay traffic is end-to-end encrypted. — https://docs.iroh.computer/languages/wasm-browser
- iroh gossip supports browser builds from version 0.33. — https://docs.iroh.computer/languages/wasm-browser
- A node requires outbound TCP on port 443 to a relay, upgraded to a WebSocket; outbound UDP supports direct connections. — https://docs.iroh.computer/about/faq
- A relay requires a public IP address, a DNS name, and a certificate, with automatic TLS by ACME; the relay binary and image are published for self-hosting. — https://docs.iroh.computer/add-a-relay
- Relay traffic is end-to-end encrypted and the relay cannot read it. — https://docs.iroh.computer/concepts/relays
- Public relays operated by the upstream project are rate-limited, carry no availability guarantee, and are designated for development and hobby use. — https://docs.iroh.computer/iroh-services/relays/public

### 5.2 Alibaba Cloud

- 3,200 points of presence globally, more than 2,300 covering the 31 provincial regions of the Chinese mainland, 180 Tbit/s. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/product-overview/pop-distribution (Jun 03, 2026)
- Anycast directs traffic to the acceleration engine with global domain resolution under 30 ms. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/product-overview/what-is-esa (Aug 26, 2026)
- One rules engine covers cache, redirection, compression, origin fetch and firewall policy; rule quotas per plan and the rule expression syntax. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/user-guide/overview-of-rules (Sep 28, 2026)
- Origin rules expose custom origin host, protocol and port, origin server-name indication, and a DNS-record override. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/user-guide/back-to-source-rule-overview (Sep 14, 2026)
- Custom cache key admits query string, header, cookie, device type, and country or region. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/user-guide/custom-cachekey (Oct 09, 2026)
- Status-code cache lifetime ranges from one second to one year. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/user-guide/configure-the-status-code-cache-expiration-time (Sep 14, 2026)
- WebSocket acceleration on ports 80 and 443, enabled by default, unavailable on the entry tier, and exclusive of HTTP/2 on the same connection. — https://www.alibabacloud.com/help/en/edge-security-acceleration/esa/user-guide/network-optimization (Jul 16, 2026)
- A custom runtime is an HTTP server package, bound to the assigned port, ready within 120 seconds, with a request timeout of 15 minutes or longer. — https://www.alibabacloud.com/help/en/functioncompute/principles-1 (Apr 01, 2026)
- A custom container image must include an HTTP server, which the platform reaches on the assigned port. — https://www.alibabacloud.com/help/en/functioncompute/custom-container (Jun 02, 2026)
- Native WebSocket on an HTTP-triggered function; the platform is stateless, so per-connection affinity requires construction within the application. — https://www.alibabacloud.com/help/en/functioncompute/configure-an-http-trigger-for-a-function-that-is-triggered-by-websocket-requests-1 (Aug 24, 2026)
- Maximum function execution duration of 86,400 seconds, concurrency per instance from 1 to 200, and a maximum of 300 instances per region. — https://www.alibabacloud.com/help/en/functioncompute/limits-of-usage (Sep 07, 2026)
- Bucket-level retention from 1 to 25,550 days, locking after 24 hours, thereafter resistant to deletion or shortening. — https://www.alibabacloud.com/help/en/oss/user-guide/oss-retention-policies (Oct 08, 2026)
- Global Traffic Manager provides proximity-based and latency-based access policies, health checks and failover, and notifies the recursive gateway on switching. — https://www.alibabacloud.com/help/en/dns/gtm3-product-introduction (Sep 09, 2026)
- Region identifiers, and the absence of an African compute region across the compute, function, storage and data-transfer tables. — https://www.alibabacloud.com/help/en/cloud-migration-guide-for-beginners/latest/regions-and-zones (Oct 10, 2026) · https://www.alibabacloud.com/help/en/functioncompute/regions (Jun 24, 2026)

### 5.3 Tencent Cloud

- 3,200 acceleration nodes across more than 70 countries and regions, 400 Tbit/s, with DNS and BGP hybrid resolution to unicast edge nodes. — https://www.tencentcloud.com/document/product/1145/45962 (Apr 23, 2026)
- Rule engine quotas by plan, origin group counts, pre-warming quotas and the 300-second WebSocket ceiling. — https://www.tencentcloud.com/document/product/1145/55650 (Oct 13, 2026)
- WebSocket acceleration over HTTP/1.1 with a maximum connection duration of 1 to 300 seconds, disconnecting a connection idle beyond it. — https://www.tencentcloud.com/document/product/1145/46971 (Jul 29, 2025)
- Origin site failover covering connection failure, timeout, resolution failure and handshake failure. — https://www.tencentcloud.com/document/product/1145/81975 (Aug 03, 2026)
- Edge functions accept JavaScript only, with 200 ms of CPU time, 128 MB of memory and 100 functions per site. — https://www.tencentcloud.com/document/product/1145/53372 (May 06, 2026)
- A custom runtime supports any language; a web-server image function listens on a fixed port. — https://www.tencentcloud.com/document/product/583/38129 (Dec 02, 2024) · https://www.tencentcloud.com/document/product/583/69486 (Jun 17, 2025)
- Native WebSocket on an HTTP-triggered function, with an idle timeout from 10 to 7,200 seconds, a 256 KB packet limit and a 128 KB/s per-connection rate limit. — https://www.tencentcloud.com/document/product/583/44842 (Dec 02, 2024)
- Object lock with retention from 1 to 36,500 days, extension only, compliance mode through the interface, and allowlist availability. — https://www.tencentcloud.com/document/product/436/40136 (Jun 22, 2026)
- Cloud DNS Resolution provides continent-level and per-country split zones, including several African countries, on the free plan and above. — https://www.tencentcloud.com/document/product/1295/76951 (Mar 24, 2026)
- Region identifiers, and the absence of an African region. — https://www.tencentcloud.com/document/product/215/31786 (Oct 30, 2025)

### 5.4 Amazon Web Services

- 750 or more points of presence in more than 100 cities, 1,140 or more embedded points of presence, and 15 regional edge caches. — https://aws.amazon.com/cloudfront/features/
- Anycast static addresses, up to 21 per account. — https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/request-static-ips.html
- Geographic origin selection and geographic redirection require a CloudFront Function. — https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/example_cloudfront_functions_select_origin_based_on_country_section.html
- Origin group failover applies to the GET, HEAD and OPTIONS methods. — https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/high_availability_origin_failover.html
- Web application firewall limits of 100 web ACLs per account per region, 5,000 web capacity units per ACL and 10 rate-based rules per ACL. — https://docs.aws.amazon.com/waf/latest/developerguide/limits.html
- Distribution limits of 75 cache behaviours, 100 origins and 10 origin groups per distribution, and 200 key-value stores per account. — https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html
- Lambda Web Functions run an application listening on a port, scale to zero when idle, and deploy across regions behind one endpoint with latency-based routing. — https://docs.aws.amazon.com/lambda/latest/dg/web-functions.html
- Application Load Balancers natively upgrade HTTP/1.1 connections into persistent WebSocket connections. — https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html
- Task counts scale to zero when the minimum capacity is set to zero. — https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-auto-scaling.html
- Object lock uses a write-once read-many model. — https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html
- Route 53 geolocation routing. — https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy-geo.html
- Thirty-four regions, with identifiers for Europe, Africa, North America and Southeast Asia. — https://docs.aws.amazon.com/global-infrastructure/latest/regions/aws-regions.html

### 5.5 Google Cloud

- More than 100 delivery locations, with 3 in Africa at Johannesburg, Lagos and Mombasa, and global anycast over IPv4 and IPv6. — https://cloud.google.com/cdn/docs/locations (Oct 13, 2026) · https://cloud.google.com/cdn/docs/features (Oct 13, 2026)
- Cache lifetimes of 3,600, 3,600 and 86,400 seconds with a serve-while-stale period of 86,400 seconds, and negative caching per status code. — https://cloud.google.com/cdn/docs/caching (Oct 13, 2026) · https://cloud.google.com/cdn/docs/using-negative-caching (Oct 13, 2026)
- Geolocation routing policies with health-check failover at the domain name service layer. — https://cloud.google.com/dns/docs/configure-routing-policies (Oct 13, 2026)
- A redirection action available in firewall policy rules, with matches on the client's geography. — https://cloud.google.com/armor/docs/security-policy-overview (Oct 14, 2026)
- Cloud Run scales to zero by default, supports WebSocket without additional configuration, and bounds the connection by a request timeout defaulting to 300 seconds and extending to 3,600. — https://cloud.google.com/run/docs/about-instance-autoscaling (Oct 13, 2026) · https://cloud.google.com/run/docs/triggering/websockets (Oct 13, 2026)
- Bucket lock provides immutable storage, with a retention policy that can be permanently locked. — https://cloud.google.com/storage/docs/bucket-lock (Oct 13, 2026)
- Forty regions, with identifiers for Europe, Africa, North America and Southeast Asia. — https://cloud.google.com/compute/docs/regions-zones (Oct 15, 2026)

### 5.6 Microsoft Azure

- 192 edge locations across 109 metro cities, and unicast for both domain resolution and HTTP traffic. — https://learn.microsoft.com/en-us/azure/frontdoor/edge-locations-by-region (Sep 25, 2025) · https://learn.microsoft.com/en-us/azure/frontdoor/front-door-traffic-acceleration (Apr 21, 2026)
- Rule limits per profile, including 100 or 200 rule sets, 100 or 200 routes, 100 or 200 origin groups and 3,000 concurrent WebSocket connections. — https://learn.microsoft.com/en-us/azure/frontdoor/front-door-routing-limits
- The complete action list for the rules engine, from which the per-status-code lifetime, pre-warming, serve-expired and DNS-record override are absent. — https://learn.microsoft.com/en-us/azure/frontdoor/front-door-rules-engine-actions (Jun 04, 2025)
- Edge lifetime up to 366 days, with the browser lifetime governed by the origin's cache-control header. — https://learn.microsoft.com/en-us/azure/frontdoor/front-door-caching (Jul 15, 2026)
- Container Apps scales to zero with a minimum replica count of zero by default, and its ingress supports WebSocket and gRPC. — https://learn.microsoft.com/en-us/azure/container-apps/scale-app (May 20, 2026) · https://learn.microsoft.com/en-us/azure/container-apps/ingress-overview
- App Service enables WebSockets per application, with limits from 5 on shared tiers to approximately 50,000 per instance. — https://learn.microsoft.com/en-us/azure/app-service/configure-common
- Immutable storage in a write-once read-many state, with retention from 1 to 146,000 days. — https://learn.microsoft.com/en-us/azure/storage/blobs/immutable-storage-overview
- Public domain zones on a global anycast network. — https://learn.microsoft.com/en-us/azure/dns/dns-faq
- Region identifiers for Europe, Africa, North America and Southeast Asia. — https://learn.microsoft.com/en-us/azure/reliability/regions-list (Oct 13, 2026)

### 5.7 Oracle Cloud Infrastructure

- The delivery service is absent from the current services index, and its documentation paths return errors. — https://docs.oracle.com/en-us/iaas/Content/home.htm
- The January 2023 accessibility report describes the delivery service as operating through a third party's global edge network. — https://docs.oracle.com/en/corporate/accessibility/templates/t2-13054.html (30-JAN-23)
- The operator's published article of January 2025 is titled as a migration guide directing delivery clients to a third party. — https://blogs.oracle.com/cloud-infrastructure/why-cdn-clients-migrate-cloudflare-oci (Jan 06, 2025)
- Web Application Acceleration applies caching and compression as a load balancer policy. — https://docs.oracle.com/en-us/iaas/Content/web-app-acceleration/home.htm (Sep 19, 2025)
- Firewall caching rules offer a cache or bypass action with a cache duration and a client cache duration. — https://docs.oracle.com/en-us/iaas/Content/WAF/Tasks/cachingrules.htm
- Traffic Management steering accounts for the health of answers and the location from which the query originated. — https://docs.oracle.com/en-us/iaas/Content/TrafficManagement/Reference/trafficmanangementgeo.htm
- Functions execution environments are removed after a period of idleness. — https://docs.oracle.com/en-us/iaas/Content/Functions/Concepts/functionsconcepts.htm
- Container Instances run until stopped, on a lifecycle of create, start, stop and delete. — https://docs.oracle.com/en-us/iaas/Content/container-instances/home.htm
- Retention rules are irreversible once locked, with a 14-day delay before locking. — https://docs.oracle.com/en-us/iaas/Content/Object/Tasks/usingretentionrules.htm
- More than 50 public cloud regions across 28 countries, with identifiers for Europe, Africa, North America and Asia-Pacific. — https://docs.oracle.com/en-us/iaas/Content/General/Concepts/regions.htm

### 5.8 Cloudflare

- 348 cities, with 33 in Africa and 57 in Europe, and anycast addressing. — https://www.cloudflare.com/network/ · https://developers.cloudflare.com/fundamentals/concepts/how-cloudflare-works/ (Apr 20, 2026)
- Cache rules of 10, 25, 50 and 300 by plan. — https://developers.cloudflare.com/cache/how-to/cache-rules/ (Aug 14, 2026)
- Edge and browser cache lifetimes, per-status-code lifetime with comparisons and ranges, and cache-key composition including the device type, country and language of the request. — https://developers.cloudflare.com/cache/how-to/cache-rules/settings/ (Oct 15, 2026)
- Origin rules with host-header, server-name and DNS-record overrides, the first three confined to the enterprise plan. — https://developers.cloudflare.com/rules/origin-rules/ (Aug 14, 2026)
- Geographic steering with a per-region failover order, and bulk redirection matching the request's country. — https://developers.cloudflare.com/load-balancing/understand-basics/traffic-steering/steering-policies/geo-steering/ (Sep 10, 2026) · https://developers.cloudflare.com/rules/url-forwarding/examples/redirect-all-country/ (May 05, 2026)
- WebSocket to origin on all plans, inspecting only the initial handshake. — https://developers.cloudflare.com/network/websockets/ (Aug 14, 2026)
- Workers accept JavaScript, TypeScript and WebAssembly, with a CPU limit rising to 5 minutes and 128 MB of memory. — https://developers.cloudflare.com/workers/platform/limits/ (Oct 12, 2026)
- Durable objects provide strongly consistent per-object state, and the wall time is unbounded while a client remains connected. — https://developers.cloudflare.com/durable-objects/platform/limits/ (Jun 01, 2026)
- Containers start on demand, stop when idle, and expose a WebSocket upgrade through a durable object. — https://developers.cloudflare.com/containers/ (Aug 28, 2026) · https://developers.cloudflare.com/containers/examples/websocket/ (Oct 12, 2026)
- Bucket locks block deletion and overwriting, with up to 1,000 per-prefix rules. — https://developers.cloudflare.com/r2/buckets/bucket-locks/ (Apr 30, 2026)
- Static assets serve an entity tag derived from the content hash, with immutable cache lifetimes configurable by path. — https://developers.cloudflare.com/workers/static-assets/headers/ (Sep 22, 2026)
- Container placement regions include an Africa region. — https://developers.cloudflare.com/containers/concepts/placement/ (Oct 01, 2026)

### 5.9 Tata Communications

- 28 super nodes, extending to 48 with a Latin American partnership, with subsea cable routes as a fallback. — https://www.tatacommunications.com/hubfs/TaCO-2024/cdn/doc/cdn-datasheet-2024.pdf (Mar 14, 2024)
- Stated latency of under 20 milliseconds for more than 80 percent of users in India, under 30 milliseconds in North America and Europe, and under 40 milliseconds across Asia. — https://www.tatacommunications.com/hubfs/TaCO-2024/cdn/doc/cdn-datasheet-2024.pdf (Mar 14, 2024)
- Runtime JavaScript engines at the content delivery edge. — https://www.tatacommunications.com/hubfs/TaCO-2024/cdn/doc/cdn-datasheet-2024.pdf (Mar 14, 2024)
- The control plane exposes a basic API for accounts, users and analytics, a media API and a caching API. — https://developer.tatacommunications.com/apis
- Vayu Edge provides container orchestration across more than 100 geographic locations with sandboxes in the United States, the United Kingdom and India. — https://www.tatacommunications.com/hubfs/TaCO-2024/VAYU%20Edge/doc/tata-communications-vayu-edge-brochure.pdf (Mar 17, 2025)
- Vayu Functions provides serverless execution with event-based and scheduled triggers and per-function processor and memory control. — https://www.tatacommunications.com/cloud/enterprise-cloud/functions
- Vayu-branded services are available in India, while the content delivery network is global. — https://www.tatacommunications.com/press-release/tata-communications-vayu-to-revolutionise-cloud-for-intelligent-enterprise (Mar 20, 2025)

## 6. Open Items

- The set of artefacts constituting the seed plane, and the publication cadence of each.
- The signing and distribution mechanism for the relay map and the lifeline discovery list, and the client's verification rule for each.
- The relay fleet's rate limiting and abuse policy, and its authentication model.
- The durable state of an operated lifeline, and whether one database product or several serve it.
- The browser peer's local key storage and the recovery procedure for a lost browser-held identity.
- The operator holding the African operated lifeline under the pairing proposed in §4, neither operator in that pairing holding an African compute region.
- The content security policy for the seed plane and the web tier, and the treatment of the per-response nonce in the application.
- The point at which the relay fleet moves from the project's own infrastructure to operator-hosted regions.
