#!/bin/bash
# Release voor macOS: universele app (Apple silicon + Intel), ondertekend, genotariseerd, in een .dmg.
#
#   ./release.sh                 bouwt dist/FM27-Training-Assistant-macOS.dmg
#
# Ondertekenen gebeurt met het eerste "Developer ID Application"-certificaat in je sleutelhanger
# (of FM27_SIGN_IDENTITY). Zonder certificaat: ad-hoc.
# Notariseren gebeurt als er een notarytool-profiel is (FM27_NOTARY_PROFILE, standaard "fm27-notary").
set -euo pipefail
cd "$(dirname "$0")"
export PATH="$HOME/.cargo/bin:$PATH"

VERSION=$(grep -m1 '^version' app/Cargo.toml | cut -d'"' -f2)
IDENTITY="${FM27_SIGN_IDENTITY:-$(security find-identity -v -p codesigning 2>/dev/null | grep -m1 'Developer ID Application' | sed -E 's/.*"(.*)"/\1/' || true)}"
PROFILE="${FM27_NOTARY_PROFILE:-fm27-notary}"
# Bouwen en ondertekenen buiten iCloud/File Provider-mappen
WORK="${TMPDIR:-/tmp}fm27-release"
APP="$WORK/FM27 Training Assistant.app"
mkdir -p dist
DMG="$PWD/dist/FM27-Training-Assistant-macOS.dmg"

echo "▸ FM27 Training Assistant $VERSION"
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME/.cargo=/cargo --remap-path-prefix=$PWD=/src --remap-path-prefix=$HOME=/home"
for T in aarch64-apple-darwin x86_64-apple-darwin; do
  cargo build --release -p fm27-trainer -p fm27-agent --target "$T"
done
mkdir -p target/universal
lipo -create -output target/universal/fm27-trainer \
  target/aarch64-apple-darwin/release/fm27-trainer target/x86_64-apple-darwin/release/fm27-trainer
lipo -create -output target/universal/libfm27_agent.dylib \
  target/aarch64-apple-darwin/release/libfm27_agent.dylib target/x86_64-apple-darwin/release/libfm27_agent.dylib

echo "▸ app bundle"
rm -rf "$WORK"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp target/universal/fm27-trainer "$APP/Contents/MacOS/FM27 Training Assistant"
cp target/universal/libfm27_agent.dylib "$APP/Contents/MacOS/libfm27_agent.dylib"

ICONSET="$WORK/AppIcon.iconset"
mkdir -p "$ICONSET"
if command -v sips >/dev/null 2>&1 && command -v iconutil >/dev/null 2>&1 && [ -f app/assets/icon/icon_1024.png ]; then
  sips -z 16 16     app/assets/icon/icon_1024.png --out "$ICONSET/icon_16x16.png" >/dev/null
  sips -z 32 32     app/assets/icon/icon_1024.png --out "$ICONSET/icon_16x16@2x.png" >/dev/null
  sips -z 32 32     app/assets/icon/icon_1024.png --out "$ICONSET/icon_32x32.png" >/dev/null
  sips -z 64 64     app/assets/icon/icon_1024.png --out "$ICONSET/icon_32x32@2x.png" >/dev/null
  sips -z 128 128   app/assets/icon/icon_1024.png --out "$ICONSET/icon_128x128.png" >/dev/null
  sips -z 256 256   app/assets/icon/icon_1024.png --out "$ICONSET/icon_128x128@2x.png" >/dev/null
  sips -z 256 256   app/assets/icon/icon_1024.png --out "$ICONSET/icon_256x256.png" >/dev/null
  sips -z 512 512   app/assets/icon/icon_1024.png --out "$ICONSET/icon_256x256@2x.png" >/dev/null
  sips -z 512 512   app/assets/icon/icon_1024.png --out "$ICONSET/icon_512x512.png" >/dev/null
  cp app/assets/icon/icon_1024.png "$ICONSET/icon_512x512@2x.png"
  iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns" || cp app/assets/icon/AppIcon.icns "$APP/Contents/Resources/AppIcon.icns"
else
  cp app/assets/icon/AppIcon.icns "$APP/Contents/Resources/AppIcon.icns"
fi
rm -rf "$ICONSET"

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
  <key>CFBundleLocalizations</key><array><string>en</string><string>tr</string><string>de</string><string>nl</string><string>fr</string><string>it</string><string>es</string></array>
  <key>LSApplicationCategoryType</key><string>public.app-category.sports</string>
  <key>LSMinimumSystemVersion</key><string>13.0</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSScreenCaptureUsageDescription</key><string>FM27 Training Assistant reads the Tactical Effectiveness screen of Football Manager.</string>
</dict></plist>
PLIST

echo "▸ signing"
xattr -cr "$APP"
if [ -n "$IDENTITY" ]; then
  echo "  with: $IDENTITY"
  codesign --force --timestamp --options runtime --sign "$IDENTITY" "$APP/Contents/MacOS/libfm27_agent.dylib"
  codesign --force --timestamp --options runtime --sign "$IDENTITY" "$APP"
else
  echo "  no Developer ID certificate found: using ad-hoc signature"
  codesign --force --sign - "$APP/Contents/MacOS/libfm27_agent.dylib"
  codesign --force --sign - "$APP"
fi
codesign --verify --deep --strict "$APP"

echo "▸ dmg"
STAGE="$WORK/dmg"
rm -rf "$STAGE" "$DMG"
mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
hdiutil create -quiet -volname "FM27 Training Assistant" -srcfolder "$STAGE" -ov -format UDZO "$DMG"
rm -rf "$STAGE"

if [ -n "$IDENTITY" ]; then
  codesign --force --timestamp --sign "$IDENTITY" "$DMG"
  if xcrun notarytool history --keychain-profile "$PROFILE" >/dev/null 2>&1; then
    echo "▸ notarizing"
    xcrun notarytool submit "$DMG" --keychain-profile "$PROFILE" --wait
    xcrun stapler staple "$DMG"
    spctl -a -t open --context context:primary-signature -v "$DMG" || true
  else
    echo "  no notarytool profile '$PROFILE': skipping notarization"
  fi
fi
rm -rf "$WORK"
echo "✓ $DMG"
