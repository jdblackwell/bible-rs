//! Select a verse range or words without keyboard modifiers on a phone.
use bible_core::{BookId, ChapterIdx, VerseRef};
use bible_rs::{
    app::{BibleApp, Startup, initialize, online_counter},
    reader::TextPos,
    theme,
};
use eframe::egui::{self, pos2, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn tap(h: &mut Harness<'_, BibleApp>, pos: egui::Pos2) {
    h.hover_at(pos);
    h.step();
    h.drag_at(pos);
    h.step();
    h.drop_at(pos);
    h.run_steps(4);
}

#[test]
fn phone_selects_verses_and_words_without_shift() {
    let dir = std::env::temp_dir().join(format!("bible-touch-selection-{}", std::process::id()));
    let mut init = initialize(dir, Arc::new(|| {})).unwrap();
    init.settings.onboarded = true;
    init.settings.sidebar_open = false;
    init.settings.translation = "kjv".into();
    let chapter = ChapterIdx::new(BookId(42), 3).unwrap();
    init.settings.chapter = chapter.0;
    init.settings.verse = 16;
    let startup =
        Startup { t0: Instant::now(), probe: false, init: Duration::ZERO, first_ui: None, frames: 0 };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).wgpu().build_eframe(|cc| {
        cc.egui_ctx.set_fonts(theme::font_definitions());
        BibleApp::new(cc, init, online_counter(), startup)
    });
    h.run_steps(6);
    let verse = |n| VerseRef::new(BookId(42), 3, n);
    let point = |h: &Harness<'_, BibleApp>, n, offset| {
        h.state().reader.point_at(TextPos { verse: verse(n), offset, column: 0 }).unwrap()
    };
    let first = point(&h, 16, 5);
    tap(&mut h, first);
    assert_eq!(h.state().reader.selection, Some((verse(16), verse(16))));
    h.get_by_label("Verses").click();
    h.run_steps(4);
    let end = point(&h, 17, 5);
    tap(&mut h, end);
    assert_eq!(h.state().reader.selection, Some((verse(16), verse(17))));
    if let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW") {
        h.render().unwrap().save(format!("{dir}/touch-selection.png")).unwrap();
    }
    h.get_by_label("Actions").click();
    h.run_steps(4);
    let words = h
        .output()
        .shapes
        .iter()
        .find_map(|shape| {
            if let egui::Shape::Text(text) = &shape.shape
                && text.galley.job.text == "Select Words"
            {
                Some(text.galley.rect.translate(text.pos.to_vec2()).center())
            } else {
                None
            }
        })
        .expect("Select Words is available in the verse menu");
    tap(&mut h, words);
    let from = point(&h, 16, 4);
    let to = point(&h, 16, 20);
    let before = h.state().reader.anchor;
    h.hover_at(from);
    h.step();
    h.drag_at(from);
    h.step();
    for n in 1..=10 {
        h.hover_at(from.lerp(to, n as f32 / 10.0));
        h.step();
    }
    h.drop_at(to);
    h.run_steps(4);
    assert!(h.state().reader.text_span().is_some());
    assert_eq!(h.state().reader.anchor, before);
    h.get_by_label("Done").click();
    h.run_steps(4);
    assert!(h.state().reader.selection.is_none());
    h.hover_at(pos2(180.0, 550.0));
    h.step();
    h.drag_at(pos2(180.0, 550.0));
    h.step();
    h.hover_at(pos2(180.0, 250.0));
    h.step();
    h.drop_at(pos2(180.0, 250.0));
    h.run_steps(4);
    assert!(
        h.state().reader.anchor.offset > before.offset || h.state().reader.anchor.chapter > before.chapter
    );
}
