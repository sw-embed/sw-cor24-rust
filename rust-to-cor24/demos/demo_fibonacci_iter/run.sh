#!/bin/bash
# Run this demo through the full Rust→COR24 pipeline
DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$(dirname "$DIR")/run-demo.sh" "$(basename "$DIR")" "$@"
