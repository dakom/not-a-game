use crate::{audio::{AudioEvent, AudioEventQueue}, delete::data::MarkForDeletion, dispatch_select_event, dom::DomView, prelude::*, tick::PauseTick};

use super::{controller::data::ActiveEnemyController, data::Enemy, events::{EnemyDestroyEvent, EnemySelectEvent}};

pub fn enemy_destroy_event_sys(
    mut active_controllers: ViewMut<ActiveEnemyController>,
    mut destroy_events: ViewMut<EnemyDestroyEvent>,
    mut select_events: ViewMut<EnemySelectEvent>,
    mut enemies: ViewMut<Enemy>,
    mut deletions: ViewMut<MarkForDeletion>,
    mut pause_tick: UniqueViewMut<PauseTick>,
    mut audio_events: UniqueViewMut<AudioEventQueue>,
    dom: DomView,
) {
    let mut did_destroy = false;
    for (id, (_, enemy)) in (&mut destroy_events, &mut enemies).iter().with_id() {
        deletions.add_component_unchecked(id, MarkForDeletion{});
        dom.ui.game_ui_unchecked().destroyed_kinds.lock_mut().insert(enemy.kind());
        did_destroy = true;
    }

    destroy_events.clear();

    if did_destroy {
        audio_events.push(AudioEvent::CollisionDie);
        let destroyed_active = (&enemies, &deletions, &active_controllers).iter().next().is_some();
        if destroyed_active {
            let new_selected_kind = (&enemies, !&deletions).iter().next().map(|(enemy, _)| enemy.kind());
            if let Some(kind) = new_selected_kind {
                dispatch_select_event!(enemies, &mut select_events, kind);
            } 
        }
    }

}