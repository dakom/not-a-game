use gloo_timers::future::TimeoutFuture;

use crate::{
    config::CONFIG,
    prelude::*,
    tick::{PauseTick, PauseTickViewMut},
};

use crate::dom::atoms::buttons::*;

use super::GameUiPhase;

pub struct Winner {}

impl Winner {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl Winner {
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
                .style("width", "80%")
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
                                .text("Congratulations! You've won the game!")
                            }),
                            html!("div", {
                                .text("Israel is recognized internationally as the ancestral homeland of the Jewish people")
                            }),
                            html!("div", {
                                .text("Palestine is a blossoming utopia, over in eastern Jordan, where the people are free and happy")
                            }),
                            html!("div", {
                                .text("Everyone is welcome everywhere, the world is at peace, and you are a hero!")
                            }),
                            html!("div", {
                                .text("That wasn't so hard, now was it?")
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
