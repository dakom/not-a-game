use crate::{prelude::*, enemy::data::{Enemy, EnemyKind}, tick::UpdateTickView, layout::data::{LayoutPosition, LayoutAnchor}};

use super::data::EnemyDirection;

pub fn enemy_position_sys(
    mut scale: ViewMut<Scale>,
    mut enemies: ViewMut<Enemy>,
    mut anchors: ViewMut<LayoutAnchor>,
    tick: UpdateTickView
) {

    (&mut scale, &mut enemies, &mut anchors).iter().for_each(|(mut scale, mut enemy, mut anchor)| {

        if enemy.controller().direction() == EnemyDirection::Left {
            scale.x = -1.0;
            if enemy.kind() != EnemyKind::Four {
                anchor.x = enemy.spritesheet().anchor_x;
            }
        } else {
            scale.x = 1.0;
            if enemy.kind() != EnemyKind::Four {
                anchor.x = -enemy.spritesheet().anchor_x;
            }
        }
    });
}