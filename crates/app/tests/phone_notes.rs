//! Saved notes open in a full-width editor and edits survive returning to the list.
use bible_core::{BookId, VerseRef, library::Library, study::StudyStore};
use bible_rs::{
    actions::Action,
    app::Settings,
    pages::{PageEnv, notes},
    theme,
};
use eframe::egui::{self, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use std::sync::Arc;
struct State {
    first: bool,
    notes: notes::State,
    library: Library,
    study: StudyStore,
    settings: Settings,
    jumps: Vec<VerseRef>,
}
#[test]
fn phone_notes_open_edit_and_reopen() {
    let dir = std::env::temp_dir().join(format!("phone-notes-{}", std::process::id()));
    let mut study = StudyStore::open_in_memory().unwrap();
    let verse = VerseRef::new(BookId(42), 3, 16);
    study.set_note(verse, "Original note").unwrap();
    let state = State {
        first: true,
        notes: notes::State::default(),
        library: Library::open(&dir, Arc::new(|| {})),
        study,
        settings: Settings::default(),
        jumps: Vec::new(),
    };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).wgpu().build_ui_state(
        |ui, s: &mut State| {
            if s.first {
                ui.ctx().set_fonts(theme::font_definitions());
                s.first = false;
                ui.ctx().request_repaint();
                return;
            }
            let env = PageEnv {
                library: &s.library,
                study: &s.study,
                settings: &s.settings,
                query: "",
                toolbar: 0.0,
            };
            let mut actions = Vec::new();
            s.notes.show(ui, ui.max_rect(), &env, &mut actions);
            for action in actions {
                match action {
                    Action::SaveNote(v, body) => s.study.set_note(v, &body).unwrap(),
                    Action::RemoveNote(v) => s.study.set_note(v, "").unwrap(),
                    Action::Jump { chapter, verse: Some(v), .. } => {
                        s.jumps.push(VerseRef::new(chapter.book(), chapter.chapter(), v))
                    }
                    _ => {}
                }
            }
        },
        state,
    );
    theme::apply(&h.ctx, theme::Theme::Light);
    h.run_steps(5);
    let preview =
        h.state().library.load(&h.state().settings.translation).unwrap().find(verse).unwrap().plain_text();
    assert!(!preview.is_empty());
    h.get_by_label(&format!("John 3:16\n{preview}\nOriginal note")).click();
    h.run_steps(4);
    h.get_by_label(&preview);
    let editor = h.get_by_role(egui::accesskit::Role::MultilineTextInput);
    assert!(editor.rect().right() <= 360.0);
    editor.click();
    editor.type_text(" edited John 1:1");
    h.run_steps(2);
    h.run_steps(25);
    assert!(h.state().study.note(verse).unwrap().body.contains("edited"));
    h.get_by_label("Back to notes").click();
    h.run_steps(4);
    let body = h.state().study.note(verse).unwrap().body.clone();
    h.get_by_label(&format!("John 3:16\n{preview}\n{body}")).click();
    h.run_steps(4);
    assert!(h.get_by_label("Back to notes").rect().right() <= 360.0);
    if let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW") {
        h.render().unwrap().save(format!("{dir}/phone-notes.png")).unwrap();
    }
    h.get_by_label("Open John 3:16 in reader").click();
    h.run_steps(4);
    assert_eq!(h.state().jumps.last(), Some(&verse));
    h.get_by_label("Open John 1:1 in reader").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Open John 1:1 in reader").click();
    h.run_steps(4);
    assert_eq!(h.state().jumps.last(), Some(&VerseRef::new(BookId(42), 1, 1)));
    h.get_by_label("Delete note").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Delete note").click();
    h.run_steps(4);
    assert!(h.state().study.note(verse).is_none());
    assert!(h.query_by_label("Back to notes").is_none());
}
