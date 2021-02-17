#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR=$(dirname $BASH_SOURCE)
$SCRIPT_DIR/build-wasm.sh
$SCRIPT_DIR/build-cli.sh