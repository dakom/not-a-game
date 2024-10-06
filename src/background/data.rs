use awsm_web::webgl::{
    AttributeOptions, DataType, NameOrLoc, PixelFormat, SimpleTextureOptions, TextureTarget,
    TextureWrapMode, VertexArray, WebGl2Renderer, WebGlTextureSource,
};

use crate::{
    media::Media,
    prelude::*,
    renderer::{shaders::ShaderProgram, Renderer},
};

pub type BackgroundViewMut<'a> = UniqueViewMut<'a, Background>;
pub type BackgroundView<'a> = UniqueView<'a, Background>;

#[derive(Component, Unique)]
pub struct Background {
    pub shader: ShaderProgram,
    pub texture_ids: Vec<Vec<Id>>,
    pub width: f32,
    pub height: f32,
    pub cloud_offset: f64,
}

impl Background {
    pub fn new(renderer: &mut Renderer, media: &Media) -> Result<Self> {
        let mut texture_ids = Vec::new();
        for bgs in &media.bg {
            let mut ids = Vec::new();
            for bg in bgs {
                let texture_id = renderer.create_texture()?;
                renderer.assign_simple_texture(
                    texture_id,
                    TextureTarget::Texture2d,
                    &SimpleTextureOptions {
                        pixel_format: PixelFormat::Rgba,
                        wrap_s: Some(TextureWrapMode::Repeat),
                        wrap_t: Some(TextureWrapMode::Repeat),
                        ..SimpleTextureOptions::default()
                    },
                    &WebGlTextureSource::ImageElement(bg),
                )?;
                ids.push(texture_id);
            }
            texture_ids.push(ids);
        }

        Ok(Self {
            shader: renderer.shaders.programs.sprite.clone(),
            texture_ids,
            width: 0.0,
            height: 0.0,
            cloud_offset: 0.0,
        })
    }
}
