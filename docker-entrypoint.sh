#!/bin/bash

# Create sane default config for a docker deployment
export AARDWOLF_CONFIG=/etc/aardwolf/aardwolf.toml

export AARDWOLF_WEB_HOST=0.0.0.0
export AARDWOLF_LOG_FILE=_CONSOLE_
export AARDWOLF_DATABASE_MIGRATIONS=/etc/aardwolf/migrations

# NOTE: Would be nice if all the asset stuff was in one directory and a single config value.
export AARDWOLF_ASSETS_WEB=/etc/aardwolf/assets/dist
export AARDWOLF_ASSETS_THEMES=/etc/aardwolf/assets/web/themes
export AARDWOLF_ASSETS_EMOJI=/etc/aardwolf/assets/web/emoji
export AARDWOLF_ASSETS_IMAGES=/etc/aardwolf/assets/web/images
export AARDWOLF_ASSETS_STYLESHEETS=/etc/aardwolf/assets/web/stylesheets
export AARDWOLF_TEMPLATES_DIR=/etc/aardwolf/assets/templates/**/*

if [[ -z $AARDWOLF_DATABASE_HOST ]]; then
  echo "ERROR: AARDWOLF_DATABASE_HOST is not set"
  return 1
fi

# Run migrations
aardwolf-setup

# Run server
aardwolf-server "$@"
