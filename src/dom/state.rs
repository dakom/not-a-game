use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement, WebGlRenderingContext, WebGl2RenderingContext};
use awsm_web::window::get_window_size;
use awsm_web::webgl::{
    get_webgl_context_2, 
    WebGlContextOptions, 
};
use futures::channel::oneshot;
use crate::prelude::*;

use super::canvas::Canvas;
use super::ui::Ui;

pub type DomViewMut<'a> = NonSendSync<UniqueViewMut<'a, DomState>>;
pub type DomView<'a> = NonSendSync<UniqueView<'a, DomState>>;

#[derive(Component, Unique)]
pub struct DomState {
    pub ui: Arc<Ui>,
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub canvas: HtmlCanvasElement,
}

impl DomState {
    pub async fn new() -> Self {
        let window = web_sys::window().expect_throw("should have a Window");
        let document = window.document().expect_throw("should have a Document");
        let body = document.body().expect_throw("should have a Body");

        // render the canvas and UI layers
        let (mut tx, mut rx) = oneshot::channel();
        dominator::append_dom(&body, Canvas::render(tx)); 

        let ui = Ui::new();
        dominator::append_dom(&body, Ui::render(ui.clone())); 

        // but don't return until the canvas is really ready
        let canvas = rx.await.unwrap_ext();

        DomState {
            ui,
            window,
            document,
            body,
            canvas
        }
    }

    pub fn window_size(&self) -> (u32, u32) {
        get_window_size(&self.window).unwrap_ext()
    }

    pub fn create_gl_context(&self) -> WebGl2RenderingContext {
        get_webgl_context_2(&self.canvas, Some(&WebGlContextOptions {
            alpha: false,
            // required for blitFrameBuffer 
            antialias: false,
            stencil: true,
            ..WebGlContextOptions::default()
        })).unwrap_ext()
    }
}