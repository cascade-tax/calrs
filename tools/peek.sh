#!/usr/bin/env bash
# Fast single-page look at the dev server, for iterating on templates.
#
#   tools/peek.sh /auth/login            → /tmp/peek.png, light, 1200 wide
#   tools/peek.sh /auth/login dark 420   → dark, mobile width
#
# minijinja caches templates per Environment, so the server is restarted on
# every call; template edits would otherwise not show up.
set -euo pipefail

URL_PATH="${1:-/auth/login}"
THEME="${2:-light}"
WIDTH="${3:-1200}"
HEIGHT="${4:-900}"
PORT=3997
DATA_DIR=/tmp/calrs-mydev
OUT="${OUT:-/tmp/peek.png}"
CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"

cd "$(dirname "$0")/.."
pkill -f "calrs serve --port $PORT" 2>/dev/null || true
sleep 0.3
mkdir -p "$DATA_DIR"
CALRS_DATA_DIR="$DATA_DIR" ./target/debug/calrs serve --port "$PORT" \
  >"$DATA_DIR/server.log" 2>&1 &
for _ in $(seq 1 40); do
  curl -sf -o /dev/null "http://localhost:$PORT/auth/login" && break
  sleep 0.25
done

SCHEME=1
[ "$THEME" = "dark" ] && SCHEME=2

"$CHROME" --headless --disable-gpu --hide-scrollbars \
  --force-device-scale-factor=2 \
  --blink-settings=preferredColorScheme=$SCHEME \
  --window-size="$WIDTH,$HEIGHT" \
  --virtual-time-budget=4000 \
  --screenshot="$OUT" \
  "http://localhost:$PORT$URL_PATH" 2>/dev/null || true

echo "$OUT"
