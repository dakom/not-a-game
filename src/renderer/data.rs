use std::ops::{Deref, DerefMut};

use awsm_web::webgl::WebGl2Renderer;
use web_sys::WebGl2RenderingContext;

use crate::{prelude::*, dom::{DomState, ui::UiPhase}};

use super::{shaders::Shaders, buffers::Buffers, framebuffers::{FrameBuffers, self}};

pub type RendererView<'a> = NonSendSync<UniqueView<'a, Renderer>>;
pub type RendererViewMut<'a> = NonSendSync<UniqueViewMut<'a, Renderer>>;

#[derive(Component, Unique)]
pub struct Renderer {
    pub gl: WebGl2Renderer,
    pub shaders: Shaders,
    pub buffers: Buffers,
    pub framebuffers: Option<FrameBuffers>,
}

impl Renderer {
    pub const UBO_CAMERA: u32 = 0;

    pub fn new(dom: &DomState) -> Result<Self> {
        let mut gl = WebGl2Renderer::new(dom.create_gl_context())?;

        // these must be set right away, _before_ shaders are compiled etc.
        gl.hardcoded_ubo_locations.insert("ubo_camera".to_string(), Self::UBO_CAMERA);  

        dom.ui.phase.set(UiPhase::Buffers);
        let buffers = Buffers::new(&mut gl)?;

        dom.ui.phase.set(UiPhase::Shaders);
        let shaders = Shaders::compile(&mut gl, &buffers)?;


        Ok(Self { 
            gl, 
            shaders,
            buffers,
            framebuffers: None,
        })
    }
}

impl Deref for Renderer {
    type Target = WebGl2Renderer;

    fn deref(&self) -> &WebGl2Renderer {
        &self.gl
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut WebGl2Renderer {
        &mut self.gl
    }
}