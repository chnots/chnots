#!/usr/bin/env bash

set -eux

CSDIR="$(dirname "$(realpath "$0")")"
SVG="$CSDIR/../data/icon/chnots.svg"
OUTPUT_PREFIX="$CSDIR/../tauri/icons/icon"

SIZES=(256)

for size in "${SIZES[@]}"; do
    inkscape "$SVG" --export-type=png --export-filename="${OUTPUT_PREFIX}.png" --export-width="$size" --export-height="$size"
    magick "${OUTPUT_PREFIX}.png" "${OUTPUT_PREFIX}.ico"
    magick "${OUTPUT_PREFIX}.png" "${OUTPUT_PREFIX}.icns"
done
