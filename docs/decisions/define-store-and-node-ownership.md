# Define store and node ownership, and the meaning of OWNER_DID

## Status

Proposed

## Context

Ownership in the exosystem belongs to Willow stores and to Conscia nodes. A user identity owns its own Willow store. A Conscia node owns its own store, created by its creator identity. The two are separate: an identity's store and a node's store are different things, and owning one does not imply owning the other.

Signing in to the application verifies a `did:peer` and opens that identity's Willow store. This requires only the iroh mesh and the Willow store; ExoTalk and Conscia are applications that build on top of the verified identity and are not prerequisites for sign-in. The exosystem supports two identity kinds today: `exotalk` for users and `conscia` for non-user nodes. Applications (ExoTalk, ExoSoul, Conscia) and exoapps (Exonomy, Exocracy, RepubLet, and others) consume the verified identity and store; they are not part of sign-in.

The instance configuration carries `OWNER_DID`, the identity of the person who deployed the Conscia node on that instance. On an empty store, that identity gains operator surfaces (settings, petitions, approvals) so the instance owner can manage the node before any joins are approved. This is a property of that specific identity on that specific instance.

When ExoTalk is built, it will carry tools to define, provision, deploy, and install new Conscia nodes on instance targets. Each such node will have its own store, owned by its creator identity. The creator's own store and the node's store remain distinct.

## Decision

1. An identity owns its Willow store. Ownership is inherent to the identity and needs no separate record; the store is the identity's.
2. A Conscia node is a `conscia`-kind identity that owns its own store, and it has its own `did:peer` distinct from its creator's. The node's store is created and owned by its creator identity, which may be the same identity as the instance owner or a different one. The node operates on its creator's authority: it acts per its owner's dictates, as if it were the owner of its own store, through Meadowcap capability delegations from the creator identity to the node identity. The node's own identity is separate and does not merge with the creator's identity.

3. `OWNER_DID` names the identity of the instance owner. On an empty store, that identity gains operator surfaces. This is the instance owner's own store receiving operator access, not a user becoming an owner.
4. Signing in to the application does not make the signing-in identity an owner of anything beyond its own store. It never grants operator surfaces on a node or on another identity's store.
5. Consciring is a separate action from signing in. The application performs it on the user's behalf only after the user's Willow store context exists, presenting the user's `did:peer` to the node's conscire endpoint for the node owner's approval under the node's own policy.

The data model needs no new ownership column. Ownership is expressed by the identity kind (`exotalk` or `conscia`) and by which identity signs in matching `OWNER_DID` on an empty store. A node's store is a separate artifact owned by its creator identity, recorded when the node is provisioned.

## Consequences

The concept of "owner" is unambiguous: it refers to the identity named in `OWNER_DID` on a given instance, acting on its own store. No user becomes an owner by signing in. The bootstrap that grants operator surfaces on an empty store applies only to that identity on that instance.

The distinction between an identity's store and a node's store stays explicit in code and documentation, so provisioning tools (such as those ExoTalk will carry) can create node stores owned by their creator identity without conflating them with the creator's personal store.

Records that touch ownership (`exosystems-exoauth.md`, `build-fresh-boot-and-sign-in-on-node-store.md`, `provision-the-authenticated-settings-profile.md`) reference this record for the definition.
