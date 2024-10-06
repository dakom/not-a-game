use awsm_web::webgl::{GlToggle, BlendFactor, BufferData, BufferTarget, BufferUsage, BeginMode};

use crate::{prelude::*, renderer::{Renderer, buffers::Buffers}};

use super::data::Projectile;

impl Projectile {
    pub fn render(&self, renderer: &mut Renderer, world_transform: &Mat4) -> Result<()> {
        renderer.toggle(GlToggle::Blend, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        renderer.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        renderer.set_depth_mask(false);
        renderer.toggle(GlToggle::DepthTest, false);

        let program = renderer.shaders.programs.sprite.clone();
        program.activate(renderer)?;

        renderer.upload_uniform_fvals_2_name("u_quad_scaler", (self.width, self.height));
        renderer.upload_buffer(
            renderer.buffers.quad_uvs,
            BufferData::new(
                Buffers::QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
            ),
        )?;

        let mut model_matrix_data: [f32;16] = [0.0;16];

        world_transform.write_to_vf32(&mut model_matrix_data);
        renderer.upload_uniform_fvals_2_name("u_uv_offset", (0.0, 0.0));
        renderer.upload_uniform_mat_4_name("u_model", &model_matrix_data)?;

        renderer.upload_uniform_fvals_4_name("u_tint", (1.0, 1.0, 1.0, 1.0));
        renderer.activate_texture_sampler_name(self.texture_id, "u_sampler")?;
        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

        Ok(())
    }
}