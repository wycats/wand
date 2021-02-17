#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR=$(dirname $BASH_SOURCE)
$SCRIPT_DIR/build.sh
$SCRIPT_DIR/run.sh