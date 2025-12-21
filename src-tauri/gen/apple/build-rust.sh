#!/bin/bash
# Build script for Rust code in iOS project
# This script dynamically detects Rust toolchain paths to avoid hardcoded values

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source build configuration constants
if [ -f "$SCRIPT_DIR/build-config.sh" ]; then
    source "$SCRIPT_DIR/build-config.sh"
else
    # Fallback if config file doesn't exist
    RUST_TOOLCHAIN_NAME="${RUST_TOOLCHAIN_NAME:-stable-aarch64-apple-darwin}"
    RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
    HOMEBREW_PREFIX="${HOMEBREW_PREFIX:-/opt/homebrew}"
    RUST_TOOLCHAIN_PATH="$RUSTUP_HOME/toolchains/$RUST_TOOLCHAIN_NAME/bin"
fi

# Build PATH with Rust toolchain and Homebrew (if they exist)
BUILD_PATH=""

# Add Rust toolchain to PATH if it exists
if [ -d "$RUST_TOOLCHAIN_PATH" ]; then
    BUILD_PATH="$RUST_TOOLCHAIN_PATH:$BUILD_PATH"
else
    echo "Warning: Rust toolchain path not found: $RUST_TOOLCHAIN_PATH"
    echo "Attempting to use cargo/rustc from current PATH..."
fi

# Add Homebrew bin to PATH if it exists (for macOS)
if [ -d "$HOMEBREW_PREFIX/bin" ]; then
    BUILD_PATH="$BUILD_PATH$HOMEBREW_PREFIX/bin:"
fi

# Export PATH with detected paths
export PATH="$BUILD_PATH$PATH"

# Set Rust toolchain
export RUSTUP_TOOLCHAIN="$RUST_TOOLCHAIN_NAME"

# Try to detect rustc and cargo if not in standard location
if [ ! -f "$RUST_TOOLCHAIN_PATH/rustc" ]; then
    RUSTC_CMD=$(which rustc 2>/dev/null || echo "")
    CARGO_CMD=$(which cargo 2>/dev/null || echo "")
    
    if [ -n "$RUSTC_CMD" ]; then
        export RUSTC="$RUSTC_CMD"
    fi
    if [ -n "$CARGO_CMD" ]; then
        export CARGO="$CARGO_CMD"
    fi
else
    export RUSTC="$RUST_TOOLCHAIN_PATH/rustc"
    export CARGO="$RUST_TOOLCHAIN_PATH/cargo"
fi

# Change to project root (where package.json is located)
# This script runs from src-tauri/gen/apple, so go up 3 levels to project root
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$PROJECT_ROOT"

# Run the Tauri build command with all provided arguments
npm run -- tauri ios xcode-script -v \
    --platform "${PLATFORM_DISPLAY_NAME:?}" \
    --sdk-root "${SDKROOT:?}" \
    --framework-search-paths "${FRAMEWORK_SEARCH_PATHS:?}" \
    --header-search-paths "${HEADER_SEARCH_PATHS:?}" \
    --gcc-preprocessor-definitions "${GCC_PREPROCESSOR_DEFINITIONS:-}" \
    --configuration "${CONFIGURATION:?}" \
    ${FORCE_COLOR} \
    "${ARCHS:?}"
