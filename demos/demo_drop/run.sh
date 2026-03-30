#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$(dirname "$DIR")/run-demo.sh" demo_drop "$@"
