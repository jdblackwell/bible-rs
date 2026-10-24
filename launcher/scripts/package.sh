#!/usr/bin/env bash
# Build a source-linked ARM64 community preview, including all redistribution notices.
set -euo pipefail
repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd -P)
cd -- "$repo_dir"
[[ $(uname -m) == aarch64 ]] || { printf '%s\n' 'Build this package on ARM64 Linux.' >&2; exit 1; }
if [[ ${ALLOW_DIRTY:-0} != 1 ]]; then
  git diff --quiet && git diff --cached --quiet || { printf '%s\n' 'Commit the reviewed release sources first (ALLOW_DIRTY=1 creates a local test package only).' >&2; exit 1; }
fi
if [[ ${SKIP_BUILD:-0} != 1 ]]; then cargo build --locked --release -p bible-rs --bins -j 2; fi
build=$(sed -n 's/^pub const BUILD: &str = "\([^"]*\)";.*/\1/p' crates/app/src/adaptation.rs)
package="bible-for-omalauncher-$build-linux-aarch64"
mkdir -p dist
stage=$(mktemp -d "$repo_dir/dist/stage-XXXXXXXX")
trap 'rm -rf -- "$stage"' EXIT
payload="$stage/$package"
mkdir -p "$payload/bin" "$payload/widget" "$payload/share/licenses"
mkdir -p "$payload/LICENSES" "$payload/objects" "$payload/crates/app/assets/fonts"
install -m755 target/release/bible-rs target/release/bible-widget "$payload/bin/"
install -m755 launcher/install.sh launcher/uninstall.sh "$payload/"
install -m644 launcher/bible-reader.owl "$payload/widget/"
install -m644 crates/app/assets/icon-256.png "$payload/share/bible-rs.png"
install -m644 launcher/INSTALL.md "$payload/README.md"
install -m644 launcher/CHANGELOG.md "$payload/CHANGELOG.md"
install -m644 LICENSE CREDITS.md "$payload/"
install -m644 LICENSES/README.md LICENSES/bibles.md LICENSES/CC-BY-4.0.txt "$payload/LICENSES/"
install -m644 objects/LICENSE "$payload/objects/"
install -m644 crates/app/assets/fonts/OFL-Inter.txt crates/app/assets/fonts/OFL-Literata.txt "$payload/crates/app/assets/fonts/"
install -m644 LICENSE CREDITS.md "$payload/share/licenses/"
install -m644 LICENSES/README.md "$payload/share/licenses/PROVENANCE.md"
install -m644 LICENSES/bibles.md LICENSES/CC-BY-4.0.txt "$payload/share/licenses/"
install -m644 objects/LICENSE "$payload/share/licenses/ARK-LICENSE"
install -m644 crates/app/assets/fonts/OFL-Inter.txt crates/app/assets/fonts/OFL-Literata.txt "$payload/share/licenses/"
cargo metadata --locked --format-version 1 --filter-platform aarch64-unknown-linux-gnu > "$stage/metadata.json"
node launcher/scripts/licenses.mjs "$stage/metadata.json" "$payload/share/licenses/THIRD-PARTY-NOTICES.md"
{
  printf 'Adaptation build: %s\nSource commit: ' "$build"
  git rev-parse HEAD
  printf 'Original base: 67aef578cc9d6065b588b48d789be3d4d2c68685\n'
  printf 'Original project: https://github.com/michaelshimeles/bible-rs\nStarting fork: https://github.com/ForrestKnight/bible-rs\n'
  if [[ ${ALLOW_DIRTY:-0} == 1 ]]; then printf 'LOCAL TEST PACKAGE: working tree may contain uncommitted changes.\n'; fi
} > "$payload/SOURCE.txt"
(cd -- "$payload" && find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS)
tar -C "$stage" -czf "dist/$package.tar.gz" "$package"
(cd dist && sha256sum "$package.tar.gz" > "$package.tar.gz.sha256")
printf 'Prepared %s/dist/%s.tar.gz\n' "$repo_dir" "$package"
