#!/usr/bin/env bash
# MIT. Restore replaced files; leave modified files and all study data intact.
set -euo pipefail
state_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
manifest="$state_dir/installed.tsv"
[[ -f $manifest ]] || { printf '%s\n' 'No installed-file manifest found. Run the installed copy of this script.' >&2; exit 1; }
remaining=$(mktemp "$state_dir/remaining-XXXXXXXX")
while IFS=$'\t' read -r expected destination original; do
  if [[ ! -e $destination && ! -L $destination ]]; then continue; fi
  if [[ ! -f $destination || -L $destination ]]; then
    printf 'Kept changed file: %s\n' "$destination"
    printf '%s\t%s\t%s\n' "$expected" "$destination" "$original" >> "$remaining"
    continue
  fi
  actual=$(sha256sum -- "$destination"); actual=${actual%% *}
  if [[ $actual != "$expected" ]]; then
    printf 'Kept modified file: %s\n' "$destination"
    printf '%s\t%s\t%s\n' "$expected" "$destination" "$original" >> "$remaining"
    continue
  fi
  if [[ $original != - && -f $original ]]; then cp -p -- "$original" "$destination"; else rm -- "$destination"; fi
done < "$manifest"
if [[ -s $remaining ]]; then mv -- "$remaining" "$manifest"; else rm -- "$remaining" "$manifest"; fi
printf '%s\n' 'Uninstall complete. Study data, settings, home layout, and installation backups were retained.'
