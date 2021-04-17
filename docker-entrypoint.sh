#!/bin/bash
set -me

SCRIPT_NAME=`basename "$0"`
AARDWOLF_SETUP_DB=${AARDWOLF_SETUP_DB:-false}

case ${AARDWOLF_SKIP_ENTRYPOINT,,} in
  true|1)
		echo "$SCRIPT_NAME: Beginning initialization..."
    exec "cargo run --bin setup"
    echo "$SCRIPT_NAME: Database configured. Starting container..."
		exec "$@"
    exit $?
    ;;
esac

echo "$SCRIPT_NAME: Starting container..."
exec "$@"
