use std::ops::Deref;

use nalgebra::{convert, Quaternion, UnitQuaternion};
use serde::de::value::MapAccessDeserializer;
use shipyard_scenegraph::traits::required::Vec3Ext;

use super::data::{
    Projectile, ProjectileMovement, ProjectileSpawner, ProjectileSpawnerViewMut, ProjectileToSpawn,
};
use crate::{
    collision::data::Collider,
    delete::data::MarkForDeletion,
    enemy::{
        data::Enemy,
        launcher::data::{EnemyLauncher, LauncherSide},
        physics::data::EnemyDirection,
    },
    layout::data::{LayoutAnchor, LayoutPosition},
    prelude::*,
    renderer::RendererView,
    tick::UpdateTickView,
};

pub fn projectile_spawn_sys(
    mut spawner: ProjectileSpawnerViewMut,
    mut projectiles: ViewMut<Projectile>,
    mut positions: ViewMut<LayoutPosition>,
    mut anchors: ViewMut<LayoutAnchor>,
    mut colliders: ViewMut<Collider>,
    mut sg_storages: SceneGraphStoragesMut,
    mut rand: RandViewMut,
    enemies: View<Enemy>,
    launcher: View<EnemyLauncher>,
    tick: UpdateTickView,
) {
    let ProjectileSpawner {
        good_rocket_height,
        good_rocket_width,
        good_rocket_texture_id,
        bad_rocket_width,
        bad_rocket_height,
        bad_rocket_texture_id,
        bullet_width,
        bullet_height,
        bullet_texture_id,
        ..
    } = *spawner;

    for to_spawn in spawner.to_spawn.drain(..) {
        match to_spawn {
            ProjectileToSpawn::Bomb {} => {
                //let scale = rand::gen_range(0.3..1.0);
                let entity = sg_storages.spawn_child_trs(None, None, None, None);

                let pos = Vec3::new(rand.gen_range(0.0..1.0), 1.0, 0.0);
                // TODO - get correct, try with pos 0.0 and 1.0
                let anchor = Vec3::new(-good_rocket_width / 2.0, 0.0, 0.0);
                let velocity = Vec3::new(0.0, -0.0000001, 0.0);
                let acceleration = Vec3::new(0.0, rand.gen_range(-0.0000001..-0.00000001), 0.0);

                (
                    &mut projectiles,
                    &mut positions,
                    &mut anchors,
                    &mut colliders,
                )
                    .add_component_unchecked(
                        entity,
                        (
                            Projectile {
                                width: good_rocket_width,
                                height: good_rocket_height,
                                texture_id: good_rocket_texture_id,
                                movement: ProjectileMovement::Bomb {
                                    velocity,
                                    acceleration,
                                },
                            },
                            LayoutPosition::new(pos),
                            LayoutAnchor::new(anchor),
                            Collider::default(),
                        ),
                    );
            }
            ProjectileToSpawn::BadRocketFromGround { side } => {
                for launcher in (&launcher).iter().filter(|launcher| launcher.side == side) {
                    let rotation = match side {
                        LauncherSide::Left => -30.0f32,
                        LauncherSide::Right => 30.0f32,
                    };

                    let rotation_speed = match side {
                        LauncherSide::Left => 0.02,
                        LauncherSide::Right => -0.02,
                    };

                    // this "just works" to keep it in the right origin place even as the page size shifts
                    let anchor = match side {
                        LauncherSide::Left => Vec3::new(215.0, 235.0, 0.0),
                        LauncherSide::Right => Vec3::new(-238.0, 220.0, 0.0),
                    };

                    let pos = match side {
                        LauncherSide::Left => Vec3::new(0.0, 0.0, 0.0),
                        LauncherSide::Right => Vec3::new(1.0, 0.0, 0.0),
                    };

                    let scale = Vec3::new(0.4, 0.7, 1.0);
                    let rot =
                        UnitQuaternion::from_axis_angle(&Vec3::z_axis(), rotation.to_radians());
                    let entity = sg_storages.spawn_child_trs_origin(
                        None,
                        None,
                        Some(rot.quaternion().to_owned()),
                        Some(scale),
                        None,
                    );

                    (
                        &mut projectiles,
                        &mut positions,
                        &mut anchors,
                        &mut colliders,
                    )
                        .add_component_unchecked(
                            entity,
                            (
                                Projectile {
                                    width: bad_rocket_width,
                                    height: bad_rocket_height,
                                    texture_id: bad_rocket_texture_id,
                                    movement: ProjectileMovement::Rocket {
                                        rotation,
                                        // thrust_speed: 0.0,
                                        // rotation_speed: 0.0,
                                        thrust_speed: 0.0005,
                                        rotation_speed,
                                    },
                                },
                                LayoutPosition::new(pos),
                                LayoutAnchor::new(anchor),
                                Collider::default(),
                            ),
                        );
                }
            }
            ProjectileToSpawn::BadRocketFromEnemy {} => {
                let mut to_spawn = vec![];

                (&enemies, &positions)
                    .iter()
                    .for_each(|(enemy, position)| match enemy {
                        Enemy::Three { controller, .. } => {
                            let mut pos = position.deref().clone();
                            let mut rot = 0.0;
                            let mut anchor = Vec3::new(0.0, 0.0, 0.0);

                            match controller.direction {
                                EnemyDirection::Left => {
                                    anchor.x = -170.0;
                                    anchor.y = 250.0;
                                    rot = 70.0f32;
                                }
                                EnemyDirection::Right => {
                                    anchor.x = 160.0;
                                    anchor.y = 280.0;
                                    rot = -70.0f32;
                                }
                            }
                            to_spawn.push((pos, rot, anchor));
                        }
                        _ => {}
                    });

                for (pos, rotation, anchor) in to_spawn.drain(..) {
                    let rot =
                        UnitQuaternion::from_axis_angle(&Vec3::z_axis(), rotation.to_radians());
                    let scale = Vec3::new(0.5, 0.5, 0.5);
                    let entity = sg_storages.spawn_child_trs_origin(
                        None,
                        None,
                        Some(rot.quaternion().to_owned()),
                        Some(scale),
                        None,
                    );
                    (
                        &mut projectiles,
                        &mut positions,
                        &mut anchors,
                        &mut colliders,
                    )
                        .add_component_unchecked(
                            entity,
                            (
                                Projectile {
                                    width: bad_rocket_width,
                                    height: bad_rocket_height,
                                    texture_id: bad_rocket_texture_id,
                                    movement: ProjectileMovement::Rocket {
                                        rotation,
                                        // thrust_speed: 0.0,
                                        // rotation_speed: 0.0,
                                        thrust_speed: 0.0005,
                                        rotation_speed: 0.0,
                                    },
                                },
                                LayoutPosition::new(pos),
                                LayoutAnchor::new(anchor),
                                Collider::default(),
                            ),
                        );
                }
            }
            ProjectileToSpawn::Bullet {} => {
                let mut to_spawn = vec![];

                (&enemies, &positions)
                    .iter()
                    .for_each(|(enemy, position)| match enemy {
                        Enemy::Two { controller, .. } => {
                            let mut pos = position.deref().clone();
                            let mut rot = 0.0;
                            let mut vel = Vec3::new(0.0, 0.0, 0.0);

                            match controller.direction {
                                EnemyDirection::Left => {
                                    pos.x -= 0.05;
                                    rot = 90.0f32;
                                    vel.x = -0.001;
                                }
                                EnemyDirection::Right => {
                                    pos.x += 0.05;
                                    rot = -90.0f32;
                                    vel.x = 0.001;
                                }
                            }
                            to_spawn.push((pos, rot, vel));
                        }
                        _ => {}
                    });

                for (pos, rotation, velocity) in to_spawn.drain(..) {
                    let rot =
                        UnitQuaternion::from_axis_angle(&Vec3::z_axis(), rotation.to_radians());
                    let origin = Vec3::new(bullet_width / 2.0, bullet_height / 2.0, 0.0);
                    let entity = sg_storages.spawn_child_trs_origin(
                        None,
                        None,
                        Some(rot.quaternion().to_owned()),
                        None,
                        Some(origin),
                    );
                    let acceleration = Vec3::new(0.0, 0.0, 0.0);
                    (
                        &mut projectiles,
                        &mut positions,
                        &mut anchors,
                        &mut colliders,
                    )
                        .add_component_unchecked(
                            entity,
                            (
                                Projectile {
                                    width: bullet_width,
                                    height: bullet_height,
                                    texture_id: bullet_texture_id,
                                    movement: ProjectileMovement::Bullet {
                                        velocity,
                                        acceleration,
                                        rotation,
                                    },
                                },
                                LayoutPosition::new(pos),
                                LayoutAnchor::new(Vec3::new(
                                    if rotation > 0.0 { -60.0 } else { 60.0 },
                                    140.0,
                                    0.0,
                                )),
                                Collider::default(),
                            ),
                        );
                }
            }
        }
    }
}

