use crate::prelude::*;

use super::data::{Enemy, EnemyKind};

#[derive(Component)]
pub struct EnemySelectEvent { }

#[macro_export]
macro_rules! dispatch_select_event {
    // this is a macro so it can be called with either &View<Enemy> or &mut ViewMut<Enemy> etc.
    // example: dispatch_select_event!(&enemies, &mut enemy_select_events, EnemyKind::One);
    // The select event is independent of just assigning the controller
    // so that it can also "do more" - for now that means also setting the dom ui
    ($enemies:expr, $enemy_select_events:expr, $kind:expr) => {
        {
            let enemies = $enemies;
            let events = $enemy_select_events;
            let kind = $kind;

            let id = enemies.iter().with_id().find_map(|(id, enemy)| {
                if enemy.kind() == kind {
                    Some(id)
                } else {
                    None
                }
            });

            if let Some(id) = id {
                events.add_component_unchecked(id, EnemySelectEvent{});
            }

        }
    };
}

#[derive(Component)]
pub struct EnemyDestroyEvent { }