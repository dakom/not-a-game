use crate::prelude::*;
use futures::channel::oneshot::Sender;
use web_sys::HtmlCanvasElement;

pub struct Canvas {}
impl Canvas {
    pub fn render(tx: Sender<HtmlCanvasElement>) -> Dom {
        html!("canvas" => HtmlCanvasElement, {
            .class(&*FULL_SCREEN)
            .style("touch-action", "none")
            .style("cursor", "pointer")
            .after_inserted(|elem| {
                tx.send(elem);
            })
        })
    }
}
