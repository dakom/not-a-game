use crate::{dom::{atoms::{buttons::ButtonSize, help::HelpButton}, ui::game::GameUiPhase, DomView}, enemy::{controller::data::EnemyController, data::{Enemy, EnemyKind}, events::EnemySelectEvent}, prelude::*};

use super::GameUi;

static CONTAINER :LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position", "absolute")
        .style("width", "100vw")
        .style("top", "calc(100vh - 64px)")
        .style("height", "64px")
        .style("overflow", "hidden")
    }
});

static BG:LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position", "absolute")
        .style("width", "100%")
        .style("height", "100%")
        .style("opacity", "0.5")
        .style("background-color", Color::Darkish.hex_str())
    }
});

static CONTENT:LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("display", "flex")
        .style("justify-content", "center")
        .style("align-items", "center")
        .style("width", "100%")
        .style("height", "100%")
        .style("gap", "2rem")
    }
});

impl GameUi {
    pub fn render_footer(self: &Arc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .class(&*CONTAINER)
            .child(html!("div", {
                .class([&*BG, &*POINTER_EVENTS_NONE])
            }))
            .child(html!("div", {
                .class(&*CONTENT)
                .children((0..4).map(|i| {
                    let kind = match i {
                        0 => EnemyKind::One,
                        1 => EnemyKind::Two,
                        2 => EnemyKind::Three,
                        3 => EnemyKind::Four,
                        _ => unreachable!("Invalid enemy index")
                    };
                    state.clone().render_enemy_box(kind)
                }))
                .child(HelpButton::render(ButtonSize::Md, clone!(state => move || {
                    state.phase.set_neq(Some(GameUiPhase::Help));
                })))
            }))
        })
    }

    fn render_enemy_box(self: Arc<Self>, kind: EnemyKind) -> Dom {
        let state = self;

        static CONTAINER:LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("cursor", "pointer")
                .style("height", "100%")
                .style("justify-content", "center")
                .style("align-items", "center")
                .style("padding", "0 2rem")
            }
        });

        static KILLED:LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("opacity", "0.2")
                .style("text-decoration", "line-through")
            }
        });

        html!("div", {
            .class([&*CONTAINER, &*TEXT_SIZE_MD, &*TEXT_WEIGHT_BOLD, Color::Darkest.class_bg()])
            .class_signal([Color::Accent.class()], state.selected_kind.signal().map(move |selected| selected == Some(kind))) 
            .class_signal([Color::Red.class()], state.selected_kind.signal().map(move |selected| selected != Some(kind))) 
            .class_signal([&*KILLED], state.destroyed_kinds.signal_cloned().map(move |destroyed| destroyed.contains(&kind)))
            .text(&format!("Enemy {}", match kind {
                EnemyKind::One => "1",
                EnemyKind::Two => "2",
                EnemyKind::Three => "3",
                EnemyKind::Four => "4",
            }))
            .event(clone!(state => move |_: events::Click| {
                state.select_enemy(kind);
            }))
        })
    }
}