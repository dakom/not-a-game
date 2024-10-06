
use awsm_web::webgl::{BeginMode, GlToggle, BlendFactor, BufferData, BufferTarget, BufferUsage};

use crate::{prelude::*, renderer::{Renderer, uvs::Uvs, buffers::Buffers}};
use super::data::Background;

impl Background {
    pub fn render(&self, renderer: &mut Renderer) -> Result<()> {
        renderer.toggle(GlToggle::Blend, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        renderer.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        renderer.set_depth_mask(false);
        renderer.toggle(GlToggle::DepthTest, false);

        self.shader.activate(renderer)?;
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


        for (idx, texture_id) in self.texture_ids[0].iter().enumerate() {

            let mat = nalgebra_glm::translate(&Mat4::identity(), &Vec3::new(-self.width / 2.0, -self.height / 2.0, (idx as f32)));

            mat.write_to_vf32(&mut model_matrix_data);
            renderer.upload_uniform_mat_4_name("u_model", &model_matrix_data)?;
            if idx == 2 {
                renderer.upload_uniform_fvals_2_name("u_uv_offset", (self.cloud_offset as f32, 0.0));
            } else {
                renderer.upload_uniform_fvals_2_name("u_uv_offset", (0.0, 0.0));
            }

            renderer.upload_uniform_mat_4_name("u_model", &model_matrix_data)?;

            renderer.upload_uniform_fvals_4_name("u_tint", (1.0, 1.0, 1.0, 1.0));
            renderer.activate_texture_sampler_name(*texture_id, "u_sampler")?;
            renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        }

        Ok(())
    }
}