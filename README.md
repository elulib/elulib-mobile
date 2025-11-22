# elulib-mobile

## Installation

### Local

Clone repository and install dependencies:

```bash
git clone https://github.com/elulib/elulib-mobile.git
cd elulib-mobile

# Install Node.js dependencies
npm install

# Install Rust toolchain (if not already installed)
rustup default stable

# Add required Rust targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

#### iOS Setup

Require Xcode 17.0 or higher.

```bash
# Install CocoaPods
sudo gem install cocoapods
# or
brew install cocoapods

# Accept Xcode license
sudo xcodebuild -license accept

# Initialize iOS project
npx tauri ios init
```

Run the app:
```bash
npm run dev:ios
```

Generate icons:
```bash
chmod +x src-tauri/icons/generate-ios-icons.sh
```

### Building

#### iOS

```bash
npm run dev:ios
```
