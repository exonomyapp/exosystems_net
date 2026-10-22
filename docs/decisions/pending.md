# Pending decisions

Decisions still open, what unblocks each, and where each lands.
Reviewed whenever the work-report's next steps change.

The register of open decisions is this file. The session transfer report at `docs/reports/session_transfer_report.md` carries the work in flight: its "What's Next" section holds the rows being taken up now, and its "Pending" section holds the rows waiting behind them. A row leaves this register only with a dated addendum or a superseding record, so the report's copy is a view of this file and this file remains the authority on what is open and what unblocks each item.

| # | Decision | Blocked on | Lands in |
|---|----------|------------|----------|
| 1 | Provision the plain node on the instance by hand | Instance access window; helper identity generation | `run-full-conscia-node-on-exosystems-net.md` addendum with live evidence |
| 2 | Create the GitHub repo and turn on the pipeline — closed 2026-10-22 | The repository and the pipeline are live; the addendum is in `set-up-github-repo-and-ci-pipeline.md` | `set-up-github-repo-and-ci-pipeline.md` addendum on first green deploy |
| 3 | Build challenge-response secret ownership | Sign-in flow settled (done); crypto review of nonce store | `prove-secret-ownership-at-sign-in.md` Accepted |
| 4 | Build Profile Bundle export/import | Secret ownership (3); second-device test | `recover-and-move-exotalk-profile-across-devices.md` Accepted |
| 5 | Sync cadence beyond sign-in and reconnect | Node running (1); observed usage | Addendum to `keep-local-synced-willow-store.md` |
| 6 | Decide which change wins when two devices edit the same content | Profile Bundle working (4), then a real two-device sync exercise | New record replacing the store record's "conflict merges out of scope" note |
| 7 | Rewire boot and sign-in to the SQLite identity store and delete `allowed_dids.json` | Node store schema live (1); SR1 accounts re-conscired through the node's conscire endpoint | Addendum to `exosystems-exoauth.md` |
| 8 | Retry the `sqlx` step to 0.9.0 | Locked `iroh` line dropping the `sha2` pre-release pin | Addendum to `choose-node-database-for-identity-and-willow-state.md` |
| 9 | Place the Brevo API key in the instance's credential file beside the SMTP pair | Credential handling decision | Addendum to `provision-the-authenticated-settings-profile.md` |
| 10 | Build the console's mail operations section: the domain's mailbox list with its create and remove actions | The instance's mail stack (done); the API key (9) | Addendum to `provision-the-authenticated-settings-profile.md` |

Rule: nothing leaves this list silently — each row closes with a
dated addendum or a superseding record, never a quiet edit.
