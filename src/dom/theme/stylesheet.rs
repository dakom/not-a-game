use super::{responsive::WindowSizeListener, typography::FONT_FAMILY_INTER};
use crate::prelude::*;
use dominator::stylesheet;

pub fn init() {
    stylesheet!(":root", {
        .style("box-sizing", "border-box")
        .style_signal("font-size", WindowSizeListener::size_signal().map(|size| {
            size.font_size()
        }))
    });

    stylesheet!("*, ::before, ::after", {
        .style("box-sizing", "inherit")
    });

    stylesheet!("html, body", {
        .style("margin", "0")
        .style("padding", "0")
        .style("width", "100%")
        .style("height", "100%")
        .style("font-family", FONT_FAMILY_INTER)
        .style(["-moz-user-select", "user-select"], "none")
    });

    stylesheet!("a", {
        .style("all", "unset")
        .style("cursor", "pointer")
    })
}
