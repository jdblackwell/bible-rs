# Bible for Omalauncher — based on bible-rs

Community preview for **ARM64 Linux inside Omalauncher's Arch environment**.
This is not an Android APK. Original project: Micky and the bible-rs contributors,
https://github.com/michaelshimeles/bible-rs. Starting fork: ForrestKnight/bible-rs.
Omalauncher integration, adaptation and device testing: **Javahax**.
See the included CREDITS.md and license notices. No upstream endorsement is implied.

## Install a preview package

Use a trusted release archive and check its accompanying SHA-256 file:

```sh
sha256sum -c bible-for-omalauncher-*.tar.gz.sha256
tar -xzf bible-for-omalauncher-<build>-linux-aarch64.tar.gz
cd bible-for-omalauncher-<build>-linux-aarch64
./install.sh
```

The archive must be extracted before installation. `install.sh` also verifies
its internal file checksums. Do not run it with sudo. Existing files that would
be replaced are backed up under the installation prefix, and a manifest records
ownership. Default prefix: `$HOME/.local`; launch `$HOME/.local/bin/bible-rs`
or use the **Bible for Omalauncher** app-drawer entry. Add `$HOME/.local/bin` to
PATH if needed. Install system runtime libraries using the Arch package manager
if missing; see the build prerequisites below. The package is dynamically linked
and requires a compatible Arch ARM64 environment.

The optional widget adds reading/search and a shortcut to the GUI:

```sh
./install.sh --widget
owl-check "${XDG_CONFIG_HOME:-$HOME/.config}/omalauncher/widgets/bible-reader.owl"
```

Widget installation is optional and requires an Omalauncher version supporting
OWL 11. Add or position it using the launcher's widget controls. The installer
does not edit your home layout or theme. The widget file in the archive is a
template; install it with the script so executable paths are filled correctly.
The widget credits the original project in its metadata and reading view.

Custom paths are supported, including spaces:

```sh
./install.sh --prefix "$HOME/.local" --widget --widget-dir "$HOME/.config/omalauncher/widgets"
```

Use absolute paths. Quotes, backslashes, dollar signs and control characters in
installation paths are rejected, since those paths are embedded in shell/OWL
commands. Paths containing ordinary spaces work.

## Upgrade, rollback, uninstall

Exit the GUI normally first so pending edits are saved. Back up your study data
before switching versions. Install the next extracted release with the same
prefix/options. Existing managed files retain their original pre-install backup;
locally modified files get a new backup instead of being discarded.

Uninstall using the installed script:

```sh
bash "$HOME/.local/share/bible-for-omalauncher/uninstall.sh"
```

It removes only files whose checksums still match this installation, restores
files that the installer replaced, and leaves modified files with a message.
Backups remain in `$HOME/.local/share/bible-for-omalauncher/backups/`. To roll
back, uninstall this preview and reinstall the previous verified package, using
the same prefix. A retained original binary may also be restored by uninstall.

Study data is kept under `${XDG_DATA_HOME:-$HOME/.local/share}/bible-rs`
(or `BIBLE_RS_HOME` when set). Close the app before copying that entire folder
for a consistent backup. Uninstall never removes study data, widget state,
backups, or launcher layout. Widget bookmarks and the GUI study store are
currently separate. An old prototype's `Bible` desktop entry may remain;
remove that older entry manually if it creates a duplicate.

## Build from source

Keep the repository's LICENSE, CREDITS.md, LICENSES and font notices.
Use Rust 1.95 or newer and a native ARM64 Arch system with Git, a C toolchain,
pkg-config, Wayland/X11, xkbcommon, OpenGL/EGL and the GUI runtime libraries.
Typical Arch build prerequisites (verify against your installed image):

```sh
sudo pacman -S --needed base-devel git pkgconf wayland libxkbcommon libx11 libglvnd mesa
cargo build --locked --release -p bible-rs --bins
```

Review the package manager's transaction before accepting it. No automatic
package installation is performed by this release.

To assemble a redistribution package, Node.js is additionally used to collect
locked dependency licenses:

```sh
./launcher/scripts/package.sh
```

The source tree must be clean and committed. Archives and checksums are written
to `dist/`. Each archive records the source commit and carries the original MIT
notice, font licenses, translation provenance, CC BY asset credit, and generated
third-party dependency notices. `SKIP_BUILD=1` uses existing release binaries;
use it only after building those binaries from the same source tree. Local
installer tests may use `ALLOW_DIRTY=1`; packages marked that way are not releases.

## Preview limitations

- Tested in one Omalauncher/Arch ARM64 environment; broader device testing and
  the full upstream CI matrix remain outstanding.
- Software graphics can limit scrolling smoothness. See the source tree's
  `launcher/performance/REPORT.md` for measured results and methodology.
- Notes return to their source verse; exact selected-word anchors are not stored.
- Bundled translations work offline. Imported translations/API keys belong to
  the user and are never included in this package. Retain all existing translation
  notices, including the KJV territorial note and WEB trademark notice.
- This preview uses the tested upstream base recorded in SOURCE.txt; newer
  upstream changes have not yet been merged. The Linux preview-release link
  is distinct from upstream's official desktop update channels.
