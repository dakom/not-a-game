use crate::{dom::DomView, layout::data::LayoutPosition, prelude::*, tick::BeginTickView};

use super::{controller::data::ActiveEnemyController, data::Enemy, events::EnemySelectEvent};

pub fn enemy_select_event_sys(
    mut active_controllers: ViewMut<ActiveEnemyController>,
    mut select_events: ViewMut<EnemySelectEvent>,
    mut enemies: ViewMut<Enemy>,
    positions: View<LayoutPosition>,
    dom: DomView,
    tick: BeginTickView,
) {
    let select_enemy_with_id = (&mut select_events, &mut enemies).iter().with_id().last();

    if let Some((id, (_, enemy))) = select_enemy_with_id {
        active_controllers.clear();
        select_events.clear();

        enemy.controller_mut().clear();
        active_controllers.add_component_unchecked(id, ActiveEnemyController {});
        dom.ui
            .game_ui_unchecked()
            .selected_kind
            .set_neq(Some(enemy.kind()));
    }
}
