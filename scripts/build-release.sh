#!/bin/bash
set -e
echo "Building WhatsApp Linux Desktop..."
cd "$(dirname "$0")/.."
npm install
npm run build
cargo tauri build
echo "Build complete. Packages:"
ls -la src-tauri/target/release/bundle/deb/ 2>/dev/null || true
ls -la src-tauri/target/release/bundle/appimage/ 2>/dev/null || true
