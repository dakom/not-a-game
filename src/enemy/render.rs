use awsm_web::webgl::{BeginMode, BlendFactor, BufferData, BufferTarget, BufferUsage, GlToggle};

use super::{data::Enemy, effects::data::EnemyEffect, physics::data::EnemyDirection};
use crate::{
    animation::data::Animation,
    enemy::data::{EnemyKind, EnemyOnePhase, EnemyTwoPhase},
    prelude::*,
    renderer::{uvs::Uvs, Renderer},
};

impl Enemy {
    pub fn render(
        &self,
        renderer: &mut Renderer,
        world_transform: &Mat4,
        animation: &Animation,
        effect: &mut EnemyEffect,
    ) -> Result<()> {
        renderer.toggle(GlToggle::Blend, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        renderer.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        renderer.set_depth_mask(false);
        renderer.toggle(GlToggle::DepthTest, false);

        let program = renderer.shaders.programs.sprite.clone();
        program.activate(renderer)?;

        let spritesheet = self.spritesheet();

        renderer.activate_texture_sampler_name(spritesheet.texture_id, "u_sampler")?;

        let mut bounds = &spritesheet.cells[animation.index];
        let mut uvs = Uvs::new(spritesheet.atlas_width, spritesheet.atlas_height, &bounds);

        renderer.upload_buffer(
            renderer.buffers.quad_uvs,
            BufferData::new(
                uvs.data,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
            ),
        )?;

        renderer.upload_uniform_fvals_2_name(
            "u_quad_scaler",
            (bounds.width as f32, bounds.height as f32),
        );

        let mut model_matrix_data: [f32; 16] = [0.0; 16];

        world_transform.write_to_vf32(&mut model_matrix_data);
        renderer.upload_uniform_mat_4_name("u_model", &model_matrix_data)?;
        renderer.upload_uniform_fvals_2_name("u_uv_offset", (0.0, 0.0));

        match self.controller().hiding() {
            Some(hiding) => {
                let value = effect.update_hiding();

                // value is between 0.2 and 1.0
                // make rgb values that give a feeling of different colors flashing - but without going all black
                // the rgb range should stay between 0.0 and 1.0
                // use sin() to make it feel smooth and varied:

                let r = (0.5 + 0.5 * (value * 2.0 * std::f32::consts::PI).sin())
                    .max(0.0)
                    .min(1.0);
                let g = (0.5 + 0.5 * ((value * 2.0 * std::f32::consts::PI + 2.0).sin()))
                    .max(0.0)
                    .min(1.0);
                let b = (0.5 + 0.5 * ((value * 2.0 * std::f32::consts::PI + 4.0).sin()))
                    .max(0.0)
                    .min(1.0);

                renderer.upload_uniform_fvals_4_name("u_tint", (r, g, b, 0.5));
            }
            None => {
                renderer.upload_uniform_fvals_4_name("u_tint", (1.0, 1.0, 1.0, 1.0));
            }
        }

        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

        Ok(())
    }
}
