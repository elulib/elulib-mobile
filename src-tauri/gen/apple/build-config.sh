#!/bin/bash
# Build configuration constants for iOS Rust build
# Override these values via environment variables if needed

# Rust toolchain configuration
export RUST_TOOLCHAIN_NAME="${RUST_TOOLCHAIN_NAME:-stable-aarch64-apple-darwin}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"

# Homebrew configuration (for macOS)
export HOMEBREW_PREFIX="${HOMEBREW_PREFIX:-/opt/homebrew}"

# Computed paths (don't override these)
export RUST_TOOLCHAIN_PATH="$RUSTUP_HOME/toolchains/$RUST_TOOLCHAIN_NAME/bin"
