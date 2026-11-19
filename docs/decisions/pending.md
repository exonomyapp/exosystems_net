# Pending decisions

Decisions still open, what unblocks each, and where each lands.
Reviewed whenever the work-report's next steps change.

The register of open decisions is this file. The session transfer report at `docs/reports/session_transfer_report.md` carries the work in flight: its "What's Next" section holds the rows being taken up now, and its "Pending" section holds the rows waiting behind them. A row leaves this register only with a dated addendum or a superseding record, so the report's copy is a view of this file and this file remains the authority on what is open and what unblocks each item.

| # | Decision | Blocked on | Lands in |
|---|----------|------------|----------|
|| 1 | Provision the plain node on the instance by hand | Instance access window; conscia identity generation | `define-plain-node-scope-with-discovery.md` addendum with live evidence |
| 2 | Create the GitHub repo and turn on the pipeline — closed 2026-10-22 | The repository and the pipeline are live; the addendum is in `set-up-github-repo-and-ci-pipeline.md` | `set-up-github-repo-and-ci-pipeline.md` addendum on first green deploy |
| 3 | Build challenge-response secret ownership | Sign-in flow settled (done); crypto review of nonce store | `prove-secret-ownership-at-sign-in.md` Accepted |
| 4 | Build Profile Bundle export/import | Secret ownership (3); second-device test | `recover-and-move-exotalk-profile-across-devices.md` Accepted |
| 5 | Sync cadence beyond sign-in and reconnect | Node running (1); observed usage | Addendum to `keep-local-synced-willow-store.md` |
| 6 | Decide which change wins when two devices edit the same content | Profile Bundle working (4), then a real two-device sync exercise | New record replacing the store record's "conflict merges out of scope" note |
| 7 | Rewire boot and sign-in to the SQLite identity store and delete `allowed_dids.json` | Node store schema live (1); SR1 accounts re-conscired through the node's conscire endpoint | Addendum to `exosystems-exoauth.md` — closed 2026-10-23, code shipped in commit `a02000d`; `identity_store.rs::open()` creates `identity.db` if missing, fixing the boot panic that left `exosystems-net.service` in a restart loop |
| 8 | Retry the `sqlx` step to 0.9.0 | Locked `iroh` line dropping the `sha2` pre-release pin | Addendum to `choose-node-database-for-identity-and-willow-state.md` |
| 9 | Place the Brevo API key in the instance's credential file beside the SMTP pair — closed 2026-10-22 | The key is in `/opt/exosystems_net/env`; the addendum is in `provision-the-authenticated-settings-profile.md` | `provision-the-authenticated-settings-profile.md` addendum |
| 10 | Build the console's mail operations section: the domain's mailbox list with its create and remove actions — closed 2026-10-22 | The section is built; the record is `manage-mailboxes-from-the-settings-console.md` | `manage-mailboxes-from-the-settings-console.md` |
| 11 | Petition flow for unknown identities that pass the challenge: site prompts to petition the node, owner approves/refuses, identity enters the store | Sign-in flow on SQLite store (7); petition email templates record | `prove-secret-ownership-at-sign-in.md` addendum — code shipped (petition modal + approve/refuse in `auth.rs`/`app.rs`), browser verification pending |
| 12 | Save-secret option on sign-in: device-local storage of the secret for returning users (shipped 2026-10-23 — `store_saved_secret` plus per-card persisted checkbox state via `store_flag`/`read_flag`), plus Conscia-node encrypted backup and Profile Bundle export | Sign-in flow on SQLite store (7) | `prove-secret-ownership-at-sign-in.md` addendum — code shipped (save-secret checkbox + store_saved_secret + per-card persisted state in `auth.rs`/`app.rs`), browser verification pending |
Rule: nothing leaves this list silently — each row leaves with a
dated addendum or a superseding record, never a quiet edit.
