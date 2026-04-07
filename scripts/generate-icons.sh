#!/bin/bash
# Requires: imagemagick
# Usage: ./scripts/generate-icons.sh path/to/source.png
set -e
SOURCE=${1:-"public/tray-icon.png"}
ICONS_DIR="src-tauri/icons"
mkdir -p "$ICONS_DIR"
for SIZE in 32 128 256 512; do
  convert "$SOURCE" -resize ${SIZE}x${SIZE} "$ICONS_DIR/${SIZE}x${SIZE}.png"
done
convert "$SOURCE" -resize 256x256 "$ICONS_DIR/128x128@2x.png"
echo "Icons generated in $ICONS_DIR/"
