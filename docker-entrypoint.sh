#!/bin/bash
set -me

SCRIPT_NAME=`basename "$0"`
AARDWOLF_SKIP_ENTRYPOINT=${AARDWOLF_SKIP_ENTRYPOINT:-false}

echo "$SCRIPT_NAME: Beginning initialization..."

case ${AARDWOLF_SKIP_ENTRYPOINT,,} in
  true|1)
    echo "$SCRIPT_NAME: Skipping entrypoint. Starting container..."
    exec "$@"
    exit $?
    ;;
esac

{{ flightdeck_run_commands }}

echo "$SCRIPT_NAME: Initialization complete. Starting container..."

exec "$@"


