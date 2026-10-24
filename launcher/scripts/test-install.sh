#!/usr/bin/env bash
# Test an extracted release package without touching the actual home layout/data.
set -euo pipefail
package_dir=$(cd -- "${1:?Usage: test-install.sh EXTRACTED_PACKAGE_DIR}" && pwd -P)
test_dir=$(mktemp -d)
trap 'rm -rf -- "$test_dir"' EXIT
prefix="$test_dir/prefix with spaces"
widgets="$test_dir/widgets with spaces"
mkdir -p "$prefix/bin" "$widgets"
printf '%s\n' 'original binary placeholder' > "$prefix/bin/bible-rs"
cp "$prefix/bin/bible-rs" "$test_dir/original-binary"
printf '%s\n' 'original widget placeholder' > "$widgets/bible-reader.owl"
"$package_dir/install.sh" --prefix "$prefix" --widget --widget-dir "$widgets"
if cmp -s "$prefix/bin/bible-rs" "$test_dir/original-binary"; then exit 1; fi
! rg -q '@BIN_DIR@|/root/' "$widgets/bible-reader.owl"
"$prefix/bin/bible-widget" read web 'John 3:16' > "$test_dir/read.json"
rg -q '"verses"' "$test_dir/read.json"
# Upgrade carries original backups forward, rather than restoring the previous package.
"$package_dir/install.sh" --prefix "$prefix" --widget --widget-dir "$widgets"
bash "$prefix/share/bible-for-omalauncher/uninstall.sh"
[[ $(< "$prefix/bin/bible-rs") == 'original binary placeholder' ]]
[[ $(< "$widgets/bible-reader.owl") == 'original widget placeholder' ]]
[[ ! -e $prefix/bin/bible-widget ]]
# A modified user file must survive uninstall.
"$package_dir/install.sh" --prefix "$prefix"
printf '%s\n' 'user modified file' > "$prefix/bin/bible-rs"
bash "$prefix/share/bible-for-omalauncher/uninstall.sh"
[[ $(< "$prefix/bin/bible-rs") == 'user modified file' ]]
# Corrupt package contents must fail checksum verification before installation.
cp -a "$package_dir" "$test_dir/corrupt"
printf '%s\n' 'modified' >> "$test_dir/corrupt/CHANGELOG.md"
if "$test_dir/corrupt/install.sh" --prefix "$test_dir/should-not-install" > "$test_dir/corrupt.log" 2>&1; then
  printf '%s\n' 'ERROR: corrupt package accepted' >&2; exit 1
fi
[[ ! -e $test_dir/should-not-install/bin/bible-rs ]]
printf '%s\n' 'PASS: spaces, bridge, backup, upgrade, restore, modified-file preservation and corrupt-package refusal.'
