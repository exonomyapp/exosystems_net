# Discover joined nodes after sign-in

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: the tools learn which conscia nodes serve the signed-in identity and work through them.

## Context and Problem Statement

The signed-in store holds the list of conscia nodes the identity has joined, and each joined node holds and forwards the identity's content. Tools that never read that list work in isolation: shares miss their audience, publications miss their mirrors, and background updates arriving through the nodes go unnoticed.

## Decision

After sign-in, the tools read the joined-node list from the local store. Shares and publications address the joined set. Background updates arriving through those nodes are accepted into the local store without user action. The tools present the joined set for inspection so the operator sees exactly which nodes participate.

## Consequences

Collaboration updates flow at the speed of the synchronized local store, with the conscia nodes contributing silently in the background. Joining a new node extends the tools' reach as soon as the store synchronises the new relationship.
