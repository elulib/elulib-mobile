# elulib-mobile

[![Test](https://github.com/elulib/elulib-mobile/actions/workflows/test.yaml/badge.svg)](https://github.com/elulib/elulib-mobile/actions/workflows/test.yaml)

## Installation

Clone repository and install dependencies:

```bash
git clone https://github.com/elulib/elulib-mobile.git
cd elulib-mobile

# Install Node.js dependencies
npm install

# Install Rust toolchain and targets (if not already installed)
rustup default stable
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

> [!NOTE]
> You might need to set these environment variables to build and run the app:
> ```bash
> export RUST_TOOLCHAIN_NAME="stable-aarch64-apple-darwin"
> export HOMEBREW_PREFIX="/opt/homebrew"
> export RUSTUP_HOME="$HOME/.rustup"
> ```

### iOS Setup

Require Xcode 15.4 or higher.

```bash
# Install CocoaPods
brew install cocoapods

# Accept Xcode license
sudo xcodebuild -license accept

# Initialize iOS project
npx tauri ios init
```

Run the development app:
```bash
npm run dev:ios
```

### Android Setup

>[!IMPORTANT]
> Android build is not supported yet. This section is placeholder for future support.

Require Android Studio 2024.3 or higher.

```bash
# Install Android Studio
brew install android-studio

# Open Android Studio
android-studio
```

Run the development app:
```bash
npm run dev:android
```

## Testing

Run all tests:
```bash
npm test
```

Run individual tests:
```bash
# Run all tests
npm run test:rust
# Run unit tests
npm run test:rust:unit
# Run integration tests
npm run test:rust:integration
```

## Building

- Build the iOS app:
    ```bash
    npm run build:ios
    ```

- Build the Android app:
    ```bash
    npm run build:android
    ```
