#!/bin/sh
set -e

SERVER_BIN="${SERVER_BIN:-/app/release/server}"
CLIENT_BIN="${CLIENT_BIN:-/app/release/client}"

"$SERVER_BIN" & SERVER_PID=$! # background
"$CLIENT_BIN"

kill "$SERVER_PID" 2>/dev/null || true
