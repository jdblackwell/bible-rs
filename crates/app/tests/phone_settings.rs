//! Settings stay in the phone's current window and their controls can be reached.
use bible_core::{library::Library, study::StudyStore};
use bible_rs::{
    actions::{Action, SettingsTab},
    app::{Settings, TaskResult},
    reader::layout::Mode,
    settings_ui::SettingsUi,
    theme,
};
use eframe::egui::{self, pos2, vec2};
use egui_kittest::{
    Harness,
    kittest::{By, Queryable},
};
use std::sync::{Arc, mpsc};

struct State {
    first: bool,
    open: bool,
    view: SettingsUi,
    library: Library,
    settings: Settings,
    study: StudyStore,
    tx: mpsc::Sender<TaskResult>,
}

#[test]
fn phone_settings_use_one_viewport_and_allow_changes() {
    let dir = std::env::temp_dir().join(format!("bible-phone-settings-{}", std::process::id()));
    let library = Library::open(&dir, Arc::new(|| {}));
    let state = State {
        first: true,
        open: true,
        view: SettingsUi::new(SettingsTab::Reading, &library),
        library,
        settings: Settings::default(),
        study: StudyStore::open_in_memory().unwrap(),
        tx: mpsc::channel().0,
    };
    let mut h = Harness::builder().with_size(vec2(360.0, 700.0)).wgpu().build_ui_state(
        |ui, s: &mut State| {
            if s.first {
                ui.ctx().set_fonts(theme::font_definitions());
                s.first = false;
                ui.ctx().request_repaint();
                return;
            }
            let mut actions = Vec::new();
            s.open &= s.view.show(
                ui.ctx(),
                &mut s.library,
                &mut s.settings,
                &s.study,
                &mut actions,
                &s.tx,
                &mut None,
            );
            for action in actions {
                if let Action::SetTheme(t) = action {
                    s.settings.theme = t;
                }
            }
        },
        state,
    );
    theme::apply(&h.ctx, theme::Theme::Light);
    h.run_steps(5);
    assert!(h.output().viewport_output.keys().all(|id| *id == egui::ViewportId::ROOT));
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, vec2(360.0, 700.0));
    let done = h.get_by_label("Done").rect();
    assert!(screen.contains_rect(done));
    let size_before = h.state().settings.typo.size;
    let slider = h.get_all(By::new().role(egui::accesskit::Role::Slider)).next().unwrap().rect();
    let at = pos2(slider.left() + slider.width() * 0.8, slider.center().y);
    h.hover_at(at);
    h.step();
    h.drag_at(at);
    h.step();
    h.drop_at(at);
    h.run_steps(4);
    assert_ne!(h.state().settings.typo.size, size_before);
    h.get_by_label("Verse per line").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Verse per line").click();
    h.run_steps(4);
    assert_eq!(h.state().settings.typo.mode, Mode::VersePerLine);
    h.get_by_label("Justify paragraphs").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Justify paragraphs").click();
    h.run_steps(4);
    assert!(h.state().settings.typo.justify);
    assert_eq!(h.get_by_label("Done").rect(), done);
    if let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW") {
        h.render().unwrap().save(format!("{dir}/phone-settings.png")).unwrap();
    }
    h.get_by_label("Done").click();
    h.run_steps(4);
    assert!(!h.state().open);
}
