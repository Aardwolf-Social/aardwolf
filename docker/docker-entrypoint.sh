#!/bin/bash
set -me

SCRIPT_NAME=`basename "$0"`
AARDWOLF_SETUP_DB=${AARDWOLF_SETUP_DB:-false}
cd app

echo "$SCRIPT_NAME: Setting Rust Override Nightly"
rustup override set nightly

case ${AARDWOLF_SETUP_DB,,} in
  true|1)
		echo "$SCRIPT_NAME: Beginning initialization..."
		cargo install -f diesel_cli --no-default-features --features postgres
    cargo run --bin setup 
    echo "$SCRIPT_NAME: Database configured. Starting container..."
		exec "$@"
    exit $?
    ;;
esac

echo "$SCRIPT_NAME: Starting container..."
exec "$@"
