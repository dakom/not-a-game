use crate::{dom::util::mixins::set_on_hover, prelude::*};
use dominator::{attrs, svg};

use super::buttons::ButtonSize;

pub struct HelpButton {}

impl HelpButton {
    pub fn render(size: ButtonSize, on_click: impl Fn() + 'static) -> Dom {
        static CONTAINER_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("cursor", "pointer")
            }
        });

        static SMALL_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("width", "1rem")
                .style("height", "1rem")
            }
        });

        static LARGE_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("width", "2rem")
                .style("height", "2rem")
            }
        });

        let hover = Mutable::new(false);

        html!("div", {
            .class(&*CONTAINER_CLASS)
            .class(match size {
                ButtonSize::Sm => &*SMALL_CLASS,
                _ => &*LARGE_CLASS,
            })
            .child(svg!("svg", {
                .attrs!{
                    "xmlns": "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 16 16",
                }
                .attr_signal("fill", hover.signal().map(|hover| match hover {
                    true => Some(Color::PureWhite.hex_str()),
                    false => Some(Color::GreyAlt1.hex_str()),
                }))
                .children([
                    svg!("path", {
                        .attr("d", "M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16")
                    }),
                    svg!("path", {
                        .attr("d", "M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286m1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94")
                    }),
                ])
                .apply(set_on_hover(&hover))
                .event(move |_:events::Click| {
                    on_click();
                })
            }))
        })
    }
}