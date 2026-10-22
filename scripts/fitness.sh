#!/bin/sh
# Fitness checks for exosystems.net decisions.
# Each check proves a recorded decision still holds. Any FAIL
# exits non-zero; run on every build and in CI (see
# decisions/set-up-github-repo-and-ci-pipeline.md).
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0

pass() { echo "PASS: $1"; }
fail() { echo "FAIL: $1"; FAIL=1; }

# F1 — motion rule (use-unstyled-primitives-for-ui-stack.md):
# no animation code under src/ may execute during server rendering.
if grep -r -i "motion" "$ROOT/src/" "$ROOT/Cargo.toml" >/dev/null 2>&1; then
  fail "F1 motion token found under src/ or Cargo.toml"
else
  pass "F1 no motion code in server sources"
fi

# F2 — no static allowlist; boot and sign-in read the SQLite identity store.
# allowed_dids.json was deleted when the SQLite store landed. Its presence is a
# failure; its absence and a present migration file are the passing state.
if [ -f "$ROOT/allowed_dids.json" ]; then
  fail "F2 static allowlist still present at allowed_dids.json; it must not exist"
elif [ -f "$ROOT/migrations/0001_identity_store.sql" ]; then
  pass "F2 allowlist removed; SQLite identity-store migration present"
else
  fail "F2 neither an allowlist nor an identity-store migration found"
fi

# F3 — decision log integrity (decisions/README.md):
# every record file must appear in the index.
MISSING=""
for f in "$ROOT"/docs/decisions/*.md; do
  name=$(basename "$f")
  [ "$name" = "README.md" ] && continue
  grep -q "($name)" "$ROOT/docs/decisions/README.md" || MISSING="$MISSING $name"
done
if [ -z "$MISSING" ]; then
  pass "F3 decision index lists every record"
else
  fail "F3 index missing:$MISSING"
fi

# F4 — gate routes present (exosystems-exoauth.md, SR1):
# the app must expose sign-in and the gated docs view.
if grep -q '"signin"' "$ROOT/src/app.rs" && grep -q '"docs"' "$ROOT/src/app.rs" && grep -q '"models"' "$ROOT/src/app.rs" && grep -q '"settings"' "$ROOT/src/app.rs"; then
  pass "F4 sign-in, docs, models, settings routes present"
else
  fail "F4 sign-in, docs, models, or settings route missing from src/app.rs"
fi

[ "$FAIL" -eq 0 ] && echo "ALL FITNESS CHECKS PASS" || echo "FITNESS FAILURES PRESENT"
exit "$FAIL"
