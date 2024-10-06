use crate::{
    animation::data::Animation,
    enemy::physics::data::EnemyDirection,
    prelude::*,
    projectiles::data::{ProjectileSpawnerViewMut, ProjectileToSpawn},
};

use std::collections::HashSet;

use crate::{
    config::CONFIG,
    enemy::{
        controller::data::{EnemyController, HorizontalMovement},
        data::{Enemy, EnemyFourPhase, EnemyOnePhase, EnemyThreePhase, EnemyTwoPhase},
    },
    media::MediaView,
    prelude::*,
    renderer::RendererViewMut,
    tick::{BeginTickView, UpdateTickView},
};

use super::data::{EnemyLauncher, LauncherSide};

pub fn launcher_animation_sys(
    mut launchers: ViewMut<EnemyLauncher>,
    mut animations: ViewMut<Animation>,
    mut projectile_spawner: ProjectileSpawnerViewMut,
    enemies: View<Enemy>,
    tick: BeginTickView,
) {
    for (launcher, animation) in (&mut launchers, &mut animations).iter() {
        if launcher.launching {
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
                    launcher.launching = false;

                    projectile_spawner
                        .to_spawn
                        .push(ProjectileToSpawn::BadRocketFromGround {
                            side: launcher.side,
                        });
                }
            }
        }
    }
}
