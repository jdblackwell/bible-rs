# Phone performance investigation — 2026-10-24

## Changes installed

- Phone sidebar transitions now complete immediately. The previous animated
  width changes repeatedly re-typeset the chapter while closing the sidebar.
  Desktop transitions and the existing sidebar controls remain available.
- Phone note editing batches saves after 350 ms without an edit. Navigation,
  other app actions, Back to notes, source-verse links, and normal application
  exit flush pending edits. Note deletion clears the editor first, so a pending
  save cannot recreate it. Closing a note popover normally also saves on exit.

## Measurements

Native release builds, in this Omalauncher/Wayland session, using isolated
study data, KJV and Psalm 119. Eight sidebar toggles per build. The same
360 × 700 size request and demo steps were used. Builds were finished before
these final runs; no compiler was competing for CPU.

| Sidebar metric | Before | After |
| --- | ---: | ---: |
| Extra chapter layouts during eight toggles | 37 | 0 |
| UI work, mean | 4.103 ms | 0.615 ms |
| UI work, 95th percentile | 12.607 ms | 1.904 ms |
| Frame interval, median | 39.662 ms | 26.375 ms |

The chapter-layout count is a direct, repeatable reduction in work. Frame
intervals also include the renderer, compositor and scheduling, and vary with
the phone's load and thermal state; these samples are not a guarantee of an
app-wide frame-rate increase.

The 900-frame scrolling probes measured 95th-percentile UI work of 0.467 ms
before and 0.382 ms after. Afterward, frame intervals were 32.100 ms at the
median and 36.520 ms at the 95th percentile. The changes do not alter normal
scrolling layout; the difference between these runs should not be attributed
to a new scrolling algorithm. The reported chapter delta includes the initial
demo jump from the default location to Psalm 119.

In the repeatable phone UI harness, 40 text events previously generated 40
study mutations during typing. The new build generates zero during the burst
and one after the pause. Navigation and normal exit were separately verified
to persist edits before the timer fires. UI-only harness timings exclude GPU
rendering; native timings above include its effects on frame pacing.

## Remaining limitation

This app inherits `LIBGL_ALWAYS_SOFTWARE=1`. The graphics device nodes are
inaccessible from this environment. In an exploratory 900-frame comparison,
the alternate wgpu renderer was slower (33.112 ms median frame interval versus
25.406 ms with OpenGL); the default OpenGL renderer was retained. Those
exploratory runs were not the final matched before/after comparison.

The large gap between UI work and native frame intervals points to the
rendering/presentation path as the remaining pacing limit. The running app's
CPU counters did not advance during a two-second idle sample: it is not
continuously redrawing at rest. No system graphics settings were changed.
Low-level perf/strace commands were unavailable; the measurements use the
app's built-in frame probes, egui harness, study-generation counters and procfs.

## Verification and reproduction

- `note_save_lifecycle.rs` passed: 40 edits coalesce into one save after a
  pause; navigating away and normal exit preserve edits without waiting.
- The phone performance harness asserts that typing survives returning to
  the note list. The native sidebar probes completed eight toggles each.
- Release build, formatting, and `git diff --check` passed.
- Measurements use temporary study databases and do not change the user's
  notes or bookmarks.

UI probe: `BIBLE_RS_OFFSCREEN=1 BIBLE_RS_SHOT_FILMSTRIP=1 cargo run --release
-p bible-rs --example phone_perf` (compile before timing).

Native probes: use `BIBLE_RS_HOME=<isolated data directory>`,
`BIBLE_RS_BACKGROUND=1`, `BIBLE_RS_DEMO='size:360x700;page:reader;jump:ps 119:1'`,
and either `BIBLE_RS_SIDEBAR_PROBE=8` or `BIBLE_RS_SCROLL_PROBE=1` with the
release executable. Set the isolated app settings to onboarded, KJV, and
sidebar closed before each run.

Raw final results: `sidebar-{before,after}-final.json`,
`scroll-{before,after}-final.json`, and `ui-{before,after}-quiet.jsonl`.
