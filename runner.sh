#!/bin/bash -e

cd "$(dirname "$0")"
source .env

if [ -z "$PS1" ]; then
  exec >> "$LOG"
  exec 2>&1
fi

cargo build --release

echo "Starting email report at $(date)"
./target/release/improvmx-report
