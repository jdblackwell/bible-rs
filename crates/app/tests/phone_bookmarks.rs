//! Phone bookmark controls wrap without overlapping and keep navigation/removal working.
use bible_core::{BookId, VerseRef, library::Library, study::StudyStore};
use bible_rs::{
    actions::Action,
    app::Settings,
    pages::{Page, PageEnv, Pages},
    theme,
};
use eframe::egui::vec2;
use egui_kittest::{Harness, kittest::Queryable};
use std::sync::Arc;
struct State {
    first: bool,
    pages: Pages,
    library: Library,
    study: StudyStore,
    settings: Settings,
    jumped: Option<VerseRef>,
}
#[test]
fn bookmarks_fit_phone_and_support_open_and_remove() {
    let dir = std::env::temp_dir().join(format!("phone-bookmarks-{}", std::process::id()));
    let mut study = StudyStore::open_in_memory().unwrap();
    let verse = VerseRef::new(BookId(42), 3, 16);
    study.toggle_bookmark(verse).unwrap();
    let mut pages = Pages::default();
    pages.open(Page::Bookmarks);
    let state = State {
        first: true,
        pages,
        library: Library::open(&dir, Arc::new(|| {})),
        study,
        settings: Settings::default(),
        jumped: None,
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
                toolbar: 48.0,
            };
            let mut actions = Vec::new();
            s.pages.show(ui, ui.max_rect(), &env, &mut actions);
            for action in actions {
                match action {
                    Action::Jump { chapter, verse: Some(v), .. } => {
                        s.jumped = Some(VerseRef::new(chapter.book(), chapter.chapter(), v))
                    }
                    Action::RemoveBookmark(v) => {
                        s.study.toggle_bookmark(v).unwrap();
                    }
                    _ => {}
                }
            }
        },
        state,
    );
    theme::apply(&h.ctx, theme::Theme::Light);
    h.run_steps(5);
    for size in [vec2(360.0, 700.0), vec2(320.0, 420.0)] {
        h.set_size(size);
        h.run_steps(4);
        let mut rects = Vec::new();
        for label in ["Canonical", "Recent", "Export", "Open John 3:16", "Remove John 3:16"] {
            h.get_by_label(label).scroll_to_me();
            h.run_steps(4);
            let rect = h.get_by_label(label).rect();
            assert!(rect.left() >= 0.0 && rect.right() <= size.x, "{label}: {rect:?}");
            assert!(rect.top() >= 0.0 && rect.bottom() <= size.y, "{label}: {rect:?}");
            rects.push(rect);
        }
        assert!(!rects[0].intersects(rects[1]));
        assert!(!rects[1].intersects(rects[2]));
        assert!(!rects[3].intersects(rects[4]));
    }
    h.get_by_label("Open John 3:16").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Open John 3:16").click();
    h.run_steps(4);
    assert_eq!(h.state().jumped, Some(verse));
    h.set_size(vec2(360.0, 700.0));
    h.run_steps(4);
    if let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW") {
        h.render().unwrap().save(format!("{dir}/phone-bookmarks.png")).unwrap();
    }
    h.get_by_label("Remove John 3:16").click();
    h.run_steps(4);
    assert!(!h.state().study.is_bookmarked(verse));
}
