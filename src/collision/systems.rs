use nalgebra::{convert, Point, Point2, Point3};
use nalgebra_glm::Vec2;

use crate::{
    animation::data::Animation,
    enemy::data::Enemy,
    explosion::data::{Explosion, ExplosionSpawner},
    layout::data::{LayoutAnchor, LayoutPosition},
    prelude::*,
    projectiles::data::{Projectile, ProjectileMovement},
    renderer::{buffers::Buffers, uvs::Uvs, RendererViewMut},
};

use super::data::{
    Collider, CollisionEvent, CollisionEventQueue, CollisionEventQueueViewMut, CollisionEventTarget,
};

// This system is responsible for updating the collider component
// to have vertices in world space... can debug with `c` key
pub fn update_collider_sys(
    mut collider: ViewMut<Collider>,
    animations: View<Animation>,
    enemies: View<Enemy>,
    projectiles: View<Projectile>,
    positions: ViewMut<LayoutPosition>,
    anchors: ViewMut<LayoutAnchor>,
    transform: View<WorldTransform>,
) {
    for (collider, enemy, animation, transform) in
        (&mut collider, &enemies, &animations, &transform).iter()
    {
        let cell = &enemy.spritesheet().cells[animation.index];
        collider.update(cell.width as f32, cell.height as f32, &transform);
    }

    for (collider, projectile, transform) in (&mut collider, &projectiles, &transform).iter() {
        collider.update(projectile.width, projectile.height, &transform);
    }
}

pub fn detect_geometric_collision_sys(
    colliders: View<Collider>,
    enemies: View<Enemy>,
    projectiles: View<Projectile>,
    transforms: View<WorldTransform>,
    mut event_queue: CollisionEventQueueViewMut,
) {
    let mut bomb_projectiles = Vec::new();
    let mut other_projectiles = Vec::new();

    for (entity_p, (collider_p, transform_p, projectile)) in
        (&colliders, &transforms, &projectiles).iter().with_id()
    {
        match projectile.movement {
            ProjectileMovement::Bomb { .. } => {
                bomb_projectiles.push((entity_p, collider_p, transform_p, projectile));
                for (entity_e, (collider_e, transform_e, enemy)) in
                    (&colliders, &transforms, &enemies).iter().with_id()
                {
                    if enemy.controller().hiding().is_some() {
                        // enemy is hiding, skipping collision check
                        continue;
                    }
                    if !event_queue.has_any_collision(entity_p, entity_e)
                        && collider_p.intersects_rect(collider_e)
                    {
                        let spritesheet = enemy.spritesheet();
                        event_queue.push(CollisionEvent {
                            a: CollisionEventTarget {
                                entity: entity_p,
                                texture_id: projectile.texture_id,
                                vertices: collider_p.vertices,
                                uvs: Buffers::QUAD_GEOM_UNIT,
                            },
                            b: CollisionEventTarget {
                                entity: entity_e,
                                texture_id: spritesheet.texture_id,
                                vertices: collider_e.vertices,
                                uvs: Uvs::new(
                                    spritesheet.atlas_width,
                                    spritesheet.atlas_height,
                                    &spritesheet.cells[0],
                                )
                                .data,
                            },
                            occlusion_query: None,
                        });
                    }
                }
            }
            ProjectileMovement::Bullet { .. } | ProjectileMovement::Rocket { .. } => {
                other_projectiles.push((entity_p, collider_p, transform_p, projectile));
            }
        }
    }

    for (entity_b, collider_b, transform_b, projectile_b) in &bomb_projectiles {
        for (entity_o, collider_o, transform_o, projectile_o) in &other_projectiles {
            if !event_queue.has_any_collision(*entity_b, *entity_o)
                && collider_b.intersects_rect(collider_o)
            {
                event_queue.push(CollisionEvent {
                    a: CollisionEventTarget {
                        entity: *entity_b,
                        texture_id: projectile_b.texture_id,
                        vertices: collider_b.vertices,
                        uvs: Buffers::QUAD_GEOM_UNIT,
                    },
                    b: CollisionEventTarget {
                        entity: *entity_o,
                        texture_id: projectile_o.texture_id,
                        vertices: collider_o.vertices,
                        uvs: Buffers::QUAD_GEOM_UNIT,
                    },
                    occlusion_query: None,
                });
            }
        }
    }
}

pub fn pixel_collision_render_sys(
    mut renderer: RendererViewMut,
    mut event_queue: CollisionEventQueueViewMut,
) {
    for event in event_queue.iter_mut() {
        if event.occlusion_query.is_none() {
            event.render_pixel_intersection(&mut renderer).unwrap_ext();
        }
    }
}
// this won't necessarily detect the collision _now_, rather it may be from some previous frame
pub fn pixel_collision_check_sys(
    mut renderer: RendererViewMut,
    mut event_queue: CollisionEventQueueViewMut,
    mut explosion_spawner: UniqueViewMut<ExplosionSpawner>,
) {
    event_queue.retain_mut(|event| {
        // if we have a None, it doesn't mean there's no collision, just that the query isn't available yet
        // so we keep it in the queue
        match event.check_occlusion_query(&mut renderer).unwrap_ext() {
            None => true,
            Some(result) => {
                if result {
                    explosion_spawner.to_spawn.insert(event.a.entity);
                    explosion_spawner.to_spawn.insert(event.b.entity);
                }

                false
            }
        }
    })
}
