# Adapt the home page to the signed-in identity

## Status

Proposed.

## Context

The home page today is one surface for everyone. The site now distinguishes three states: the visitor, the signed-in identity, and the signed-in identity that is conscired with a Conscia node.

The visitor surface stays as it is. Visitors are visitors: the site knows them through their browser alone (cookies and their kin), and the site treats them as unauthenticated.

Signing in to exosystems.net and consciring with a Conscia node are separate acts. Signing in establishes the did:peer user context and unlocks the local Willow store. The store syncs the account's content from every location that syncs with that identity, so the page can reflect the identity's content as soon as the store carries it. The application is node-aware through the Conscierge plugin (`install-the-conscierge-plugin-and-become-node-aware.md`).

## Decision

1. The unauthenticated home page keeps the visitor surface, including the ExoWorker chat (`present-the-exoworker-chat-to-visitors.md`).

2. Signing in transforms the home page into the identity's landing page. The landing page assembles the identity's world: the content of the unlocked Willow store, the ExoApps the identity subscribes to (beginning with ExoTalk), and the assistant surfaces.

3. The landing page presents the two assistants. ExoWorker is always present, powered by the site's own oc-worker. The Conscierge app of a node the identity is conscired with appears in its authenticated mode: it knows the did:peer using it, and it loads the features and content authored for that identity in ExoSoul.

4. The page adapts as the store syncs. Content arriving from any Conscia within iroh reach of the did:peer lands through the store and shows on the page. Consciring with a node adds that node's services to the surface; deconsciring removes them.

## Consequences

- One route carries three states, and each state's surface is complete for that state.
- The Willow store's sync state drives the landing page's content, so the page reflects what has actually synced.
- The settings console remains the control surface (`shape-settings-page-as-identity-and-mesh-console.md`); the landing page is the home surface.

## Links

- `present-the-exoworker-chat-to-visitors.md`, `install-the-conscierge-plugin-and-become-node-aware.md`
- `keep-local-synced-willow-store.md`, `discover-joined-nodes-after-sign-in.md`, `conscire-local-exotalk-with-instance-node.md`, `shape-settings-page-as-identity-and-mesh-console.md`
- `run-oc-worker-on-the-operational-instance.md` (oc-worker repository)
