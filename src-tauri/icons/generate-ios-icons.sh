#!/bin/bash

# Generate iOS app icons from source 1024px icon
# This script can be run from anywhere and will work with relative paths

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Define paths relative to script location
SOURCE_ICON="$SCRIPT_DIR/ios-1024.png"
TARGET_DIR="$SCRIPT_DIR/../gen/apple/Assets.xcassets/AppIcon.appiconset"

# Check if source icon exists
if [ ! -f "$SOURCE_ICON" ]; then
    echo "❌ Error: Source icon not found at $SOURCE_ICON"
    exit 1
fi

# Check if target directory exists
if [ ! -d "$TARGET_DIR" ]; then
    echo "❌ Error: Target directory not found at $TARGET_DIR"
    echo "   Run 'npx tauri ios init' first to create the iOS project structure"
    exit 1
fi

echo "🎨 Generating iOS app icons from $SOURCE_ICON"
echo "📁 Target: $TARGET_DIR"
echo "⚠️  Existing icons will be overwritten"
echo ""

cd "$TARGET_DIR" || exit 1

# Generate all required iOS icon sizes
sips -z 20 20 "$SOURCE_ICON" --out AppIcon-20x20@1x.png > /dev/null
echo "✓ Generated 20x20@1x"

sips -z 40 40 "$SOURCE_ICON" --out AppIcon-20x20@2x.png > /dev/null
sips -z 40 40 "$SOURCE_ICON" --out AppIcon-20x20@2x-1.png > /dev/null
echo "✓ Generated 20x20@2x"

sips -z 60 60 "$SOURCE_ICON" --out AppIcon-20x20@3x.png > /dev/null
echo "✓ Generated 20x20@3x"

sips -z 29 29 "$SOURCE_ICON" --out AppIcon-29x29@1x.png > /dev/null
echo "✓ Generated 29x29@1x"

sips -z 58 58 "$SOURCE_ICON" --out AppIcon-29x29@2x.png > /dev/null
sips -z 58 58 "$SOURCE_ICON" --out AppIcon-29x29@2x-1.png > /dev/null
echo "✓ Generated 29x29@2x"

sips -z 87 87 "$SOURCE_ICON" --out AppIcon-29x29@3x.png > /dev/null
echo "✓ Generated 29x29@3x"

sips -z 40 40 "$SOURCE_ICON" --out AppIcon-40x40@1x.png > /dev/null
echo "✓ Generated 40x40@1x"

sips -z 80 80 "$SOURCE_ICON" --out AppIcon-40x40@2x.png > /dev/null
sips -z 80 80 "$SOURCE_ICON" --out AppIcon-40x40@2x-1.png > /dev/null
echo "✓ Generated 40x40@2x"

sips -z 120 120 "$SOURCE_ICON" --out AppIcon-40x40@3x.png > /dev/null
echo "✓ Generated 40x40@3x"

sips -z 120 120 "$SOURCE_ICON" --out AppIcon-60x60@2x.png > /dev/null
echo "✓ Generated 60x60@2x"

sips -z 180 180 "$SOURCE_ICON" --out AppIcon-60x60@3x.png > /dev/null
echo "✓ Generated 60x60@3x"

sips -z 76 76 "$SOURCE_ICON" --out AppIcon-76x76@1x.png > /dev/null
echo "✓ Generated 76x76@1x"

sips -z 152 152 "$SOURCE_ICON" --out AppIcon-76x76@2x.png > /dev/null
echo "✓ Generated 76x76@2x"

sips -z 167 167 "$SOURCE_ICON" --out AppIcon-83.5x83.5@2x.png > /dev/null
echo "✓ Generated 83.5x83.5@2x"

sips -z 1024 1024 "$SOURCE_ICON" --out AppIcon-512@2x.png > /dev/null
echo "✓ Generated 1024x1024 (App Store)"

echo ""
echo "✅ All iOS app icons generated successfully!"
echo "   Clean build in Xcode (⇧⌘K) and rebuild to see changes"

