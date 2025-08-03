#!/usr/bin/env bash

set -euxo pipefail

# Constants
declare -A ICON_SIZES=(
    ["16x16.png"]="16"
    ["32x32.png"]="32"
    ["48x48.png"]="48"
    ["64x64.png"]="64"
    ["128x128.png"]="128"
    ["128x128@2x.png"]="256"
    ["Square30x30Logo.png"]="30"
    ["Square44x44Logo.png"]="44"
    ["Square71x71Logo.png"]="71"
    ["Square89x89Logo.png"]="89"
    ["Square107x107Logo.png"]="107"
    ["Square142x142Logo.png"]="142"
    ["Square150x150Logo.png"]="150"
    ["Square284x284Logo.png"]="284"
    ["Square310x310Logo.png"]="310"
    ["Square71x71Logo.png"]="71"
    ["StoreLogo.png"]="50"
    ["ios/icon-20.png"]="20"
    ["ios/icon-20@2x.png"]="40"
    ["ios/icon-20@3x.png"]="60"
    ["ios/icon-29.png"]="29"
    ["ios/icon-29@2x.png"]="58"
    ["ios/icon-29@3x.png"]="87"
    ["ios/icon-40.png"]="40"
    ["ios/icon-40@2x.png"]="80"
    ["ios/icon-40@3x.png"]="120"
    ["ios/icon-60@2x.png"]="120"
    ["ios/icon-60@3x.png"]="180"
    ["ios/icon-76.png"]="76"
    ["ios/icon-76@2x.png"]="152"
    ["ios/icon-83.5@2x.png"]="167"
)

# Check dependencies
check_dependencies() {
    local missing=()
    command -v inkscape >/dev/null 2>&1 || missing+=("inkscape")
    command -v magick >/dev/null 2>&1 || missing+=("imagemagick")

    # command -v iconutil >/dev/null 2>&1 || missing+=("iconutil (macOS only)")
    
    if [ ${#missing[@]} -ne 0 ]; then
        echo "Error: Missing required tools:"
        printf " - %s\n" "${missing[@]}"
        exit 1
    fi
}

# Validate SVG file
validate_svg() {
    local svg_file=$1
    if [ ! -f "$svg_file" ]; then
        echo "Error: SVG file not found at $svg_file"
        exit 1
    fi
    
    if ! file "$svg_file" | grep -qi "svg"; then
        echo "Error: Not a valid SVG file: $svg_file"
        exit 1
    fi
}

# Generate all PNGs directly from SVG
generate_pngs() {
    local svg_file=$1
    local output_dir=$2
    
    echo "Generating PNGs directly from SVG..."
    
    for filename in "${!ICON_SIZES[@]}"; do
        local size=${ICON_SIZES[$filename]}
        local output_path="$output_dir/$filename"
        
        # Create directory if needed
        mkdir -p "$(dirname "$output_path")"
        
        echo "Creating $filename (${size}x${size}) from SVG..."
        inkscape "$svg_file" \
            --export-type=png \
            --export-filename="$output_path" \
            --export-width="$size" \
            --export-height="$size"
    done
}

# Generate ICO file (Windows)
generate_ico() {
    local output_dir=$1
    echo "Generating icon.ico..."
    
    magick convert \
        "$output_dir/16x16.png" \
        "$output_dir/32x32.png" \
        "$output_dir/48x48.png" \
        "$output_dir/64x64.png" \
        "$output_dir/128x128.png" \
        -background transparent \
        "$output_dir/icon.ico"
}

# Generate ICNS file (macOS)
generate_icns() {
    local output_dir=$1
    echo "Generating icon.icns..."
    
    # Create temporary iconset directory
    local iconset_dir="$output_dir/temp.iconset"
    mkdir -p "$iconset_dir"
    
    # Generate all required sizes for ICNS format
    declare -A required_sizes=(
        ["icon_16x16.png"]="16"
        ["icon_16x16@2x.png"]="32"
        ["icon_32x32.png"]="32"
        ["icon_32x32@2x.png"]="64"
        ["icon_128x128.png"]="128"
        ["icon_128x128@2x.png"]="256"
        ["icon_256x256.png"]="256"
        ["icon_256x256@2x.png"]="512"
        ["icon_512x512.png"]="512"
        ["icon_512x512@2x.png"]="1024"
    )
    
    # Generate each size directly from SVG
    for icon_name in "${!required_sizes[@]}"; do
        local size=${required_sizes[$icon_name]}
        inkscape "$svg_file" \
            --export-type=png \
            --export-filename="$iconset_dir/$icon_name" \
            --export-width="$size" \
            --export-height="$size"
    done
    
    # Convert to ICNS format
    iconutil --convert icns --output "$output_dir/icon.icns" "$iconset_dir"
    
    # Clean up
    rm -rf "$iconset_dir"
}

# Main function
main() {
    check_dependencies
    
    local script_dir="$(dirname "$(realpath "$0")")"
    local default_svg="${script_dir}/../data/icon/chnots.svg"
    local default_output="${script_dir}/../tauri/src-tauri/icons"
    
    local svg_file="${1:-$default_svg}"
    local output_dir="${2:-$default_output}"
    
    validate_svg "$svg_file"
    
    # Create output directory
    mkdir -p "$output_dir"
    
    # Generate all PNGs directly from SVG
    generate_pngs "$svg_file" "$output_dir"
    
    # Generate ICO file
    generate_ico "$output_dir"
    
    # Generate ICNS file
    # generate_icns "$output_dir"
    
    # Copy main icon
    cp "$output_dir/128x128.png" "$output_dir/icon.png"
    
    echo "Icon generation complete!"
    echo "Output directory: $output_dir"
}

main "$@"
