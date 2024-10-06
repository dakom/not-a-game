use crate::prelude::*;

pub struct GameOver {}

impl GameOver {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameOver {
    pub fn render(&self) -> Dom {
        static CONTAINER: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("align-items", "center")
            }
        });

        static CONTENT: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("opacity", "0.5")
                .style("width", "50%")
                .style("height", "50%")
                .style("background-color", Color::Darkish.hex_str())
            }
        });
        html!("div", {
            .class([&*FULL_SCREEN, &*CONTAINER])
            .child(html!("div", {
                .class(&*CONTENT)
                .children(&mut [
                    html!("h1", {
                        .class([Color::Whiteish.class(), &*TEXT_SIZE_MD])
                        .text("Game Over")
                    })
                ])
            }))
        })
    }
}
