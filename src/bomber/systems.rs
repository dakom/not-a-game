use crate::{
    config::CONFIG,
    prelude::*,
    projectiles::data::{ProjectileSpawnerViewMut, ProjectileToSpawn},
    tick::UpdateTickView,
};

use super::data::BomberViewMut;

pub fn bomber_drop_sys(
    mut bomber: BomberViewMut,
    mut projectile_spawner: ProjectileSpawnerViewMut,
    mut rand: RandViewMut,
    tick: UpdateTickView,
) {
    if let Some(drop_countdown) = &mut bomber.drop_countdown {
        *drop_countdown -= tick.delta;

        if *drop_countdown <= 0.0 {
            *drop_countdown = rand.gen_range(CONFIG.live_drop_countdown_range.clone());

            projectile_spawner.to_spawn.push(ProjectileToSpawn::Bomb {});
        }
    }
}
