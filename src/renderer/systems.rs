use awsm_web::webgl::{BufferMask, FrameBufferTarget, GlToggle};
use nalgebra::geometry;

use crate::{
    animation::data::Animation,
    background::data::BackgroundView,
    collision::{
        data::{Collider, CollisionEventQueue, CollisionEventQueueViewMut},
        debug::CollisionDebugger,
    },
    config::CONFIG,
    enemy::{
        data::Enemy, effects::data::EnemyEffect, launcher::data::EnemyLauncher,
        physics::data::EnemyDirection,
    },
    explosion::data::{Explosion, ExplosionSpawner, ExplosionSpawnerView},
    media::MediaView,
    prelude::*,
    projectiles::data::Projectile,
};

use super::RendererViewMut;

pub fn render_sys(
    mut renderer: RendererViewMut,
    world_transforms: View<WorldTransform>,
    // nesting tuples since we hit the 10 views limit
    game_objects: (
        View<Enemy>,
        View<EnemyLauncher>,
        View<Projectile>,
        View<Explosion>,
        ViewMut<EnemyEffect>,
    ),
    collision: (
        View<Collider>,
        CollisionEventQueueViewMut,
        UniqueViewMut<CollisionDebugger>,
    ),
    animations: View<Animation>,
    background: BackgroundView,
    explosion_spawner: ExplosionSpawnerView,
) {
    let (enemies, enemy_launchers, projectiles, explosions, mut enemy_effects) = game_objects;
    let (colliders, mut collision_events, collision_debugger) = collision;

    if !(renderer.pre_draw().unwrap_ext()) {
        return;
    }
    background.render(&mut renderer).unwrap_ext();

    for (enemy, world_transform, animation, effect) in
        (&enemies, &world_transforms, &animations, &mut enemy_effects).iter()
    {
        enemy
            .render(&mut renderer, world_transform, animation, effect)
            .unwrap_ext();
    }

    for (launcher, world_transform, animation) in
        (&enemy_launchers, &world_transforms, &animations).iter()
    {
        launcher
            .render(&mut renderer, world_transform, animation)
            .unwrap_ext();
    }

    for (projectile, world_transform) in (&projectiles, &world_transforms).iter() {
        projectile
            .render(&mut renderer, world_transform)
            .unwrap_ext();
    }

    for (explosion, world_transform, animation) in
        (&explosions, &world_transforms, &animations).iter()
    {
        explosion
            .render(
                &mut renderer,
                world_transform,
                &explosion_spawner,
                animation,
            )
            .unwrap_ext();
    }

    if collision_debugger.draw {
        for (entity, collider) in colliders.iter().with_id() {
            let event = collision_events
                .iter()
                .find(|e| e.a.entity == entity || e.b.entity == entity);
            let geometry_colliding = event.is_some();
            collider
                .render_debug(&mut renderer, &collision_debugger, geometry_colliding)
                .unwrap_ext();
        }
    }

    renderer.post_draw().unwrap_ext();
}
