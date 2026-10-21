# Grant document access per identity from the docs view

## Status

Proposed

## Context

Publishing means choosing who may read and who may write, the way
sharing works on mainstream docs: the author names people and grants
whatever powers Meadowcap supports (read, write, admin over
namespaces per Spec 11 — the record names no fixed labels of its
own). Whether strangers see a document listed is the author's
per-document publish setting (Spec 41 publish toggles), never a
default imposed by the software. The three SR1 accounts exist for
exactly this shape — author, enabled reader, stranger. No record
defines how a docs author grants those powers per document, or where
the grant lands.

## Decision

Every publishable document carries a sharing control beside it: the
author picks an identity (`did:peer`), grants whichever Meadowcap
powers the document needs, and sets the document's listing
visibility; the grant is issued as a Meadowcap delegation scoped to
that document's namespace, recorded in the node's identity store
(SR2). Removing a name revokes the delegation; sync stops carrying
that namespace to the removed identity. The control lives in the
docs view; the enforcement lives in the node, never in page
markup.

## Consequences

Sharing works like the apps people already know, backed by
capability math instead of a server list. The docs view gains one
control per document; the node gains per-namespace delegations.
Grant, revoke, and stranger-refused are part of the SR1 pass.
Per-document admin (handing the sharing control itself to another
identity) is out of scope until multi-author docs are exercised.
