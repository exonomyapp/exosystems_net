# Implement hardening on the Tencent Lighthouse instance

## Status

Accepted

## Context

The Tencent Lighthouse instance at 43.157.89.31 (OpenCloudOS 9.6) hosts Exosystems.net with public web on ports 80/443 and public SSH on port 22, plus loopback-only dashboard and API services. The hardening guide at `docs/harden.md` defines the standing expectations for every instance: key-only SSH under a named admin, custom least-privilege firewall groups, a local firewall, brute-force throttling, automatic security updates, justified listeners, current TLS, and reversible changes. Web service hardening (nginx enabled plus the certbot renewal timer) is done; the remaining items are a program of work in the documented order.

## Decision

The hardening guide program applies to the Tencent Lighthouse instance item by item with owner approval at each step, verified by the read-only commands in the guide and rolled back through snapshots plus dated file backups.

## Consequences

Public web keeps serving while each layer lands. SSH moves to named-user key-only logins, which removes password guessing as an attack path and requires the owner to hold a working key before root login closes. Local firewall and update automation add ongoing maintenance surface in exchange for a smaller attack window. The dashboard and API stay loopback-only behind the existing proxy, so their exposure does not change.
