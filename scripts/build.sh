#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo test
cargo build --release
echo "Build complete. Binaries in target/release/"
