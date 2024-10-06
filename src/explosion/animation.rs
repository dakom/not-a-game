use crate::{prelude::*, enemy::{events::EnemyDestroyEvent, physics::data::EnemyDirection}, projectiles::data::{ProjectileSpawnerViewMut, ProjectileToSpawn}, animation::data::Animation, delete::data::MarkForDeletion};

use std::collections::HashSet;

use crate::{prelude::*, tick::{BeginTickView, UpdateTickView}, renderer::RendererViewMut, media::MediaView, enemy::{data::{EnemyOnePhase, EnemyTwoPhase, EnemyThreePhase, EnemyFourPhase, Enemy}, controller::data::{EnemyController, HorizontalMovement}}, config::CONFIG};

use super::data::Explosion;

pub fn explosion_animation_sys(
    mut animations: ViewMut<Animation>,
    mut deletions: ViewMut<MarkForDeletion>,
    mut enemy_destroy_events: ViewMut<EnemyDestroyEvent>,
    enemies: View<Enemy>,
    explosions: View<Explosion>,
    tick: BeginTickView 
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
            if animation.index == animation.len /2 {
                if enemies.contains(explosion.explodee) {
                    enemy_destroy_events.add_component_unchecked(explosion.explodee, EnemyDestroyEvent{});
                } else {
                    deletions.add_component_unchecked(explosion.explodee, MarkForDeletion{});
                }
            }
            if animation.index >= animation.len {
                animation.index = 0; 
                deletions.add_component_unchecked(entity, MarkForDeletion{});
            }

        }
    }
}