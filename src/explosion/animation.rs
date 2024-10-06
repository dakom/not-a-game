use crate::{
    animation::data::Animation,
    delete::data::MarkForDeletion,
    enemy::{events::EnemyDestroyEvent, physics::data::EnemyDirection},
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

use super::data::Explosion;

pub fn explosion_animation_sys(
    mut animations: ViewMut<Animation>,
    mut deletions: ViewMut<MarkForDeletion>,
    mut enemy_destroy_events: ViewMut<EnemyDestroyEvent>,
    enemies: View<Enemy>,
    explosions: View<Explosion>,
    tick: BeginTickView,
) {
    for (entity, (explosion, animation)) in (&explosions, &mut animations).iter().with_id() {
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
            if animation.index == animation.len / 2 {
                if enemies.contains(explosion.explodee) {
                    enemy_destroy_events
                        .add_component_unchecked(explosion.explodee, EnemyDestroyEvent {});
                } else {
                    deletions.add_component_unchecked(explosion.explodee, MarkForDeletion {});
                }
            }
            if animation.index >= animation.len {
                animation.index = 0;
                deletions.add_component_unchecked(entity, MarkForDeletion {});
            }
        }
    }
}
