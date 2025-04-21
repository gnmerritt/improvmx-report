#!/bin/bash -e

source .env

if [ -z "$PS1" ]; then
  exec >> "$LOG"
  exec 2>&1
fi

cd "$(dirname "$0")"
cargo build --release

echo "Starting email report at $(date)"
./target/release/improvmx-report
