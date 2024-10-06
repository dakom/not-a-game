use awsm_web::webgl::{BeginMode, GlToggle, BlendFactor, BufferData, BufferTarget, BufferUsage};

use crate::{prelude::*, renderer::{Renderer, uvs::Uvs}, enemy::data::{EnemyKind, EnemyOnePhase, EnemyTwoPhase}, animation::data::Animation};
use super::data::EnemyLauncher;

impl EnemyLauncher {
    pub fn render(&self, renderer: &mut Renderer, world_transform: &Mat4, animation: &Animation) -> Result<()> {
        renderer.toggle(GlToggle::Blend, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        renderer.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        renderer.set_depth_mask(false);
        renderer.toggle(GlToggle::DepthTest, false);

        let program = renderer.shaders.programs.sprite.clone();
        program.activate(renderer)?;

        renderer.activate_texture_sampler_name(self.spritesheet.texture_id, "u_sampler")?;

        let mut bounds = &self.spritesheet.cells[animation.index];
        let mut uvs = Uvs::new(self.spritesheet.atlas_width, self.spritesheet.atlas_height, &bounds);

        renderer.upload_buffer(
            renderer.buffers.quad_uvs,
            BufferData::new(
                uvs.data,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
            ),
        )?;

        renderer.upload_uniform_fvals_2_name("u_quad_scaler", (bounds.width as f32, bounds.height as f32));

        let mut model_matrix_data: [f32;16] = [0.0;16];

        world_transform.write_to_vf32(&mut model_matrix_data);
        renderer.upload_uniform_mat_4_name("u_model", &model_matrix_data)?;
        renderer.upload_uniform_fvals_2_name("u_uv_offset", (0.0, 0.0));

        renderer.upload_uniform_fvals_4_name("u_tint", (1.0, 1.0, 1.0, 1.0));
        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

        Ok(())
    }
}