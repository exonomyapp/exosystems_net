# Install the Conscierge plugin and become node-aware

## Status

Proposed.

## Context

The Conscia installation ships a component that a web application plugs in to become node-aware: an iroh listener that joins the mesh, gathers the announcements that running Conscia nodes broadcast, and reports the node list to the application that embedded it. A node announces its display name, its ip, and its flavor. The application shows each detected node as the display name followed by the ip and the flavor in parentheses. This display is independent of signing in: every visitor sees the detected nodes.

A signed-in user sees more. Beside a node's name sits the conscire button, or the deconscire button when the signed-in identity is already conscired with that node and wants to detach. Consciring and signing in are separate acts (`conscire-local-exotalk-with-instance-node.md`): signing in establishes the user context and unlocks the local Willow store; consciring is the handshake with a node.

When the signed-in identity is conscired with a detected node, the assistant surface becomes that node's Conscierge app. The Conscierge app knows which did:peer is using it, so it loads the authenticated features and content for that identity, in the shape authored for it in ExoSoul (Spec 51 and Spec 54, exotalk repository).

## Decision

1. The Conscierge plugin ships with the Conscia installation as a component the host application embeds. This record proposes where the listener runs; the owner picks the scenario.

2. **Scenario A — the listener runs in the server, at the Leptos level.** The plugin runs a native iroh endpoint inside the Leptos server process. It subscribes to the Conscia gossip topics and keeps the node list. The application's server functions carry the list to the front end. Every visitor's page shows the nodes, and the visitor's browser joins no mesh.

3. **Scenario B — the listener runs in the browser, at the Leptos level, as WebAssembly.** The plugin compiles to wasm32 and runs inside the front-end bundle. Each browser joins the mesh itself and hears node announcements directly: the visitor's own connection carries the discovery. A browser peer cannot open a raw socket, so every browser connection to the mesh goes through a relay. This scenario depends on the self-hosted relay (`self-host-iroh-relay.md`) and adds the plugin's WebAssembly weight and a mesh connection to every page load. iroh 1.4.1 — the line this repository locks — provides the wasm browser target (verified against docs.rs, 2026-10-26).

4. **Scenario C — nginx mounts the Conscia daemon's own web apps.** nginx routes the applications the Conscia daemon serves — the Conscierge app at `/conscierge/`, and the node's public dashboard at its path — under the exosystems.net domain, following the same routing pattern the site already uses for `/api/` and `/store/` (`ship-the-nginx-configuration-with-the-deployment.md`). Scenario C answers where the Conscierge app is served from. Scenarios A and B answer how the application hears the nodes. C pairs with A or with B.

5. The plugin's iroh line follows its host. This repository locks iroh 1.4.1. The Conscia daemon builds on the exotalk workspace line, which is iroh 0.98.2 today with the migration record open. The version register (`docs/reports/version_status.md`) tracks the split until pending row 8 resolves it.

## Consequences

- The application shows the detected nodes to everyone, and the signed-in identity gets the conscire and deconscire buttons beside each node's name.
- The scenario choice fixes where the mesh connection lives: on the server for A, in every browser for B. C is available either way for serving the Conscierge app.
- The Conscierge app in authenticated mode loads the per-identity features and content authored in ExoSoul, and the Willow store's sync carries the identity's content to the page (`adapt-the-home-page-to-the-signed-in-identity.md`).

## Links

- `conscire-local-exotalk-with-instance-node.md` — the conscire handshake and its separation from signing in.
- `self-host-iroh-relay.md` — the relay Scenario B depends on.
- `ship-the-nginx-configuration-with-the-deployment.md` — the routing pattern Scenario C follows.
- `present-the-exoworker-chat-to-visitors.md`, `adapt-the-home-page-to-the-signed-in-identity.md`
- Spec 67, Spec 51, Spec 54 (exotalk repository) — the Conscierge app and the ExoSoul authoring contracts.
- `docs/reports/version_status.md` — the iroh line split.
