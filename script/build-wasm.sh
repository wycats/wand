#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR=$(dirname $BASH_SOURCE)
ROOT=$(git rev-parse --show-toplevel)

. "$SCRIPT_DIR/utils.sh" --source-only

if [[ !$(compare_sha $ROOT/script/wand.checksum.blake2 $ROOT/crates/wand) ]]; then
  echo "no change"
  exit 0
fi

echo "* building wand.wasm"
# # cargo build --target wasm32-unknown-unknown -p wand
wasm-pack build $ROOT/crates/wand --release
SOURCE=pkg/wand_bg.wasm
DATA=crates/wand-cli/data/wand.wasm

echo "  - copying pkg/wand_bg.wasm -> crates/wand-cli/data/wand.wasm"
cp $SOURCE $DATA
