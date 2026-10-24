//! Phone swipes scroll the actual reader even when Wayland sends mouse events.
use std::sync::Arc;
use std::time::{Duration, Instant};

use bible_rs::app::{BibleApp, Startup, initialize, online_counter};
use bible_rs::theme;
use eframe::egui::{self, pos2, vec2};
use egui_kittest::Harness;

fn swipe(h: &mut Harness<'_, BibleApp>, from: egui::Pos2, to: egui::Pos2) {
    h.hover_at(from);
    h.step();
    h.drag_at(from);
    h.step();
    for n in 1..=10 {
        h.hover_at(from.lerp(to, n as f32 / 10.0));
        h.step();
    }
    h.drop_at(to);
    h.run_steps(3);
}

#[test]
fn pointer_swipes_scroll_reading_without_selecting_words() {
    for size in [vec2(360.0, 700.0), vec2(700.0, 360.0)] {
        let dir = std::env::temp_dir().join(format!("bible-reader-swipe-{}", std::process::id()));
        let mut init = initialize(dir, Arc::new(|| {})).unwrap();
        init.settings.onboarded = true;
        init.settings.sidebar_open = false;
        let startup =
            Startup { t0: Instant::now(), probe: false, init: Duration::ZERO, first_ui: None, frames: 0 };
        let mut h = Harness::builder().with_size(size).build_eframe(|cc| {
            cc.egui_ctx.set_fonts(theme::font_definitions());
            BibleApp::new(cc, init, online_counter(), startup)
        });
        h.run_steps(6);
        let before = h.state().reader.anchor;
        swipe(&mut h, pos2(size.x / 2.0, size.y - 70.0), pos2(size.x / 2.0, 140.0));
        let after = h.state().reader.anchor;
        assert!(after.chapter > before.chapter || after.offset > before.offset + 50.0);
        assert!(h.state().reader.text_span().is_none(), "swiping must not select words");
        swipe(&mut h, pos2(size.x / 2.0, 140.0), pos2(size.x / 2.0, size.y - 70.0));
        let returned = h.state().reader.anchor;
        assert!(returned.chapter < after.chapter || returned.offset < after.offset - 50.0);
    }
}
