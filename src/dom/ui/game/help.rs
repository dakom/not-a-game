use gloo_timers::future::TimeoutFuture;

use crate::{
    config::CONFIG,
    prelude::*,
    tick::{PauseTick, PauseTickViewMut},
};

use crate::dom::atoms::buttons::*;

use super::GameUiPhase;

pub struct Help {
    world: Arc<World>,
    game_phase: Mutable<Option<GameUiPhase>>,
}

impl Help {
    pub fn new(world: Arc<World>, game_phase: Mutable<Option<GameUiPhase>>) -> Arc<Self> {
        Arc::new(Self { world, game_phase })
    }
}

impl Help {
    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CONTAINER: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("align-items", "center")
            }
        });
        static BG: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("background-color", Color::Darkish.hex_str())
                .style("opacity", "0.5")

            }
        });

        static CONTENT: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("width", "50%")
                .style("height", "50%")
                .style("background-color", Color::Whiteish.hex_str())
                .style("display", "flex")
                .style("border-radius", "8px")
                .style("gap", "1rem")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("align-items", "center")
            }
        });
        html!("div", {
            .future(clone!(state => async move {
                state.world.run(|mut pause_tick: PauseTickViewMut| {
                    *pause_tick = PauseTick::Help;
                });
            }))
            .child(html!("div", {
                .child(html!("div", {
                    .class([&*FULL_SCREEN, &*BG])
                }))
                .child(html!("div", {
                    .class([&*FULL_SCREEN, &*CONTAINER])
                    .child(html!("div", {
                        .class(&*CONTENT)
                        .children(&mut [
                            render_controls(),
                            Button::new()
                                .with_text("Back to Game")
                                .with_color(ButtonColor::Green)
                                .with_on_click(clone!(state => move || {
                                    state.world.run(|mut pause_tick: PauseTickViewMut| {
                                        state.game_phase.set_neq(None);
                                        *pause_tick = PauseTick::Running;
                                    });
                                }))
                                .render(),
                            Button::new()
                                .with_text("What's this all about?")
                                .with_color(ButtonColor::Blue)
                                .with_on_click(clone!(state => move || {
                                    web_sys::window().unwrap_ext().location().set_href(CONFIG.html_url("about.html").as_str()).unwrap();
                                }))
                                .render()
                        ])
                    }))
                }))
            }))
        })
    }
}

fn render_controls() -> Dom {
    html!("div", {
        .class([Color::Darkish.class(), &*TEXT_SIZE_MD])
        .children(&mut [
            html!("div", {
                .class([&*TEXT_ALIGN_CENTER, &*TEXT_SIZE_H3])
                .text("Controls")
            }),
            html!("ul", {
                .children(&mut [
                    html!("li", {
                        .text("WASD or Arrow keys or HJKL to move")
                    }),
                    html!("li", {
                        .text("Space to shoot")
                    }),
                    html!("li", {
                        .text("P to pause")
                    }),
                    html!("li", {
                        .text("1,2,3,4 or click to select enemy")
                    }),
                ])
            })
        ])
    })
}
