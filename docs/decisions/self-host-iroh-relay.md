# Self-host exosystems.net iroh relay for network isolation

- Status: proposed
- Deciders: project owner
- Date: 2026-10-16

## Context and Problem Statement

ExoTalk and Conscia activity on exosystems.net currently relies on iroh's public relays when direct peer-to-peer connections cannot be established. Public relays are shared with every other iroh developer worldwide, rate-limited, and operated by n0 computer. This means ExoTalk sync traffic and Conscia node traffic could traverse infrastructure outside of exosystems' control, and the network is not isolated from other iroh users.

The goal is to run a self-hosted iroh relay dedicated to exosystems.net so that all ExoTalk and Conscia P2P traffic stays within infrastructure operated exclusively for this project, with node-level invite-based choice governing which peers can connect.

## Decision Drivers

- All ExoTalk and Conscia traffic must remain between ExoTalk and Conscia nodes only
- No traffic should route through public iroh relays shared with unrelated users
- The relay must be invite-based: only nodes that have been explicitly invited can use it
- The network should be invisible to other iroh networks and users
- Relay must work with iroh 1.x (current release series)

## Considered Options

1. **Use iroh's public relays** — Zero infrastructure cost, but traffic shares relay capacity with all other iroh users, is rate-limited, and is not isolated from other iroh networks.
2. **Self-host a dedicated iroh relay** — Run iroh-relay on exosystems.net infrastructure, configure ExoTalk and Conscia endpoints to use only this relay, disable public relay fallback. Full control over access, version, and isolation.
3. **Use Iroh Services dedicated relay** — Managed relay provisioned for the project, but still operated by n0 computer and not fully isolated from their infrastructure.

## Decision Outcome

Option 2: Self-host a dedicated iroh relay on exosystems.net. Configure all ExoTalk and Conscia endpoints to use RelayMode::Custom with only the self-hosted relay URL. Disable public relay fallback so traffic never touches n0's public relay infrastructure. Configure the relay with authentication so only invited node identities can connect.

### Positive Consequences

- All ExoTalk and Conscia sync traffic stays on project-operated infrastructure
- No exposure to public iroh relay rate limits or unrelated users
- Version locking: the relay and endpoints can be kept on a matching iroh version
- Full control over access control: invite-based node participation
- Network is invisible to other iroh users because endpoints never advertise on public relays

### Negative Consequences

- Additional infrastructure to operate and maintain (relay binary, TLS certificates, monitoring)
- Single point of failure for relayed connections unless multiple relays are deployed
- Direct peer-to-peer connections still work when NAT traversal succeeds; the relay is only the fallback
- Must keep relay version compatible with endpoint versions

## Links

- iroh relay documentation: https://docs.iroh.computer/iroh-services/relays/self-hosted
- iroh relay source: https://github.com/n0-computer/iroh/tree/main/iroh-relay
- iroh release v1.2.0: https://github.com/n0-computer/iroh/releases/tag/v1.2.0
- iroh release v1.3.0: https://www.iroh.computer/blog/iroh-1-3-0
- ExoTalk spec 03 (peer-to-peer networking): /home/exocrat/code/exotalk/docs/spec/03_peer_to_peer_networking.md
- Exosystems.net deploy: /home/exocrat/code/exosystems_net/.github/workflows/deploy.yml

## Implementation checklist

1. Deploy iroh-relay binary on exosystems.net infrastructure with a public DNS name and TLS certificate
2. Configure relay with access control so only invited node identities can connect
3. Update ExoTalk endpoints to use RelayMode::Custom with the self-hosted relay URL
4. Update Conscia node endpoints to use RelayMode::Custom with the self-hosted relay URL
5. Disable public relay fallback on all endpoints
6. Test that ExoTalk-to-Conscia sync works through the self-hosted relay
7. Verify that endpoints do not advertise on or connect to public iroh relays
