//! Debounced phone notes save after a pause, on navigation, and on normal exit.
use bible_core::{BookId, VerseRef, study::StudyStore};
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
#[test]
fn debounced_notes_survive_navigation_and_exit() {
    let dir = std::env::temp_dir().join(format!("bible-note-lifecycle-{}", std::process::id()));
    let mut init = initialize(dir.clone(), Arc::new(|| {})).unwrap();
    init.settings.onboarded = true;
    init.settings.sidebar_open = false;
    let v = VerseRef::new(BookId(42), 3, 16);
    init.study.set_note(v, "Initial note").unwrap();
    let startup =
        Startup { t0: Instant::now(), probe: false, init: Duration::ZERO, first_ui: None, frames: 0 };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).build_eframe(|cc| {
        cc.egui_ctx.set_fonts(theme::font_definitions());
        BibleApp::new(cc, init, online_counter(), startup)
    });
    h.run_steps(8);
    h.state_mut().pages.open(Page::Notes);
    h.run_steps(5);
    let preview =
        h.state().library.load(&h.state().settings.translation).unwrap().find(v).unwrap().plain_text();
    h.get_by_label(&format!("John 3:16\n{preview}\nInitial note")).click();
    h.run_steps(4);
    h.get_by_role(egui::accesskit::Role::MultilineTextInput).click();
    h.run_steps(3);
    let generation = h.state().study.generation();
    for _ in 0..40 {
        h.input_mut().events.push(egui::Event::Text("x".into()));
        h.step();
    }
    assert_eq!(h.state().study.generation(), generation, "Typing should not write on each keystroke");
    h.run_steps(25);
    assert_eq!(h.state().study.generation(), generation + 1);
    assert!(h.state().study.note(v).unwrap().body.contains(&"x".repeat(40)));
    h.input_mut().events.push(egui::Event::Text("navigate".into()));
    h.step();
    h.key_press_modifiers(egui::Modifiers::ALT, egui::Key::ArrowLeft);
    h.step();
    assert_eq!(h.state().pages.page, Page::Reader);
    assert!(h.state().study.note(v).unwrap().body.contains("navigate"));
    h.state_mut().pages.open(Page::Notes);
    h.run_steps(4);
    h.get_by_role(egui::accesskit::Role::MultilineTextInput).click();
    h.run_steps(3);
    h.input_mut().events.push(egui::Event::Text("exit".into()));
    h.step();
    eframe::App::on_exit(h.state_mut(), None);
    let saved = StudyStore::open(&dir.join("study.sqlite")).unwrap();
    assert!(saved.note(v).unwrap().body.contains("exit"));
}
