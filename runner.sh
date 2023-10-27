#!/bin/bash -e

cd "$(dirname "$0")"
cargo build -r

source .env
./target/release/improvmx-report
