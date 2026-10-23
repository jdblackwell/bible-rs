# After: one-click update on Windows (PR #66)

## Tested artifact

- Commit: `16843bd44c0bcb61a248b50cbfbacc3d43f5c4ff`
- Branch: `agent/oneclick-update-7b3e2-evidence`
- Environment: GitHub windows-latest runner, 1024x768, Direct3D 12 on WARP
- Recorder: windows / gdigrab
- Recording: `evidence.mp4` (56.07s)

## Result

- Passed: 10
- Failed: 0
- Untested: 1

## Timeline

- `000.61s` **SETUP** — Bible 0.2.0 installed; the local server offers 0.3.0 as latest
- `000.70s` **PASSED** — Precondition: the installed app is 0.2.0
- `000.79s` **TEST_START** — It should offer 0.3.0 in the app when Bible starts
- `001.60s` **PASSED** — The daily check asked for the latest release
- `006.25s` **TEST_START** — It should download, install and reopen 0.3.0 after Update
- `006.43s` **PASSED** — Update started the download inside the app
- `031.37s` **PASSED** — Bible 0.2.0 quit so the installer could replace it
- `033.88s` **PASSED** — The installer opened Bible again
- `034.08s` **PASSED** — Installed version is now 0.3.0
- `034.18s` **PASSED** — The reopened app runs from the install folder
- `034.27s` **PASSED** — No web browser opened
- `042.92s` **TEST_START** — It should say 0.3.0 is the newest version
- `044.27s` **PASSED** — Help > Check for Updates asked again
- `048.42s` **TEST_START** — It should explain a failed check in the app, with Retry
- `056.22s` **UNTESTED** — Error text and Retry: checked by eye in after-07-offline.png
- `056.30s` **PASSED** — Still no web browser opened

## Caveats

- None recorded. Add manual caveats before publishing if needed.
