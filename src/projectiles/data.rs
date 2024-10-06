use awsm_web::webgl::{TextureTarget, SimpleTextureOptions, PixelFormat, TextureWrapMode, WebGlTextureSource};

use crate::{prelude::*, renderer::Renderer, media::Media, enemy::launcher::data::LauncherSide, spritesheet::SpriteSheet};

#[derive(Component)]
pub struct Projectile {
    pub width: f32,
    pub height: f32,
    pub texture_id: Id,
    pub movement: ProjectileMovement,
}

pub enum ProjectileMovement {
    Bullet {
        velocity: Vec3,
        acceleration: Vec3,
        rotation: f32,
    },
    Rocket {
        rotation: f32, // in euler angles, across the z-axis
        thrust_speed: f32,
        rotation_speed: f32,
    },
    Bomb {
        velocity: Vec3,
        acceleration: Vec3,
    }
}

pub type ProjectileSpawnerViewMut<'a> = UniqueViewMut<'a, ProjectileSpawner>;
pub type ProjectileSpawnerView<'a> = UniqueView<'a, ProjectileSpawner>;

#[derive(Component, Unique)]
pub struct ProjectileSpawner {
    pub to_spawn: Vec<ProjectileToSpawn>,
    pub bad_rocket_width: f32,
    pub bad_rocket_height: f32,
    pub bad_rocket_texture_id: Id,
    pub good_rocket_width: f32,
    pub good_rocket_height: f32,
    pub good_rocket_texture_id: Id,
    pub bullet_width: f32,
    pub bullet_height: f32,
    pub bullet_texture_id: Id,
}

impl ProjectileSpawner{
    pub fn new(renderer: &mut Renderer, media: &Media) -> Result<Self> {

        let bad_rocket_texture_id = renderer.create_texture()?;

        renderer.assign_simple_texture(
            bad_rocket_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                wrap_s: Some(TextureWrapMode::ClampToEdge),
                wrap_t: Some(TextureWrapMode::ClampToEdge),
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&media.objects.rocket_bad_img),
        )?;

        let bullet_texture_id = renderer.create_texture()?;

        renderer.assign_simple_texture(
            bullet_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                wrap_s: Some(TextureWrapMode::ClampToEdge),
                wrap_t: Some(TextureWrapMode::ClampToEdge),
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&media.objects.bullet_img),
        )?;

        let good_rocket_texture_id = renderer.create_texture()?;

        renderer.assign_simple_texture(
            good_rocket_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                wrap_s: Some(TextureWrapMode::ClampToEdge),
                wrap_t: Some(TextureWrapMode::ClampToEdge),
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&media.objects.rocket_good_img),
        )?;

        Ok(Self {
            to_spawn: vec![],
            bad_rocket_width: media.objects.rocket_bad_img.width() as f32, 
            bad_rocket_height: media.objects.rocket_bad_img.height() as f32, 
            bad_rocket_texture_id,
            bullet_width: media.objects.bullet_img.width() as f32, 
            bullet_height: media.objects.bullet_img.height() as f32, 
            bullet_texture_id,
            good_rocket_width: media.objects.rocket_good_img.width() as f32, 
            good_rocket_height: media.objects.rocket_good_img.height() as f32, 
            good_rocket_texture_id, 
        })
    }
}

pub enum ProjectileToSpawn {
    BadRocketFromGround {
        side: LauncherSide
    },
    BadRocketFromEnemy {
    },
    Bullet { },

    Bomb { }
}