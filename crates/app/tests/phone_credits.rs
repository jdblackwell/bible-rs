//! The phone About screen visibly credits upstream and identifies the community preview.
use bible_core::{library::Library, study::StudyStore};
use bible_rs::{
    actions::SettingsTab,
    adaptation,
    app::{Settings, TaskResult},
    settings_ui::SettingsUi,
    theme,
};
use eframe::egui::{self, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use std::sync::{Arc, mpsc};
struct State {
    first: bool,
    view: SettingsUi,
    library: Library,
    settings: Settings,
    study: StudyStore,
    tx: mpsc::Sender<TaskResult>,
}
#[test]
fn phone_about_identifies_original_and_adaptation() {
    let dir = std::env::temp_dir().join(format!("bible-credits-{}", std::process::id()));
    let library = Library::open(&dir, Arc::new(|| {}));
    let state = State {
        first: true,
        view: SettingsUi::new(SettingsTab::About, &library),
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
            s.view.show(
                ui.ctx(),
                &mut s.library,
                &mut s.settings,
                &s.study,
                &mut Vec::new(),
                &s.tx,
                &mut None,
            );
        },
        state,
    );
    theme::apply(&h.ctx, theme::Theme::Light);
    h.run_steps(5);
    assert!(h.output().viewport_output.keys().all(|id| *id == egui::ViewportId::ROOT));
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, vec2(360.0, 700.0));
    for label in [
        adaptation::NAME,
        "based on bible-rs",
        "Original bible-rs project",
        "Starting fork: ForrestKnight/bible-rs",
        "Omalauncher adaptation and testing: Javahax",
    ] {
        let node = h.get_by_label(label);
        assert!(screen.contains_rect(node.rect()), "Credit outside the phone: {label}");
    }
    if let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW") {
        h.render().unwrap().save(format!("{dir}/phone-credits.png")).unwrap();
    }
}
