#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/codewen/scripts/debug-codewen.sh" in VSCode settings to always get the 
# latest codewen-rs binary when debugging Codewen Extension.


set -euo pipefail

CODEWEN_RS_DIR=$(realpath "$(dirname "$0")/../codewen-rs")
(cd "$CODEWEN_RS_DIR" && cargo run --quiet --bin codewen -- "$@")