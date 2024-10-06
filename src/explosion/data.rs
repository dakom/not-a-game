use std::collections::HashSet;

use awsm_web::webgl::{TextureTarget, SimpleTextureOptions, PixelFormat, TextureWrapMode, WebGlTextureSource};

use crate::{prelude::*, renderer::Renderer, media::Media, spritesheet::SpriteSheet};

pub type ExplosionSpawnerViewMut<'a> = UniqueViewMut<'a, ExplosionSpawner>;
pub type ExplosionSpawnerView<'a> = UniqueView<'a, ExplosionSpawner>;

#[derive(Component, Unique)]
pub struct ExplosionSpawner {
    pub to_spawn: HashSet<EntityId>,
    pub spawned: HashSet<EntityId>,
    pub spritesheet: SpriteSheet,
}

impl ExplosionSpawner {
    pub fn new(renderer: &mut Renderer, media: &Media) -> Result<Self> {
        let spritesheet = SpriteSheet::new(renderer, &media.objects.explosion_img, &media.objects.explosion_info)?;

        Ok(Self {
            to_spawn: HashSet::new(),
            spawned: HashSet::new(),
            spritesheet,
        })
    }
}

#[derive(Component)]
pub struct Explosion {
    pub explodee: EntityId
}