#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

echo "* running wand-cli"
./target/release/wand-cli $@