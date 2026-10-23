# Local workstation notes (out of instance scope)

Work done on this machine during the instance-hardening work. Recorded here so the instance guide stays instance-only. Recommendations are proposals, not applied work.

## Done

- Generated SSH keypair for Singapore instance access: private half `~/.ssh/alibaba_sg_ecs` (mode 600), public half imported to Alibaba Cloud as keypair `sg-ecs-admin-20261009` and attached to instance `i-t4n5ee5yucj5f84co4s4` on 2026-10-09.
- Tightened secret file permissions to owner-only: the Alibaba API key file, the Model Studio workspace CSV (both since moved under `exosystems_net/secrets/`), the OpenCode config (reverted to its original permissions afterward as out-of-scope touching), the Alibaba CLI store, and the new SSH private key.
- Moved the Alibaba API key file and the Model Studio workspace CSV from Downloads into `exosystems_net/secrets/` (git-ignored). Consequence: anything referencing the old Downloads paths, such as the models round script's key-file setting, needs its path updated.

## Proposed (not done)

- Sweep this machine for other world-readable secret files (deploy keys, password stores, API keys in configs) and set owner-only permissions.
- Keep long-lived secrets out of Downloads; land them in the ignored project folder at receipt.
