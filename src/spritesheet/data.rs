use awsm_web::webgl::{TextureTarget, SimpleTextureOptions, PixelFormat, TextureWrapMode, WebGlTextureSource};
use web_sys::HtmlImageElement;

use crate::{config::CONFIG, media::SpriteSheetMediaInfo, prelude::*, renderer::Renderer};

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    pub texture_id: Id,
    pub cells: Vec<Bounds>,
    pub atlas_width: f32,
    pub atlas_height: f32,
    pub anchor_x: f32,
    pub max_cell_width: f32,
    pub max_cell_height: f32,
    pub cell_duration: f64,
}

impl SpriteSheet {
    pub fn new(renderer: &mut Renderer, img: &HtmlImageElement, info: &SpriteSheetMediaInfo) -> Result<Self> {
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
            &WebGlTextureSource::ImageElement(&img),
        )?;

        let atlas_width = img.width() as f32;
        let atlas_height = img.height() as f32;

        let cells = info.sub_textures.iter().map(|info| {
            Ok(Bounds {
                x: info.x.parse()?,
                y: info.y.parse()?,
                width: info.width.parse()?,
                height: info.height.parse()?,
            })
        }).collect::<Result<Vec<_>>>()?;

        let anchor_x = match info.anchor_x {
            Some(anchor_x) => anchor_x,
            None => {
                (cells.iter().fold(0.0, |acc, curr| acc + curr.width) as f32 / cells.len() as f32) / 2.0
            }
        };

        let mut cell_duration = match info.cell_duration {
            Some(speed) => speed,
            None => CONFIG.cell_duration
        };

        let (max_cell_width, max_cell_height) = cells.iter().fold((0.0f32, 0.0f32), |(acc_width, acc_height), curr| {
            (acc_width.max(curr.width as f32), acc_height.max(curr.height as f32))
        });

        Ok(Self {
            texture_id,
            atlas_width,
            atlas_height,
            cells,
            anchor_x,
            max_cell_width,
            max_cell_height,
            cell_duration 
        })
    }
}