# Manage the domain's mailboxes from the Settings console

- Status: Proposed
- Deciders: project owner
- Date: 2026-10-22
- Technical Story: The Settings page is the site's authenticated console. The domain's mailboxes live on the instance beside the site, and the console gains the section that creates and withdraws them.

## Context and Problem Statement

The mail server on the instance holds a mailbox per address. A mailbox is a system account with a password, a mail store under the account's home directory, an alias line that routes the address to the account, and a Roundcube user that reads it. Adding one has been a manual operation on the instance, and the console had no surface for it.

The webmail's own interface cannot take the work on. Roundcube manages the users in its own database and reads the mail over IMAP, and it does not create accounts on the mail server, write the alias file or make a mail store. The mailbox list belongs with the console's other mail surfaces, which the email record names as the suppression list, Brevo's blocked contacts, the opt-out history and the DMARC report view.

## Decision

The Settings page at `/settings` carries a Mailboxes section. It shows each mailbox with its address and the name that appears on the mail it sends, and it offers the creation of a mailbox and the removal of one.

**Authority.** The section is reachable by the identities holding the Admin delegation, and by the owner named in `OWNER_DID` while the delegation store is empty, which is the same rule the owner tools and the grant function follow.

**Creation.** Creating a mailbox performs these steps on the instance, in this order: the account, with a home directory under `/home` and no login shell; its password; its mail store, with the three folders a mail store holds; the alias line that routes the address to the account, with the alias database rebuilt; and the Roundcube user and the identity that carries the display name. The password is drawn by the site from letters and digits, twenty-four characters long.

**The password.** The password is shown once, in the section that created the mailbox, and is held in the instance's password file at `/root/.mailbox-passwords` with owner-only permissions, one line per mailbox. That file is the instance's record of the mailbox passwords, and the console keeps it current.

**Removal.** Removing a mailbox withdraws the Roundcube user and its identity, drops the alias line, and withdraws the account. The mail store stays on the instance as the archive of what the address received.

**The address.** An address is lowercase letters and digits with dots, underscores and hyphens, up to sixty-four characters. It becomes an account name and a directory name, and it is held to that shape.

**Failure.** A step that fails undoes the steps before it, so a half-made mailbox leaves no account behind. The section reports the failure in the language of the step that failed.

## Consequences

A mailbox is created and withdrawn from one place, by the person who administers the site, without a session on the instance. The mail server's own tooling remains the source of the mailboxes, so the Roundcube interface and the console agree on what exists.

The console reaches the mail server's accounts directly, because the site and the mail server run on the same instance and the site's service runs as root. The section therefore carries the authority of the Admin delegation into the mail server's account tools, which the owner policy record governs.

The passwords of the mailboxes live in one file on the instance, readable by root, which is the same posture the instance's other credentials hold. The section shows a password once, at creation; the file is where a password is found afterwards.

Removing a mailbox leaves its mail on the instance without an account to read it. The archive is there for the record, and reading it again means restoring the account.