pub fn projectile_physics_sys(
    mut projectiles: ViewMut<Projectile>,
    mut deletions: ViewMut<MarkForDeletion>,
    mut positions: ViewMut<LayoutPosition>,
    mut rotations: ViewMut<Rotation>,
    tick: UpdateTickView,
) {
    let mut to_delete = vec![];

    for (id, (mut projectile, mut pos, mut quat)) in
        (&mut projectiles, &mut positions, &mut rotations)
            .iter()
            .with_id()
    {
        match &mut projectile.movement {
            ProjectileMovement::Bullet {
                velocity,
                acceleration,
                rotation,
            } => {
                pos.x += velocity.x * tick.delta as f32;
                pos.y += velocity.y * tick.delta as f32;
                pos.z += velocity.z * tick.delta as f32;

                velocity.x += acceleration.x * tick.delta as f32;
                velocity.y += acceleration.y * tick.delta as f32;
                velocity.z += acceleration.z * tick.delta as f32;

                if pos.x > 1.5 || pos.x < -0.5 {
                    to_delete.push(id);
                }
            }

            ProjectileMovement::Rocket {
                rotation,
                thrust_speed,
                rotation_speed,
            } => {
                let unit_quat = UnitQuaternion::from_quaternion(**quat);
                let velocity = unit_quat.transform_vector(&Vec3::new(0.0, 1.0, 0.0));
                pos.x += *thrust_speed * velocity.x * tick.delta as f32;
                pos.y += *thrust_speed * velocity.y * tick.delta as f32;
                pos.z += *thrust_speed * velocity.z * tick.delta as f32;

                *rotation -= *rotation_speed * tick.delta as f32;
                *quat = Rotation::new(
                    UnitQuaternion::from_axis_angle(&Vec3::z_axis(), rotation.to_radians())
                        .quaternion()
                        .to_owned(),
                );

                if pos.x > 1.5 || pos.x < -0.5 {
                    to_delete.push(id);
                }
            }

            ProjectileMovement::Bomb {
                velocity,
                acceleration,
            } => {
                pos.x += velocity.x * tick.delta as f32;
                pos.y += velocity.y * tick.delta as f32;
                pos.z += velocity.z * tick.delta as f32;

                velocity.x += acceleration.x * tick.delta as f32;
                velocity.y += acceleration.y * tick.delta as f32;
                velocity.z += acceleration.z * tick.delta as f32;

                if pos.y < -0.5 {
                    to_delete.push(id);
                }
            }
        }
    }

    for id in to_delete {
        deletions.add_component_unchecked(id, MarkForDeletion {});
    }
}
