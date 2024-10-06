pub mod game;

use crate::{
    dom::ui::game::GameUi,
    enemy::{controller::data::EnemyController, data::Enemy},
    prelude::*,
};

pub struct Ui {
    pub phase: Mutable<UiPhase>,
}

#[derive(Clone)]
pub enum UiPhase {
    Loading(Option<String>),
    Buffers,
    Shaders,
    Initializing,
    Playing(Arc<GameUi>),
}

impl Ui {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            phase: Mutable::new(UiPhase::Loading(None)),
        })
    }

    pub fn game_ui_unchecked(&self) -> Arc<GameUi> {
        match self.phase.get_cloned() {
            UiPhase::Playing(game_ui) => game_ui,
            _ => panic!("Expected UiPhase::Playing"),
        }
    }
    pub fn render(self: Arc<Self>) -> Dom {
        static SECTION: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("align-items", "center")
            }
        });
        let state = self;
        html!("div", {
            .class(&*FULL_SCREEN)
            .child_signal(state.phase.signal_cloned().map(|phase| {
                match phase {
                    UiPhase::Loading(details) => {
                        Some(html!("div", {
                            .class([&*FULL_SCREEN, &*SECTION, Color::Orange.class(), &*TEXT_SIZE_MD])
                            .text(&match details {
                                None => "Loading...".to_string(),
                                Some(details) => format!("Loading {}...", details)
                            })
                        }))
                    },
                    UiPhase::Shaders => {
                        Some(html!("div", {
                            .class([&*FULL_SCREEN, &*SECTION, Color::Orange.class(), &*TEXT_SIZE_MD])
                            .text("Compiling shaders...")
                        }))
                    },
                    UiPhase::Buffers => {
                        Some(html!("div", {
                            .class([&*FULL_SCREEN, &*SECTION, Color::Orange.class(), &*TEXT_SIZE_MD])
                            .text("Creating buffers...")
                        }))
                    },
                    UiPhase::Initializing => {
                        Some(html!("div", {
                            .class([&*FULL_SCREEN, &*SECTION, Color::Orange.class(), &*TEXT_SIZE_MD])
                            .text("Initializing...")
                        }))
                    },
                    UiPhase::Playing(game_ui) => {
                        Some(game_ui.render())
                    }
                }
            }))
        })
    }
}
