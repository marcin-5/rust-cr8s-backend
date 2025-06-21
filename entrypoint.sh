#!/bin/bash
set -e

FISH_CONFIG_DIR="/root/.config/fish"
DEFAULT_CONFIG_DIR="/opt/fish_config_default"

# If the config directory is mounted but uninitialized (i.e., fisher is missing),
# copy the default config from the image into the volume.
if [ ! -f "$FISH_CONFIG_DIR/functions/fisher.fish" ]; then
   echo "Initializing fish config in $FISH_CONFIG_DIR..."
   # Ensure the target functions directory exists before copying
   mkdir -p "$FISH_CONFIG_DIR/functions"
   cp -a "$DEFAULT_CONFIG_DIR/." "$FISH_CONFIG_DIR/"
fi

# Execute the command passed to the container
exec "$@"
