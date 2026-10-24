# Proposed upstream introduction — draft, not sent

Thank you for creating bible-rs. I've adapted the Rust app for Omalauncher's
Arch environment on Android, beginning with ForrestKnight's fork. The adaptation
retains your licensing, original-project links, and complete contributor history.

Would you welcome focused pull requests for the general improvements? They cover
small-window setup/settings/collection layouts, mouse-style touch scrolling and
selection, and batched note saves with flush-on-navigation/normal-exit checks.
A phone sidebar change also removes repeated chapter layouts while closing; its
scope and desktop behavior would benefit from your review.

I can keep the optional OWL widget, bridge, and preview packaging separate.
Device/harness reproduction steps and before/after measurements are available.
I have kept the current preview pinned to its tested base instead of mixing in
newer upstream update/release work without retesting it.

## Candidate patch groups for review

1. **Phone input and layouts:** fix clipped controls and keyboard-modifier-only
   selection on a phone. Include the phone-width/touch fallback rationale and
   desktop selection/layout regression results.
2. **Note persistence and responsiveness:** coalesce typing writes, flush on
   actions/exit, and avoid hidden repeated layout during phone sidebar closure.
   Include save-lifecycle checks and the controlled profiling results.
3. **Launcher integration, if desired:** optional JSON bridge and OWL template,
   kept outside the standard desktop release flow.

No issue, message, or pull request has been submitted. Before submission, replay
and test each proposed patch group on the maintainer's current branch; the local
preview base is 67aef578cc9d6065b588b48d789be3d4d2c68685.
