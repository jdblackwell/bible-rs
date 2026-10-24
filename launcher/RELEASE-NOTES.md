# Bible for Omalauncher — based on bible-rs

## Preview 1 — draft for review

A community adaptation of [bible-rs by Micky and contributors](https://github.com/michaelshimeles/bible-rs),
starting from [ForrestKnight's fork](https://github.com/ForrestKnight/bible-rs).
Omalauncher adaptation and testing: **Javahax**. The original MIT notice and
contributor history are preserved; the release carries font, dependency,
translation, and CC BY asset notices. No upstream endorsement is implied.

This ARM64 Linux build runs inside Omalauncher's Arch environment. It is not a
standalone Android APK. The package contains the GUI and an optional OWL 11
widget with offline reading/search and a GUI shortcut.

Phone improvements include reachable setup controls, swipe scrolling, verse
and word selection without Shift, settings in the same window, phone layouts
for notes/bookmarks, note previews and navigation, visible delete/remove controls,
and tapping outside the sidebar to reveal the selected section.

The sidebar avoids repeated chapter typesetting while closing. In the recorded
eight-toggle test, extra layouts fell from 37 to zero and 95th-percentile UI work
from 12.6 ms to 1.9 ms. Note edits are batched and flushed before navigation and
normal exit. Frame pacing still depends on software graphics and device load.

Install using the archive's README and install.sh; the widget is opt-in.
Verify the SHA-256 checksum first. The installer backs up files it replaces,
and the uninstaller preserves study data and modified files. See
[installation and rollback instructions](INSTALL.md).

The package records its source commit in SOURCE.txt. The adaptation uses the
tested original base `67aef578cc9d6065b588b48d789be3d4d2c68685`; newer upstream
updates have not yet been merged.

## Validation and limitations

- Native release build and startup smoke check; targeted phone behavior tests.
- Installer checks for spaces, backup/restore, upgrades, modified-file retention,
  and corrupt-package refusal; rendered widget checked with owl-check.
- One Omalauncher/Arch ARM64 environment tested. A second-device test and the full
  upstream CI matrix remain outstanding. This is a preview candidate.
- Exact selected-word anchors are not persisted with notes; links return to the verse.
- Widget and GUI bookmarks remain separate. Imported text and API credentials
  are never part of the package.

Report adaptation issues to the community fork when its review release opens.
Use the original project link above to discover and support upstream.
