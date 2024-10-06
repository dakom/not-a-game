use awsm_web::audio;

use crate::{audio::{AudioEvent, AudioEventQueue}, delete::data::MarkForDeletion, dom::DomView, enemy::{data::Enemy, events::{EnemyDestroyEvent, EnemySelectEvent}, launcher::data::LauncherSide, physics::data::EnemyDirection}, layout::data::{LayoutAnchor, LayoutPosition}, prelude::*, tick::{BeginTickView, UpdateTickView}};

use super::data::{EnemyController, HorizontalMovement, Hiding, Jump, ActiveEnemyController, ControllerUpdate};

pub fn enemy_controller_physics_sys(
    mut enemies: ViewMut<Enemy>,
    mut positions: ViewMut<LayoutPosition>,
    mut anchors: ViewMut<LayoutAnchor>,
    tick: UpdateTickView
) {

    let horizontal_speed = 0.0005 * tick.delta as f32;
    let hiding_speed = 0.0005 * tick.delta as f32;

    for (mut enemy, mut pos, mut anchor) in (&mut enemies, &mut positions, &mut anchors).iter() {
        let mut update_controller_and_pos = |controller: &mut dyn EnemyController, movement: Option<HorizontalMovement>| {
            let mut update = ControllerUpdate::default();

            match movement {
                Some(HorizontalMovement::Left) => {
                    pos.x -= horizontal_speed;
                    update.direction = Some(EnemyDirection::Left);
                },
                Some(HorizontalMovement::Right) => {
                    pos.x += horizontal_speed;
                    update.direction = Some(EnemyDirection::Right);
                },
                _ => {}
            }

            match controller.hiding() {
                Some(Hiding::Down{start_y}) => {
                    pos.y -= hiding_speed;
                    let start_y = *start_y;
                    let end_y = -0.5; 
                    if pos.y < end_y {
                        update.hiding = Some(Some(Hiding::Up {start_y} ));
                    }
                },
                Some(Hiding::Up{start_y}) => {
                    pos.y += hiding_speed;
                    let start_y = *start_y;

                    if pos.y >= start_y { 
                        pos.y = start_y;
                        update.hiding = Some(None);
                    }
                },
                _ => {}
            }

            match controller.jump().clone() {
                Some(mut jump) => {
                    pos.y += jump.velocity;
                    jump.velocity += jump.acceleration;

                    if pos.y <= jump.start_y {
                        pos.y = jump.start_y;
                        update.jump = Some(None);
                    } else {
                        update.jump = Some(Some(jump));
                    }
                },
                _ => {}
            }

            if pos.x < 0.0 {
                pos.x = 0.0;
            }
            if pos.x > 1.0 {
                pos.x = 1.0;
            }


            controller.apply_update(update);
        };

        match enemy {
            Enemy::One{controller, ..} => {
                let horizontal_movement = controller.horizontal_movement.clone();
                update_controller_and_pos(controller, horizontal_movement);
            },
            Enemy::Two{controller, ..} => {
                let horizontal_movement = controller.horizontal_movement.clone();
                update_controller_and_pos(controller, horizontal_movement);
            },
            Enemy::Three{controller, ..} => {
                let horizontal_movement = controller.horizontal_movement.clone();
                update_controller_and_pos(controller, horizontal_movement);
            },
            Enemy::Four{controller, ..} => {
                update_controller_and_pos(controller, None);
                match controller.side {
                    LauncherSide::Left => {
                        pos.x = 0.0;
                        anchor.x = 50.0;
                        pos.y = 0.005; 
                        controller.direction = EnemyDirection::Right;
                    },
                    LauncherSide::Right => {
                        pos.x = 1.0;
                        anchor.x = -50.0;
                        pos.y = 0.005; 
                        controller.direction = EnemyDirection::Left;
                    }
                }
            },
            _ => {}
        }
    }
}
