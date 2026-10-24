#!/usr/bin/env bash
# MIT. Bible for Omalauncher, based on bible-rs. See CREDITS.md and LICENSE.
set -euo pipefail
shopt -u patsub_replacement
bundle_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
install_prefix="$HOME/.local"
install_widget=false
widget_dir="${XDG_CONFIG_HOME:-$HOME/.config}/omalauncher/widgets"
while (($#)); do
  case "$1" in
    --prefix) install_prefix=${2:?--prefix requires a path}; shift 2 ;;
    --widget) install_widget=true; shift ;;
    --widget-dir) widget_dir=${2:?--widget-dir requires a path}; shift 2 ;;
    -h|--help) printf '%s\n' 'Usage: ./install.sh [--prefix PATH] [--widget] [--widget-dir PATH]'; exit 0 ;;
    *) printf 'Unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ $(uname -m) == aarch64 && $(uname -s) == Linux ]] || { printf '%s\n' 'This package requires ARM64 Linux inside Omalauncher.' >&2; exit 1; }
for install_path in "$install_prefix" "$widget_dir"; do
  [[ $install_path == /* && $install_path != *[\'\"\`\$\\$'\n\r\t']* ]] || { printf '%s\n' 'Use absolute installation paths without quotes, backslashes, dollar signs, or control characters. Spaces are supported.' >&2; exit 2; }
done
(cd -- "$bundle_dir" && sha256sum --quiet -c SHA256SUMS)
[[ -x $bundle_dir/bin/bible-rs && -x $bundle_dir/bin/bible-widget ]] || { printf '%s\n' 'Incomplete release package.' >&2; exit 1; }
mkdir -p -- "$install_prefix"
install_prefix=$(cd -- "$install_prefix" && pwd -P)
state_dir="$install_prefix/share/bible-for-omalauncher"
mkdir -p -- "$state_dir/backups"
backup_dir=$(mktemp -d "$state_dir/backups/install-XXXXXXXX")
manifest="$state_dir/installed.tsv"
new_manifest="$backup_dir/installed.tsv"
declare -A prior_hash prior_backup
if [[ -f $manifest ]]; then
  cp -- "$manifest" "$backup_dir/previous-manifest.tsv"
  while IFS=$'\t' read -r digest destination original; do
    prior_hash["$destination"]=$digest; prior_backup["$destination"]=$original
  done < "$manifest"
fi
install_file() {
  local source=$1 destination=$2 mode=$3 original=- digest
  mkdir -p -- "$(dirname -- "$destination")"
  if [[ -e $destination || -L $destination ]]; then
    [[ -f $destination && ! -L $destination ]] || { printf 'Refusing to replace non-regular file: %s\n' "$destination" >&2; exit 1; }
    digest=$(sha256sum -- "$destination"); digest=${digest%% *}
    if [[ ${prior_hash[$destination]-} == "$digest" ]]; then
      original=${prior_backup[$destination]}
    else
      original="$backup_dir/file-$(wc -l < "$new_manifest" 2>/dev/null || printf 0)"
      cp -p -- "$destination" "$original"
    fi
  fi
  install -m "$mode" -- "$source" "$destination.new"
  mv -f -- "$destination.new" "$destination"
  digest=$(sha256sum -- "$destination"); digest=${digest%% *}
  printf '%s\t%s\t%s\n' "$digest" "$destination" "$original" >> "$new_manifest"
}
: > "$new_manifest"
install_file "$bundle_dir/bin/bible-rs" "$install_prefix/bin/bible-rs" 755
install_file "$bundle_dir/bin/bible-widget" "$install_prefix/bin/bible-widget" 755
install_file "$bundle_dir/share/bible-rs.png" "$install_prefix/share/icons/hicolor/256x256/apps/bible-for-omalauncher.png" 644
cat > "$backup_dir/bible-for-omalauncher.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Bible for Omalauncher
Comment=Based on bible-rs by Micky and contributors; adaptation by Javahax
Exec="$install_prefix/bin/bible-rs"
Icon=$install_prefix/share/icons/hicolor/256x256/apps/bible-for-omalauncher.png
Terminal=false
Categories=Education;
StartupWMClass=bible-rs
DESKTOP
install_file "$backup_dir/bible-for-omalauncher.desktop" "$install_prefix/share/applications/bible-for-omalauncher.desktop" 644
for license_file in "$bundle_dir"/share/licenses/*; do
  install_file "$license_file" "$state_dir/licenses/$(basename -- "$license_file")" 644
done
install_file "$bundle_dir/uninstall.sh" "$state_dir/uninstall.sh" 755
if $install_widget; then
  template=$(< "$bundle_dir/widget/bible-reader.owl")
  printf '%s\n' "${template//@BIN_DIR@/$install_prefix/bin}" > "$backup_dir/bible-reader.owl"
  install_file "$backup_dir/bible-reader.owl" "$widget_dir/bible-reader.owl" 644
elif [[ -f $manifest ]]; then
  # Preserve ownership of an optional widget from a previous installation.
  while IFS=$'\t' read -r digest destination original; do
    [[ $destination == *.owl ]] && printf '%s\t%s\t%s\n' "$digest" "$destination" "$original" >> "$new_manifest"
  done < "$manifest"
fi
cp -- "$new_manifest" "$manifest"
printf 'Installed Bible for Omalauncher into %s\nBackups: %s\nUninstall: bash "%s/uninstall.sh"\n' "$install_prefix" "$backup_dir" "$state_dir"
printf '%s\n' 'Study data and launcher home layout were not modified. Add the optional widget to your home layout as desired.'
