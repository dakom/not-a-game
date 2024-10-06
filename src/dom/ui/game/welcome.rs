use gloo_timers::future::TimeoutFuture;

use crate::{config::CONFIG, prelude::*, tick::{PauseTick, PauseTickViewMut}};

use crate::dom::atoms::buttons::*;

use super::GameUiPhase;

pub struct Welcome {
    world: Arc<World>,
    game_phase: Mutable<Option<GameUiPhase>>,
}

impl Welcome {
    pub fn new(world: Arc<World>, game_phase: Mutable<Option<GameUiPhase>>) -> Arc<Self> {
        Arc::new(Self {world, game_phase})
    }
}

impl Welcome {
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

        static BUTTON_ROW: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("padding", "1rem")
                .style("display", "flex")
                .style("align-items", "center")
                .style("gap", "1rem")
                .style("text-align", "center")
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
                            html!("div", {
                                .class(&*BUTTON_ROW)
                                .child(Button::new()
                                    .with_text("Start a war because I'm an idiot!")
                                    .with_color(ButtonColor::Red)
                                    .with_on_click(clone!(state => move || {
                                        state.world.run(|mut pause_tick: PauseTickViewMut| {
                                            state.game_phase.set_neq(None);
                                            *pause_tick = PauseTick::Running;
                                        });
                                    }))
                                    .render()
                                )
                                .child(html!("div", {
                                    .class(&*TEXT_SIZE_LG)
                                    .text("or")
                                }))
                                .child(Button::new()
                                    .with_text("Be a decent person, choose peace!")
                                    .with_color(ButtonColor::Red)
                                    .with_on_click(clone!(state => move || {
                                        state.world.run(|mut pause_tick: PauseTickViewMut| {
                                            state.game_phase.set_neq(Some(GameUiPhase::Winner));
                                        });
                                    }))
                                    .render()
                                )
                            }),
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