//! The phone's exposed content strip closes the sidebar without activating content.
use bible_rs::{
    app::{BibleApp, Startup, initialize, online_counter},
    pages::Page,
    theme,
};
use eframe::egui::{pos2, vec2};
use egui_kittest::Harness;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
#[test]
fn phone_tap_outside_sidebar_reveals_selected_section() {
    let dir = std::env::temp_dir().join(format!("sidebar-dismiss-{}", std::process::id()));
    let mut init = initialize(dir, Arc::new(|| {})).unwrap();
    init.settings.onboarded = true;
    init.settings.sidebar_open = true;
    init.settings.reduce_motion = true;
    let startup =
        Startup { t0: Instant::now(), probe: false, init: Duration::ZERO, first_ui: None, frames: 0 };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).build_eframe(|cc| {
        cc.egui_ctx.set_fonts(theme::font_definitions());
        BibleApp::new(cc, init, online_counter(), startup)
    });
    for page in [Page::Notes, Page::Bookmarks, Page::Reader] {
        h.state_mut().settings.sidebar_open = true;
        h.state_mut().pages.open(page);
        h.run_steps(5);
        assert!(h.state().settings.sidebar_open);
        let selection = h.state().reader.selection;
        let at = pos2(350.0, 350.0);
        h.hover_at(at);
        h.step();
        h.drag_at(at);
        h.step();
        h.drop_at(at);
        h.run_steps(5);
        assert!(!h.state().settings.sidebar_open);
        assert_eq!(h.state().pages.page, page);
        assert_eq!(h.state().reader.selection, selection);
    }
}
