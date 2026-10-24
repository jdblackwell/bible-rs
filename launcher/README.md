# Bible for Omalauncher — based on bible-rs

Original project: [Micky and the bible-rs contributors](https://github.com/michaelshimeles/bible-rs).
Starting fork: [ForrestKnight/bible-rs](https://github.com/ForrestKnight/bible-rs).
Omalauncher adaptation and testing: **Javahax**. See [full credits](../CREDITS.md).

This directory contains the optional OWL 11 widget, portable package scripts,
and documentation for the ARM64 Arch application running inside Omalauncher.
The Rust bridge in `crates/app/src/bin/bible-widget.rs` uses the original core;
it provides JSON reading/search results for the widget. GUI and bridge data
share the normal bible-rs data directory, but widget bookmarks are stored in
launcher state and are separate from GUI bookmarks.

- [Installation, building, backup and uninstall](INSTALL.md)
- [Preview changes](CHANGELOG.md)
- [Measured performance and remaining limits](performance/REPORT.md)

The widget and desktop entry in this source directory are templates. Use the
installer rather than copying them directly. Installation never edits the
home layout, automatically replaces the user's theme, or includes personal
study data. The GUI can be installed without the widget.

## Validation

Targeted integration tests cover small-window onboarding, reader swipes,
touch selection, settings, note saving/navigation/deletion, bookmarks, and
sidebar dismissal. `crates/app/examples/phone_perf.rs` reproduces UI work
measurements. See the release review/test report for which checks actually ran;
a complete upstream CI matrix and second-device test remain outstanding.

After changing the widget, install a rendered copy to the launcher's widgets
directory, run `owl-check` on it, and inspect the generated tile/screen images.
Use synthetic public-domain passages and test notes in any public screenshots.
