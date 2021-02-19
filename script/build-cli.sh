#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR=$(dirname $BASH_SOURCE)
ROOT=$(git rev-parse --show-toplevel)

. "$SCRIPT_DIR/utils.sh" --source-only

if [[ !$(compare_sha $ROOT/script/wand-cli.checksum.blake2 $ROOT/crates/wand-cli) ]]; then
  echo "no change"
  exit 0
fi

echo "* building wand-cli"
cargo build --release -p wand-cli
