#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

echo "* building wand.wasm"
# cargo build --target wasm32-unknown-unknown -p wand
wasm-pack build --release

# SOURCE=target/wasm32-unknown-unknown/debug/wand.wasm
SOURCE=pkg/wand_bg.wasm
DATA=crates/wand-cli/data/wand.wasm

OLD_SHA=$(sha256sum $DATA | cut -c -64)
NEW_SHA=$(sha256sum $SOURCE | cut -c -64)

if [[ $OLD_SHA = $NEW_SHA ]]; then
  echo "  - no changes, skipping copy"
else
  echo "  - copying pkg/wand_bg.wasm -> crates/wand-cli/data/wand.wasm"
  cp $SOURCE $DATA
fi
