#!/bin/bash
# Bouwt "FM27 Training Assistant.app" in ./dist (release, ad-hoc ondertekend).
set -euo pipefail
cd "$(dirname "$0")"
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release -p fm27-trainer
# module voor in FM27: universeel (arm64 + x86_64), want FM27 is universeel
cargo build --release -p fm27-agent --target aarch64-apple-darwin
cargo build --release -p fm27-agent --target x86_64-apple-darwin
mkdir -p target/release
lipo -create -output target/release/libfm27_agent.dylib \
  target/aarch64-apple-darwin/release/libfm27_agent.dylib \
  target/x86_64-apple-darwin/release/libfm27_agent.dylib
codesign --force --sign - target/release/libfm27_agent.dylib
VERSION=$(grep -m1 '^version' app/Cargo.toml | cut -d'"' -f2)
APP="dist/FM27 Training Assistant.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp target/release/fm27-trainer "$APP/Contents/MacOS/FM27 Training Assistant"
cp target/release/libfm27_agent.dylib "$APP/Contents/MacOS/libfm27_agent.dylib"
cp app/assets/icon/AppIcon.icns "$APP/Contents/Resources/AppIcon.icns"
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>FM27 Training Assistant</string>
  <key>CFBundleDisplayName</key><string>FM27 Training Assistant</string>
  <key>CFBundleIdentifier</key><string>app.fm27trainingassistant</string>
  <key>CFBundleExecutable</key><string>FM27 Training Assistant</string>
  <key>CFBundleIconFile</key><string>AppIcon</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleDevelopmentRegion</key><string>en</string>
  <key>CFBundleLocalizations</key><array><string>en</string><string>tr</string><string>nl</string></array>
  <key>LSMinimumSystemVersion</key><string>13.0</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSScreenCaptureUsageDescription</key><string>FM27 Training Assistant reads the Tactical Effectiveness screen of Football Manager.</string>
</dict></plist>
PLIST
codesign --force --deep --sign - "$APP"
echo "Ready: $APP"
