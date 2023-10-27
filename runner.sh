#!/bin/bash -e

cd "$(dirname "$0")"
cargo build --release

source .env
./target/release/improvmx-report
