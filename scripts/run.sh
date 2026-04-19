#!/bin/sh
set -e

./server & SERVER_PID=$!
./client

kill "$SERVER_PID" 2>/dev/null || true
