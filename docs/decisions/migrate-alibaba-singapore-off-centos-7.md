# Migrate the Alibaba Singapore instance off CentOS 7

## Status

Proposed

## Context

The Alibaba Cloud ECS instance `i-t4n5ee5yucj5f84co4s4` in Singapore (47.84.81.181) runs CentOS 7.9, past vendor end of life since 2024-06-30 with public security patches stopped, as Alibaba documents. The instance is surveyed and hardened at the network layer, and the ExoMetro-flavor Conscia node is planned on it as the second Exosystems.net peer next to the Tencent instance. Both the hardening record and the ExoMetro deployment record hold production traffic until the migration completes.

## Decision

The Singapore instance moves to a supported operating system image before it serves production traffic. The migration preserves the hardened network posture (custom `custom-main` group with SSH only) and the key-only SSH access, and the ExoMetro deployment follows the migration. The target image and the migration window stay open until the owner names them.

## Consequences

The box sheds exposure to unpatched CentOS 7 flaws and becomes eligible for the production peer role. The migration carries downtime on the Singapore box and re-verification of the hardening items after the move, since a fresh image resets the in-box state the survey recorded.
