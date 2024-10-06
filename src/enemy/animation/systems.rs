use std::collections::HashSet;

use crate::{
    animation::data::Animation,
    config::CONFIG,
    enemy::{
        controller::data::{ActiveEnemyController, HorizontalMovement},
        data::{Enemy, EnemyFourPhase, EnemyOnePhase, EnemyThreePhase, EnemyTwoPhase},
        launcher::data::{EnemyLauncher, LauncherSide},
        physics::data::EnemyDirection,
    },
    media::MediaView,
    prelude::*,
    projectiles::data::{ProjectileSpawnerViewMut, ProjectileToSpawn},
    renderer::RendererViewMut,
    tick::{BeginTickView, UpdateTickView},
};

pub fn enemy_animation_sys(
    mut enemies: ViewMut<Enemy>,
    mut animations: ViewMut<Animation>,
    mut launchers: ViewMut<EnemyLauncher>,
    mut projectile_spawner: ProjectileSpawnerViewMut,
    tick: BeginTickView,
) {
    for (enemy, animation) in (&mut enemies, &mut animations).iter() {
        let mut reset_animation = false;

        match enemy {
            Enemy::One {
                phase, controller, ..
            } => {
                let old_phase = *phase;

                let mut new_phase = match controller.horizontal_movement {
                    Some(HorizontalMovement::Left) => EnemyOnePhase::Walk,
                    Some(HorizontalMovement::Right) => EnemyOnePhase::Walk,
                    _ => EnemyOnePhase::Idle,
                };

                if controller.attack.is_some() {
                    new_phase = EnemyOnePhase::Blast;
                }

                if old_phase != new_phase {
                    *phase = new_phase;
                    reset_animation = true;
                }
            }

            Enemy::Two {
                phase, controller, ..
            } => {
                let old_phase = *phase;

                let mut new_phase = match controller.horizontal_movement {
                    Some(HorizontalMovement::Left) => EnemyTwoPhase::Walk,
                    Some(HorizontalMovement::Right) => EnemyTwoPhase::Walk,
                    _ => EnemyTwoPhase::Idle,
                };

                if controller.attack.is_some() {
                    new_phase = EnemyTwoPhase::Shooting;
                };

                if old_phase != new_phase {
                    *phase = new_phase;
                    reset_animation = true;
                }
            }

            Enemy::Three {
                phase, controller, ..
            } => {
                let old_phase = *phase;

                let mut new_phase = match controller.horizontal_movement {
                    Some(HorizontalMovement::Left) => EnemyThreePhase::Walk,
                    Some(HorizontalMovement::Right) => EnemyThreePhase::Walk,
                    _ => EnemyThreePhase::Idle,
                };

                if controller.attack.is_some() {
                    new_phase = EnemyThreePhase::Shoot;
                };

                if old_phase != new_phase {
                    *phase = new_phase;
                    reset_animation = true;
                }
            }

            Enemy::Four {
                phase, controller, ..
            } => {
                let old_phase = *phase;
                let mut new_phase = EnemyFourPhase::Idle;

                if controller.attack.is_some() {
                    new_phase = EnemyFourPhase::Shoot;
                };

                if old_phase != new_phase {
                    *phase = new_phase;
                    reset_animation = true;
                }
            }
            _ => {}
        }

        if reset_animation {
            animation.reset(enemy.spritesheet());
        }
    }

    let mut animation_ended = Vec::new();

    // animate _all_ enemies
    for (id, (enemy, animation)) in (&enemies, &mut animations).iter().with_id() {
        let next = match animation.timeout {
            None => true,
            Some(timeout) => {
                let timeout = timeout - tick.delta;
                if timeout <= 0.0 {
                    true
                } else {
                    animation.timeout = Some(timeout);
                    false
                }
            }
        };

        if next {
            animation.timeout = Some(animation.cell_duration);
            animation.index = (animation.index + 1);
            if animation.index >= animation.len {
                animation.index = 0;
                animation_ended.push(id);
            } else {
                match enemy {
                    Enemy::One { .. } => {
                        // nothing, just explodes
                    }
                    Enemy::Two {
                        phase, controller, ..
                    } => match phase {
                        EnemyTwoPhase::Shooting => {
                            if animation.index == 10 {
                                projectile_spawner
                                    .to_spawn
                                    .push(ProjectileToSpawn::Bullet {});
                            }
                        }
                        _ => {}
                    },
                    Enemy::Three {
                        phase, controller, ..
                    } => match phase {
                        EnemyThreePhase::Shoot => {
                            if animation.index == 5 {
                                projectile_spawner
                                    .to_spawn
                                    .push(ProjectileToSpawn::BadRocketFromEnemy {});
                            }
                        }
                        _ => {}
                    },
                    Enemy::Four {
                        controller, phase, ..
                    } => {
                        match phase {
                            EnemyFourPhase::Shoot => {
                                // launch rocket partway through this animation phase
                                if animation.index == 20 {
                                    if let Some(launcher) =
                                        (&mut launchers).iter().find(|launcher| {
                                            // inverted because it's the direction the enemy is _facing_
                                            controller.direction == EnemyDirection::Left
                                                && launcher.side == LauncherSide::Right
                                                || controller.direction == EnemyDirection::Right
                                                    && launcher.side == LauncherSide::Left
                                        })
                                    {
                                        launcher.launching = true;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // handle animation ended for those with controllers
    for id in animation_ended {
        if let Ok(enemy) = (&mut enemies).get(id) {
            enemy.controller_mut().stop_attack();
        }
    }
}
