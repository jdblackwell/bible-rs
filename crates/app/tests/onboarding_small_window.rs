//! Setup stays navigable in a phone-sized window, including after scrolling.
use std::sync::Arc;

use bible_core::library::Library;
use bible_rs::{actions::Action, app::Settings, onboarding::Onboarding, theme};
use eframe::egui::{self, pos2, vec2};
use egui_kittest::{Harness, kittest::Queryable};

struct Setup {
    onboarding: Onboarding,
    library: Library,
    settings: Settings,
    actions: Vec<Action>,
    done: bool,
}

// The desktop navigation buttons are custom painted; find their visible text.
fn continue_rect(h: &Harness<'_, Option<Setup>>) -> egui::Rect {
    h.output()
        .shapes
        .iter()
        .find_map(|shape| {
            if let egui::Shape::Text(text) = &shape.shape
                && text.galley.job.text == "Continue"
            {
                Some(text.galley.rect.translate(text.pos.to_vec2()))
            } else {
                None
            }
        })
        .expect("Continue is painted")
}

fn click_continue(h: &mut Harness<'_, Option<Setup>>) {
    let pos = continue_rect(h).center();
    h.hover_at(pos);
    h.step();
    h.drag_at(pos);
    h.step();
    h.drop_at(pos);
    h.step();
}

#[test]
fn phone_setup_scrolls_and_can_finish_both_steps() {
    for size in [vec2(360.0, 700.0), vec2(360.0, 314.0), vec2(700.0, 360.0)] {
        let dir = std::env::temp_dir().join(format!("bible-setup-test-{}", std::process::id()));
        let mut h = Harness::builder().with_size(size).wgpu().build_ui_state(
            |ui, state: &mut Option<Setup>| {
                if state.is_none() {
                    ui.ctx().set_fonts(theme::font_definitions());
                    *state = Some(Setup {
                        onboarding: Onboarding::new(ui.ctx()),
                        library: Library::open(&dir, Arc::new(|| {})),
                        settings: Settings::default(),
                        actions: Vec::new(),
                        done: false,
                    });
                    ui.ctx().request_repaint();
                    return;
                }
                let s = state.as_mut().unwrap();
                s.done |= s.onboarding.show(ui, &s.library, &mut s.settings, &mut s.actions);
            },
            None,
        );
        h.ctx.set_fonts(theme::font_definitions());
        theme::apply(&h.ctx, theme::Theme::Light);
        h.run();
        if size == vec2(360.0, 700.0)
            && let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW")
        {
            h.render().unwrap().save(format!("{dir}/welcome-phone.png")).unwrap();
        }
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
        let footer = continue_rect(&h);
        assert!(screen.contains_rect(footer), "Continue is reachable at {size:?}");
        let last = {
            let entry = h.state().as_ref().unwrap().library.get("dby").unwrap();
            format!("{} · {}", entry.meta.abbrev, entry.meta.name)
        };
        let before_swipe = h.get_by_label(&last).rect().top();

        // A swipe, represented as pointer input as on some Wayland compositors.
        h.hover_at(pos2(100.0, size.y - 110.0));
        h.step();
        h.drag_at(pos2(100.0, size.y - 110.0));
        h.step();
        h.hover_at(pos2(100.0, 70.0));
        h.step();
        h.drop_at(pos2(100.0, 70.0));
        h.run();
        assert_eq!(continue_rect(&h), footer);
        if before_swipe > footer.top() {
            assert!(h.get_by_label(&last).rect().top() < before_swipe, "swiping scrolls choices at {size:?}");
        }

        // Scroll the last option into view and select it.
        h.get_by_label(&last).scroll_to_me();
        h.run();
        assert!(screen.contains_rect(h.get_by_label(&last).rect()));
        h.get_by_label(&last).click();
        h.run();
        assert!(
            h.state()
                .as_ref()
                .unwrap()
                .actions
                .iter()
                .any(|a| matches!(a, Action::SetTranslation(id) if id == "dby"))
        );
        click_continue(&mut h);
        h.run();
        assert_eq!(h.state().as_ref().unwrap().onboarding.step, 1);
        if size == vec2(360.0, 700.0)
            && let Ok(dir) = std::env::var("BIBLE_RS_SETUP_PREVIEW")
        {
            h.render().unwrap().save(format!("{dir}/theme-phone.png")).unwrap();
        }
        assert!(screen.contains_rect(continue_rect(&h)));

        h.get_by_label("Match system appearance").scroll_to_me();
        h.run();
        assert!(screen.contains_rect(h.get_by_label("Match system appearance").rect()));
        click_continue(&mut h);
        h.run();
        assert!(h.state().as_ref().unwrap().done);
    }
}
