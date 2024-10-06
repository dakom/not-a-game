use anyhow::Context;
use awsm_web::{
    canvas::get_2d_context,
    webgl::{
        BeginMode, BlendFactor, BufferData, BufferTarget, BufferUsage, GlToggle, PixelFormat,
        SimpleTextureOptions, TextureTarget, TextureWrapMode, WebGlTextureSource,
    },
};
use web_sys::HtmlCanvasElement;

use crate::{
    prelude::*,
    renderer::{buffers::Buffers, Renderer},
};

use super::data::Collider;

#[derive(Component, Unique)]
pub struct CollisionDebugger {
    pub draw: bool,
    pub box_texture_id: Id,
}

impl CollisionDebugger {
    pub fn new(renderer: &mut Renderer) -> Result<Self> {
        let box_texture_id = renderer.create_texture()?;

        let create_canvas_element = || -> Result<HtmlCanvasElement> {
            web_sys::window()
                .context("could not create window")
                .and_then(|window| window.document().context("could not create document"))
                .and_then(|document| {
                    document
                        .create_element("canvas")
                        .map_err(|_| anyhow!("could not create canvas"))
                })
                .and_then(|canvas| {
                    canvas
                        .dyn_into::<HtmlCanvasElement>()
                        .map_err(|_| anyhow!("could not convert canvas to HtmlCanvasElement"))
                })
        };

        let canvas = create_canvas_element()?;

        canvas.set_width(32);
        canvas.set_height(32);
        let ctx = get_2d_context(&canvas, None)?;

        ctx.set_line_width(1.0);
        ctx.set_stroke_style(&JsValue::from_str("#ffffff"));
        ctx.stroke_rect(0.0, 0.0, 32.0f64, 32.0f64);

        renderer.assign_simple_texture(
            box_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                wrap_s: Some(TextureWrapMode::ClampToEdge),
                wrap_t: Some(TextureWrapMode::ClampToEdge),
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::CanvasElement(&canvas),
        )?;

        // to debug the canvas style itself
        /*
        canvas.style().set_property("position", "absolute");
        canvas.style().set_property("z-index", "100");
        let body = web_sys::window().context("could not create window")
                .and_then(|window| window.document().context("could not create document"))
                .and_then(|document| document.body().context("could not create body"))?;

        body.append_child(&canvas);
        */

        #[cfg(feature = "dev")]
        let draw = true;

        #[cfg(not(feature = "dev"))]
        let draw = true;

        Ok(Self {
            draw: false,
            box_texture_id,
        })
    }
}

impl Collider {
    pub fn render_debug(
        &self,
        renderer: &mut Renderer,
        debugger: &CollisionDebugger,
        geometry_colliding: bool,
    ) -> Result<()> {
        renderer.toggle(GlToggle::Blend, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        renderer.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        renderer.set_depth_mask(false);
        renderer.toggle(GlToggle::DepthTest, false);

        let program = renderer.shaders.programs.collision.clone();
        program.activate(renderer)?;

        renderer.upload_buffer(
            renderer.buffers.collision_geom,
            BufferData::new(
                self.vertices,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
            ),
        )?;

        renderer.upload_buffer(
            renderer.buffers.collision_uvs,
            BufferData::new(
                &Buffers::QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;

        renderer.activate_texture_sampler_name(debugger.box_texture_id, "u_sampler")?;

        let color = match geometry_colliding {
            true => (1.0, 0.0, 0.0, 1.0),
            false => (1.0, 1.0, 1.0, 1.0),
        };

        renderer.upload_uniform_fvals_4_name("u_tint", color);

        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        Ok(())
    }
}
