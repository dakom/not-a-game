use crate::{
    config::CONFIG, enemy::data::EnemySpriteSheets, media::Media, prelude::*, renderer::Renderer,
    spritesheet::SpriteSheet,
};

pub type EnemySpawnerViewMut<'a> = UniqueViewMut<'a, EnemySpawner>;
pub type EnemySpawnerView<'a> = UniqueView<'a, EnemySpawner>;

#[derive(Component, Unique)]
pub struct EnemySpawner {
    pub sprite_sheets: EnemySpriteSheets,
    pub launcher_sprite_sheet: SpriteSheet,
}

impl EnemySpawner {
    pub fn new(renderer: &mut Renderer, media: &Media) -> Result<Self> {
        Ok(Self {
            sprite_sheets: EnemySpriteSheets::new(renderer, media)?,
            launcher_sprite_sheet: SpriteSheet::new(
                renderer,
                &media.objects.launcher_img,
                &media.objects.launcher_info,
            )?,
        })
    }
}
