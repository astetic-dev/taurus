#!/bin/bash
# Signed, notarised Taurus .dmg for macOS (#266).
#
#   TAURUS_SIGN_ID=<identity hash> tools/macos-release.sh [build|sign] [universal]
#
#   universal  Apple Silicon + Intel in one app (needs `rustup target add
#              x86_64-apple-darwin`). Without it: this Mac's architecture.
#
#   build  release .app, unsigned (takes minutes; no keychain needed)
#   sign   sign + notarise + staple the app, then the .dmg (seconds + Apple's
#          queue). The signing keychain must be unlocked for this step:
#            security unlock-keychain ~/Library/Keychains/taurus-signing.keychain-db
#
# Needs the notarytool profile "taurus-notary"
# (xcrun notarytool store-credentials taurus-notary ...). Output lands in
# src-tauri/target/release/bundle/signed/. The version comes from
# tauri.conf.json; this script never changes it.
set -euo pipefail
cd "$(dirname "$0")/.."
STEP="${1:-sign}"
FLAVOUR="${2:-}"
ROOT="$PWD"
if [ "$FLAVOUR" = universal ]; then
  REL="$ROOT/src-tauri/target/universal-apple-darwin/release"
  ARCH=universal
  TARGET_ARGS=(--target universal-apple-darwin)
else
  REL="$ROOT/src-tauri/target/release"
  ARCH=$(uname -m)
  TARGET_ARGS=()
fi
APP="$REL/bundle/macos/Taurus.app"
OUT="$ROOT/src-tauri/target/release/bundle/signed"
ENT="$ROOT/src-tauri/Entitlements.plist"
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' src-tauri/Cargo.toml | head -1)

if [ "$STEP" = build ]; then
  touch src-tauri/src/lib.rs   # frontend re-embed (PITFALLS.md)
  npx tauri build --bundles app ${TARGET_ARGS[@]+"${TARGET_ARGS[@]}"}
  lipo -archs "$APP/Contents/MacOS/taurus" || true
  echo "built: $APP"
  exit 0
fi

: "${TAURUS_SIGN_ID:?set TAURUS_SIGN_ID to the Developer ID Application identity hash}"
[ -d "$APP" ] || { echo "no $APP; run '$0 build' first" >&2; exit 1; }
mkdir -p "$OUT"

# Both signatures back to back: the signing keychain relocks after a few
# minutes, and Apple's notarisation queue can take longer than that.
echo "== sign app"
codesign --force --options runtime --timestamp --entitlements "$ENT" \
  --sign "$TAURUS_SIGN_ID" "$APP"
codesign --verify --strict --verbose=2 "$APP"

echo "== dmg"
DMG="$OUT/Taurus_${VERSION}_${ARCH}.dmg"
STAGE=$(mktemp -d)
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
rm -f "$DMG"
hdiutil create -volname "Taurus $VERSION" -srcfolder "$STAGE" -ov -format UDZO "$DMG" >/dev/null
rm -rf "$STAGE"
codesign --force --timestamp --sign "$TAURUS_SIGN_ID" "$DMG"

# One notarisation for the .dmg covers the app inside it; the ticket can then
# be stapled to both.
echo "== notarise"
xcrun notarytool submit "$DMG" --keychain-profile taurus-notary --wait
xcrun stapler staple "$DMG"
xcrun stapler staple "$APP"

echo "== check"
spctl --assess --type execute --verbose=2 "$APP"
spctl --assess --type open --context context:primary-signature --verbose=2 "$DMG"
xcrun stapler validate "$DMG"
echo "done: $DMG"
