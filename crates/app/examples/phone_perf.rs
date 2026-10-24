//! Repeatable phone UI CPU probe. Run with BIBLE_RS_OFFSCREEN=1 and
//! BIBLE_RS_SHOT_FILMSTRIP=1 to retain animations. Rendering is measured separately
//! with the native sidebar/scroll probes; these samples include egui and harness work.
use bible_core::{BookId, ChapterIdx, VerseRef};
use bible_rs::{
    app::{BibleApp, Startup, initialize, online_counter},
    pages::Page,
    theme,
};
use eframe::egui::{self, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
fn report(name: &str, mut samples: Vec<f64>, layouts: u64, writes: u64) {
    samples.sort_by(f64::total_cmp);
    let pct = |p: f64| samples[((samples.len() - 1) as f64 * p).round() as usize];
    println!(
        "{{\"scene\":\"{name}\",\"samples\":{},\"mean_ms\":{:.3},\"p50_ms\":{:.3},\"p95_ms\":{:.3},\"max_ms\":{:.3},\"chapter_layouts\":{layouts},\"study_mutations\":{writes}}}",
        samples.len(),
        samples.iter().sum::<f64>() / samples.len() as f64,
        pct(0.5),
        pct(0.95),
        pct(1.0)
    );
}
fn sample(
    h: &mut Harness<'_, BibleApp>,
    count: usize,
    mut step: impl FnMut(&mut Harness<'_, BibleApp>, usize),
) -> Vec<f64> {
    (0..count)
        .map(|i| {
            step(h, i);
            let at = Instant::now();
            h.step();
            at.elapsed().as_secs_f64() * 1000.0
        })
        .collect()
}
fn main() {
    let dir = std::env::temp_dir().join(format!("bible-phone-perf-{}", std::process::id()));
    let mut init = initialize(dir.clone(), Arc::new(|| {})).unwrap();
    init.settings.onboarded = true;
    init.settings.sidebar_open = false;
    init.settings.theme_system = false;
    let psalm = ChapterIdx::new(BookId(18), 119).unwrap();
    init.settings.chapter = psalm.0;
    init.settings.verse = 1;
    let v = VerseRef::new(BookId(42), 3, 16);
    init.study.set_note(v, "Performance test note").unwrap();
    init.study.toggle_bookmark(v).unwrap();
    let startup =
        Startup { t0: Instant::now(), probe: false, init: Duration::ZERO, first_ui: None, frames: 0 };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).build_eframe(|cc| {
        cc.egui_ctx.set_fonts(theme::font_definitions());
        BibleApp::new(cc, init, online_counter(), startup)
    });
    theme::apply(&h.ctx, theme::Theme::Light);
    h.run_steps(25);
    let before = h.state().reader.layouts;
    let s = sample(&mut h, 120, |_, _| {});
    report("reader_idle", s, h.state().reader.layouts - before, 0);
    let before = h.state().reader.layouts;
    let s = sample(&mut h, 240, |h, _| h.state_mut().reader.anchor.offset += 37.0);
    report("reader_scroll", s, h.state().reader.layouts - before, 0);
    h.state_mut().reader.jump(psalm, Some(40), false);
    h.run_steps(12);
    let before = h.state().reader.layouts;
    let s = sample(&mut h, 240, |h, i| {
        if i % 30 == 0 {
            let app = h.state_mut();
            app.settings.sidebar_open = !app.settings.sidebar_open;
        }
    });
    report("sidebar_toggle", s, h.state().reader.layouts - before, 0);
    h.state_mut().settings.sidebar_open = false;
    h.state_mut().pages.open(Page::Bookmarks);
    h.run_steps(8);
    let s = sample(&mut h, 120, |_, _| {});
    report("bookmarks", s, 0, 0);
    h.state_mut().pages.open(Page::Notes);
    h.run_steps(5);
    let preview =
        h.state().library.load(&h.state().settings.translation).unwrap().find(v).unwrap().plain_text();
    h.get_by_label(&format!("John 3:16\n{preview}\nPerformance test note")).click();
    h.run_steps(4);
    h.get_by_role(egui::accesskit::Role::MultilineTextInput).click();
    h.run_steps(3);
    let before = h.state().study.generation();
    let s = sample(&mut h, 80, |h, i| {
        if i % 2 == 0 {
            h.input_mut().events.push(egui::Event::Text("x".into()));
        }
    });
    report("note_typing", s, 0, h.state().study.generation() - before);
    h.get_by_label("Back to notes").click();
    h.run_steps(4);
    assert!(h.state().study.note(v).unwrap().body.contains('x'));
    drop(h);
    let _ = std::fs::remove_dir_all(dir);
}
