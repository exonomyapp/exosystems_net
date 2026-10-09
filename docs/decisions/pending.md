# Pending decisions

Decisions still open, what unblocks each, and where each lands.
Reviewed whenever the work-report's next steps change.

| # | Decision | Blocked on | Lands in |
|---|----------|------------|----------|
| 1 | Provision the plain node on the instance by hand | Instance access window; helper identity generation | `run-full-conscia-node-on-exosystems-net.md` addendum with live evidence |
| 2 | Create the GitHub repo and turn on the pipeline | Owner/repo name | `set-up-github-repo-and-ci-pipeline.md` addendum on first green deploy |
| 3 | Build challenge-response secret ownership | Sign-in flow settled (done); crypto review of nonce store | `prove-secret-ownership-at-sign-in.md` Accepted |
| 4 | Build Profile Bundle export/import | Secret ownership (3); second-device test | `recover-and-move-exotalk-profile-across-devices.md` Accepted |
| 5 | Sync cadence beyond sign-in and reconnect | Node running (1); observed usage | Addendum to `keep-local-synced-willow-store.md` |
| 6 | Decide which change wins when two devices edit the same content | Profile Bundle working (4), then a real two-device sync exercise | New record replacing the store record's "conflict merges out of scope" note |
| 7 | Rewire boot and sign-in to the SQLite identity store and delete `allowed_dids.json` | Node store schema live (1); R1 accounts re-conscired through the node's conscire endpoint | Addendum to `exosystems-exoauth.md` |
| 8 | Retry the `sqlx` step to 0.9.0 | Locked `iroh` line dropping the `sha2` pre-release pin | Addendum to `choose-node-database-for-identity-and-willow-state.md` |

Rule: nothing leaves this list silently — each row closes with a
dated addendum or a superseding record, never a quiet edit.
