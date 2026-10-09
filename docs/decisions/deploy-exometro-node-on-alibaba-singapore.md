# Deploy the ExoMetro node on the Alibaba Singapore instance

## Status

Proposed

## Context

Both Exosystems.net instances, the Tencent Lighthouse box and the Alibaba Singapore box, run the ExoMetro flavor of the Conscia node line. ExoMetro carries node management as its first priority, per the three-flavors record. The full-node record defines the feature set: relay, indexing, discovery, persistence, authentication, observability, and federation over Willow namespaces, SQLite state, and Iroh peer connections, with the Spec 67 web applications for dashboard, chat, and administration. The Singapore box is surveyed, reachable, and freshly hardened at the network layer; its CentOS 7 system is past vendor end of life with a migration tracked in the hardening guide.

## Decision

The Alibaba Singapore instance hosts an ExoMetro-flavor Conscia node as the second Exosystems.net peer next to the Tencent instance. The node carries the full Conscia feature set under the owner's operator policies for conscire approvals and settings. Hardening items for SSH identity, throttling, and the local firewall land first, and the operating system migration completes before the node serves production traffic.

## Consequences

Exosystems.net gains a Singapore peer with relay, discovery, persistence, and federation presence in the region. Operator load grows: conscire requests, capability records, and settings need management on two nodes. Node listeners arrive with the deployment and re-open the listener survey in the hardening guide. The EOL operating system blocks production cutover until migration finishes.
