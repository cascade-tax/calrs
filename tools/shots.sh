#!/usr/bin/env bash
#
# calrs visual review harness.
#
#   ./tools/shots.sh                     # build if needed, reseed, serve, capture everything
#   ./tools/shots.sh --only admin        # only routes whose name/path contains "admin"
#   ./tools/shots.sh --outdir /tmp/foo   # write PNGs somewhere else
#   ./tools/shots.sh --no-seed           # keep the existing demo DB
#   ./tools/shots.sh --no-build          # skip cargo, use the existing binary
#   ./tools/shots.sh --keep-server       # leave the server running afterwards
#   ./tools/shots.sh --combos desktop-dark,mobile-dark
#
# Idempotent and re-runnable: it kills anything already listening on $PORT,
# rebuilds only if the binary is stale, and reseeds a throwaway DB.
#
# Login for manual poking:  alice@example.com / password1234
# (calrs requires >= 12 character passwords, so this is NOT "password123".)
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO"

PORT="${CALRS_SHOTS_PORT:-3999}"
DATA_DIR="${CALRS_SHOTS_DATA_DIR:-/tmp/calrs-screenshots}"
OUTDIR="/tmp/calrs-shots"
PASSWORD="${CALRS_SHOTS_PASSWORD:-password1234}"
BIN="$REPO/target/release/calrs"
LOG="/tmp/calrs-shots-server.log"
CHROME="${CALRS_SHOTS_CHROME:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"

DO_SEED=1
DO_BUILD=1
KEEP_SERVER=0
NODE_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-seed)     DO_SEED=0; shift ;;
    --no-build)    DO_BUILD=0; shift ;;
    --keep-server) KEEP_SERVER=1; shift ;;
    --outdir)      OUTDIR="$2"; NODE_ARGS+=(--outdir "$2"); shift 2 ;;
    --only)        NODE_ARGS+=(--only "$2"); shift 2 ;;
    --combos)      NODE_ARGS+=(--combos "$2"); shift 2 ;;
    -h|--help)     sed -n '2,20p' "${BASH_SOURCE[0]}"; exit 0 ;;
    *)             echo "unknown option: $1" >&2; exit 2 ;;
  esac
done

# Make sure --outdir is passed through even when the default is used.
if [[ ! " ${NODE_ARGS[*]:-} " == *" --outdir "* ]]; then
  NODE_ARGS+=(--outdir "$OUTDIR")
fi

export PATH="$HOME/.cargo/bin:$PATH"

say() { printf '\n\033[1m== %s\033[0m\n' "$*"; }

# ── 1. Build ──────────────────────────────────────────────────────────
say "Build"
NEEDS_BUILD=0
if [[ "$DO_BUILD" == 0 ]]; then
  if [[ ! -x "$BIN" ]]; then
    echo "--no-build given but $BIN does not exist" >&2
    exit 1
  fi
  echo "(--no-build: using existing $BIN)"
elif [[ ! -x "$BIN" ]]; then
  NEEDS_BUILD=1
elif [[ -n "$(find src migrations templates i18n Cargo.toml Cargo.lock -newer "$BIN" -print -quit 2>/dev/null)" ]]; then
  NEEDS_BUILD=1
fi
if [[ "$DO_BUILD" == 1 ]]; then
  if [[ "$NEEDS_BUILD" == 1 ]]; then
    echo "building release binary…"
    cargo build --release
  else
    echo "release binary is up to date: $BIN"
  fi
fi

# ── 2. Free the port ──────────────────────────────────────────────────
say "Server on port $PORT"
OLD_PIDS="$(lsof -ti tcp:"$PORT" 2>/dev/null || true)"
if [[ -n "$OLD_PIDS" ]]; then
  echo "killing existing listener(s) on :$PORT -> $OLD_PIDS"
  # shellcheck disable=SC2086
  kill $OLD_PIDS 2>/dev/null || true
  for _ in $(seq 1 20); do
    lsof -ti tcp:"$PORT" >/dev/null 2>&1 || break
    sleep 0.25
  done
  # shellcheck disable=SC2086
  kill -9 $(lsof -ti tcp:"$PORT" 2>/dev/null) 2>/dev/null || true
fi

# ── 3. Seed ───────────────────────────────────────────────────────────
if [[ "$DO_SEED" == 1 ]]; then
  say "Seed demo data into $DATA_DIR"
  CALRS_SHOTS_PASSWORD="$PASSWORD" python3 "$REPO/tools/seed.py" \
    --binary "$BIN" --data-dir "$DATA_DIR" --password "$PASSWORD"
else
  echo "(--no-seed: reusing $DATA_DIR/calrs.db)"
fi

# ── 4. Serve ──────────────────────────────────────────────────────────
CALRS_BASE_URL="http://127.0.0.1:$PORT" \
  "$BIN" serve --port "$PORT" --data-dir "$DATA_DIR" >"$LOG" 2>&1 &
SERVER_PID=$!

cleanup() {
  if [[ "$KEEP_SERVER" == 0 ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "server PID: $SERVER_PID  (log: $LOG)"
for _ in $(seq 1 120); do
  if curl -fsS -o /dev/null "http://127.0.0.1:$PORT/auth/login"; then
    UP=1; break
  fi
  sleep 0.25
done
if [[ "${UP:-0}" != 1 ]]; then
  echo "server failed to start; last 40 log lines:" >&2
  tail -40 "$LOG" >&2
  exit 1
fi
echo "server is up at http://127.0.0.1:$PORT"

# ── 5. Node deps ──────────────────────────────────────────────────────
if [[ ! -d "$REPO/tools/node_modules/puppeteer-core" ]]; then
  say "Installing puppeteer-core into tools/"
  (cd "$REPO/tools" && npm install --no-audit --no-fund)
fi

# ── 6. Capture ────────────────────────────────────────────────────────
say "Capture"
set +e
CALRS_SHOTS_CHROME="$CHROME" \
CALRS_SHOTS_PASSWORD="$PASSWORD" \
node "$REPO/tools/shots.mjs" \
  --base "http://127.0.0.1:$PORT" \
  --db "$DATA_DIR/calrs.db" \
  "${NODE_ARGS[@]}"
RC=$?
set -e

say "Done"
echo "server PID: $SERVER_PID"
if [[ "$KEEP_SERVER" == 1 ]]; then
  echo "server left running at http://127.0.0.1:$PORT (kill $SERVER_PID to stop it)"
  trap - EXIT
else
  echo "shutting the server down"
fi
echo "screenshots: $OUTDIR"
echo "login:       alice@example.com / $PASSWORD"

exit "$RC"
