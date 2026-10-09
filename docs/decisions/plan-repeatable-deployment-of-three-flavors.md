# Plan repeatable deployment of the three Conscia flavors

- Status: Proposed
- Deciders: project owner
- Date: 2026-09-29
- Technical Story: the hand-provisioned instance serves development verification under the temporary-instance record; repeatability is planned for the flavored product lines.

## Context and Problem Statement

Three flavored node lines carry the product future, while the current instance exists so the local application has a live counterpart during development. Automation targets the flavor lines from the start, because those lines carry the shipped product.

## Decision

Repeatable deployment is defined per flavor line, each as a versioned image plus its configuration surface: the plain core helper, the ExoSocial flavor, and the ExoMetro flavor with node management as its first priority. Each flavor build runs the proof suite for its scope before release. The temporary hand-provisioned instance is excluded from this scheme and is retired once the plain flavor image lands.

## Consequences

Development verification continues against the temporary instance while deployment engineering proceeds on the flavor lines. No procedure in this record applies to the temporary instance, and no temporary step is included in flavor deployment.
