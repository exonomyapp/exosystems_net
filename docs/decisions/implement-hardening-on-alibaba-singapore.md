# Implement hardening on the Alibaba Singapore instance

## Status

Accepted

## Context

The Alibaba Cloud ECS instance `i-t4n5ee5yucj5f84co4s4` in Singapore (47.84.81.181, CentOS 7.9) belongs to Exosystems.net. Alibaba's security evaluation flagged the account because the instance ran in an auto-created default security group open to the whole internet. The instance now runs in the custom group `custom-main` (`sg-t4n3boqblzd4mif70x1x`) carrying only SSH port 22, with the default group detached. The in-box survey of 2026-10-09 found a fresh box: root as the only shell user, key-only SSH already active, no local firewall, no update automation, rpcbind and atd disabled, loopback-only mail delivery. The owner roams between addresses and keeps SSH world-open by explicit decision, with key-only auth plus throttling as compensation. The box runs CentOS 7, past vendor end of life since 2024-06-30 as Alibaba documents, so public security patches have stopped.

## Decision

The hardening guide program applies to the Alibaba Singapore instance with two recorded variances: SSH port 22 stays world-open under the owner's roaming decision, and the operating system moves to a supported image under a separate migration before the box serves production traffic. Remaining guide items (named admin user with root login closed, brute-force throttle, local firewall, update automation) land in the documented order with owner approval at each step.

## Consequences

The evaluation finding clears through the custom group. World-open SSH keeps access flexible from any address and keeps login-attempt noise, which the throttle and key-only auth absorb. CentOS 7 stays exposed to unpatched flaws until the migration completes, so the box carries no production role before then. Admin access runs through the registered Alibaba keypair, whose private half lives at `~/.ssh/alibaba_sg_ecs` with mode 600.
