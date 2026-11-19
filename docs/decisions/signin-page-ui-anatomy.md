# Sign-In Page UI Anatomy

## Status

Proposed

## Scope

This ADR defines the complete UI anatomy, features, and functions of the sign-in page (`/signin`) for Exosystems.net. It is the decision record for the SigninPage component in `exosystems_net/src/app.rs`.

---

## Page Structure

```
SigninPage
├── Header: "Exosystems.net"
├── Two-Column Layout (side by side)
│   ├── Left Column: "Sign in with existing identity"
│   │   ├── Identity Card List (known did:peer accounts from localStorage)
│   │   │   ├── Card: did:peer (truncated) + "Last used: <date>"
│   │   │   ├── Card: [Select] button → proceeds to challenge-response
│   │   │   └── Card: [Save secret] checkbox (persisted per card via exo_save_secret_signin)
│   │   └── Manual Entry: Text input for pasting did:peer + [Continue] button
│   │
│   └── Right Column: "Create new identity"
│       ├── [Generate identity] button
│       │   └── On click: creates Ed25519 keypair, derives did:peer:2.Vz..., shows recovery secret ONCE
│       ├── Recovery Secret Display (modal/alert)
│       │   ├── Secret string (base58)
│       │   ├── "Copy" button
│       │   ├── "I have saved this" checkbox (required to proceed)
│       │   └── [Continue] button → proceeds to challenge-response with new identity
│       └── [Save secret] checkbox (persisted per card via exo_save_secret_create)
│
├── Challenge-Response Step (shown after identity selected/created)
│   ├── Server nonce display (hidden, auto-handled)
│   ├── Client signs with secret (auto if secret saved, manual prompt if not)
│   └── Progress indicator
│
└── Footer: Links to recovery, help
```

---

## Identity Card (Known Accounts)

Each known `did:peer` from `localStorage.exo_sessions` renders as a card:

| Element | Behavior |
|---------|----------|
| Truncated DID | `did:peer:2.Vz...Yy` (first 12 + last 4 chars) |
| Last used timestamp | From session metadata |
| [Select] button | Initiates challenge-response (Step 2 of ExoAuth flow) |
| [Save secret] checkbox | Persists to `exo_save_secret_signin` per card; if checked, skips manual secret entry on next sign-in |

**Persistence**: Cards survive browser reloads. `exo_sessions` array stores session objects per identity. `exo_current` tracks active index.

---

## Create New Identity Flow

| Step | UI | Persistence |
|------|-----|-------------|
| 1. Click "Generate identity" | Button disabled during generation | — |
| 2. Secret displayed | Modal with base58 secret, copy button, "I have saved this" checkbox (required) | Secret NOT stored unless user checks "Save secret" |
| 3. "Save secret" checkbox | If checked → secret stored in browser via `store_saved_secret()` | `exo_save_secret_create` per card |
| 4. Click "Continue" | Proceeds to challenge-response with new identity | New identity added to `exo_sessions` |

**Security**: Secret shown **once only**. No recovery from UI if not saved.

---

## Challenge-Response Step

| Condition | UI |
|-----------|-----|
| Secret saved (checkbox was checked) | Automatic — no user interaction, immediate verify |
| Secret not saved | Prompt: "Enter your recovery secret" + text input + [Verify] button |
| Verification success | Full page reload → signed-in state visible from first paint |
| Verification failure | Inline error: "Invalid secret. Try again or select another identity." |

---

## Save-Secret Behavior (Per-Card)

| Checkbox | Key | Scope |
|----------|-----|-------|
| Sign-in column | `exo_save_secret_signin` | Per known identity card |
| Create column | `exo_save_secret_create` | Per newly created identity |

**Implementation**: `exosystems_net/src/auth.rs` — `store_flag(key, value)`, `read_flag(key)` → `localStorage`

---

## Signed-In State Transition

On successful verification:

1. Session token created and stored in `localStorage.exo_session`
2. Identity added/updated in `localStorage.exo_sessions`
3. `localStorage.exo_current` set to new identity index
4. **Full page reload** (`window.location.reload()`)
5. App reads `exo_current` → renders signed-in UI (profile, gated docs, conscire)

---

## Accessibility & UX

- Keyboard navigation: Tab order follows visual layout (left column → right column → actions)
- Screen readers: ARIA labels on identity cards, secret modal, form inputs
- Error states: Inline, non-toast, persistent until corrected
- Loading states: Buttons disabled, spinner shown during crypto operations

---

## Implementation Reference

| File | Location |
|------|----------|
| SigninPage component | `exosystems_net/src/app.rs:588-849` |
| Auth helpers (flags, secrets, session) | `exosystems_net/src/auth.rs` |
| Identity engine (Rust) | `exotalk/exoauth/rust/src/api/identity.rs` |
| ExoAuth flow authority | `exotalk/docs/decisions/exoauth-flow-steps.md` |

---

## Related Decisions

- `exosystems_net/docs/decisions/exosystems-exoauth.md` — identity kinds, session model
- `exosystems_net/docs/decisions/prove-secret-ownership-at-sign-in.md` — challenge-response
- `exosystems_net/docs/decisions/build-fresh-boot-and-sign-in-on-node-store.md` — node boot/sign-in
- `exosystems_net/docs/decisions/keep-local-synced-willow-store.md` — Willow store resolution
- `exosystems_net/docs/decisions/provision-the-authenticated-settings-profile.md` — settings profile