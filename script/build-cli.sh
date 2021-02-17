#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

echo "* building wand-cli"
cargo build -p wand-cli --release