use crate::prelude::*;
use anyhow::Context;
use awsm_web::webgl::{
    BeginMode, BlendFactor, Buffer, BufferData, BufferMask, BufferTarget, BufferUsage, CmpFunction,
    DrawBuffer, FrameBufferTarget, GlQueryKind, GlToggle,
};
use nalgebra_glm::Vec2;

use crate::renderer::Renderer;

use super::data::{Collider, CollisionEvent, CollisionEventQueue, CollisionEventTarget};

impl Collider {
    pub fn intersects_rect(&self, other: &Collider) -> bool {
        // Separating Axis Theorem for 2D rectangles
        // the idea of just using 2 of the sides is inspired by https://www.gamedev.net/tutorials/_/technical/game-programming/2d-rotated-rectangle-collision-r2604/
        // though the implementation here is different, i.e.:
        // * uses the same format for each rectangle's axis (i.e. it's top_right, top_left and top_right, bottom_right in both cases)
        // * uses dot product for projection of a vertex onto an axis
        // * some cleaner variables since we can index the vertices directly

        let a = self.vertices;
        let b = other.vertices;

        let axes: [Vec2; 4] = [
            // The sides of rectangle a
            // top_right, top_left
            Vec2::new(a[4] - a[0], a[5] - a[1]),
            // top_right, bottom_right
            Vec2::new(a[4] - a[6], a[5] - a[7]),
            // The sides of rectangle b
            // top_right, top_left
            Vec2::new(b[4] - b[0], b[5] - b[1]),
            // top_right, bottom_right
            Vec2::new(b[4] - b[6], b[5] - b[7]),
        ];

        for axis in axes.iter() {
            let mut a_min = f32::INFINITY;
            let mut a_max = f32::NEG_INFINITY;
            let mut b_min = f32::INFINITY;
            let mut b_max = f32::NEG_INFINITY;

            for i in 0..4 {
                // project the vertex onto the axis
                let a_proj = axis.dot(&Vec2::new(a[i * 2], a[i * 2 + 1]));
                // keep track of the min and max of the projection
                a_min = a_min.min(a_proj);
                a_max = a_max.max(a_proj);

                // do the same for the other rectangle
                let b_proj = axis.dot(&Vec2::new(b[i * 2], b[i * 2 + 1]));
                b_min = b_min.min(b_proj);
                b_max = b_max.max(b_proj);
            }

            // if the max of `a` (i.e. right-most `a`) is to the "left" of `b` like on a number-line
            // or if the min of `a` (i.e. left-most `a`) is to the "right" of `b` like on a number-line
            // then `a` is definitely outside of `b` on this axis (and `b` is definitely outside of `a` on this axis)
            if a_max < b_min || a_min > b_max {
                return false;
            }
        }
        true
    }
}

impl CollisionEvent {
    // Overall idea is to draw the two objects to the collision framebuffer
    // and create an occlusion query to check if they overlap
    // but occlusion queries aren't guaranteed to be available immediately
    // so we need to check if they're available "later"
    // we could do that at the end of this render, but, meh, we instead defer that to next physics step
    pub fn render_pixel_intersection(&mut self, renderer: &mut Renderer) -> Result<()> {
        // TODO: move these into awsm_web
        const ZERO: u32 = 0x0;
        const KEEP: u32 = 0x1E00;
        const REPLACE: u32 = 0x1E01;
        const INCR: u32 = 0x1E02;
        const DECR: u32 = 0x1E03;
        const INVERT: u32 = 0x150A;
        const INCR_WRAP: u32 = 0x8507;
        const DECR_WRAP: u32 = 0x8508;

        let query = renderer.create_query()?;

        // bind our off-screen framebuffer that's just for collision detection
        let fbo_id = renderer
            .framebuffers
            .as_ref()
            .map(|f| f.fbo_collision.id)
            .context("no fbo found")?;
        renderer.bind_framebuffer(fbo_id, FrameBufferTarget::DrawFrameBuffer)?;
        renderer.reset_depth_stencil_draw_buffer();

        // draw a stencil around object `a`
        renderer.toggle(GlToggle::DepthTest, false);
        renderer.toggle(GlToggle::StencilTest, true);
        renderer.gl.gl.color_mask(false, false, false, false);
        renderer.gl.gl.depth_mask(false);
        renderer
            .gl
            .gl
            .stencil_func(CmpFunction::Always as u32, 1, 0xFF);
        renderer.gl.gl.stencil_op(0, REPLACE, REPLACE);
        self.a.render(renderer)?; // alpha of 0.0 will be discarded

        // use the stencil to draw object `b`, surrounded by an occlusion query
        renderer
            .gl
            .gl
            .stencil_func(CmpFunction::Equal as u32, 1, 0xFF);
        renderer.begin_query(GlQueryKind::AnySamplesPassed, &query);
        self.b.render(renderer)?;
        renderer.end_query(GlQueryKind::AnySamplesPassed);

        self.occlusion_query = Some(query);

        Ok(())
    }

    // will return Ok(None) if the query is not available yet
    pub fn check_occlusion_query(&mut self, renderer: &mut Renderer) -> Result<Option<bool>> {
        match &self.occlusion_query {
            Some(query) => {
                if renderer.query_available(query)? {
                    let value = renderer.query_result(query)?;
                    renderer.delete_query(query.clone());
                    self.occlusion_query = None;
                    Ok(Some(value > 0))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

impl CollisionEventTarget {
    pub fn render(&self, renderer: &mut Renderer) -> Result<()> {
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
                self.uvs,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
            ),
        )?;

        renderer.activate_texture_sampler_name(self.texture_id, "u_sampler")?;
        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

        Ok(())
    }
}
