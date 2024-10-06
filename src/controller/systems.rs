use crate::{
    audio::AudioEventQueue,
    collision::debug::CollisionDebugger,
    config::CONFIG,
    dispatch_select_event,
    dom::DomViewMut,
    enemy::{
        controller::{data::ActiveEnemyController, process::EnemyControllerInput},
        data::{Enemy, EnemyKind},
        events::EnemySelectEvent,
    },
    layout::data::LayoutPosition,
    prelude::*,
    tick::PauseTick,
};

use super::{
    data::{Input, Key},
    queue::InputQueueViewMut,
};

// The input_queue itself was added to via DOM events (see Listener)
// This generally just processes the input_queue and dispatches events or sets quick state
pub fn controller_process_queue_sys(
    mut input_queue: InputQueueViewMut,
    mut enemies: ViewMut<Enemy>,
    mut enemy_select_events: ViewMut<EnemySelectEvent>,
    mut collision_debugger: UniqueViewMut<CollisionDebugger>,
    mut pause_tick: UniqueViewMut<PauseTick>,
    mut audio_events: UniqueViewMut<AudioEventQueue>,
    active_controllers: View<ActiveEnemyController>,
    positions: View<LayoutPosition>,
) {
    for input in input_queue.0.drain(..) {
        match &input {
            Input::KeyDown(key) => match key {
                Key::Pause => match *pause_tick {
                    PauseTick::Running => {
                        *pause_tick = PauseTick::ManuallyPaused;
                    }
                    PauseTick::ManuallyPaused => {
                        *pause_tick = PauseTick::Running;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        if *pause_tick != PauseTick::Running {
            continue;
        }

        // specific per-enemy controllers (move, shoot, etc.)
        // will only process input if the enemy has an active controller
        // which currently is only set when the enemy is selected
        for (id, (enemy, position, _)) in (&mut enemies, &positions, &active_controllers)
            .iter()
            .with_id()
        {
            enemy.controller_mut().process_input(
                EnemyControllerInput {
                    id,
                    input: &input,
                    position: &position,
                },
                &mut audio_events,
            );
        }

        let enemies = &enemies;
        match input {
            Input::KeyDown(key) => {
                match key {
                    // Toggle debug colliders
                    Key::ToggleDebugColliders => {
                        if CONFIG.can_debug_colliders {
                            collision_debugger.draw = !collision_debugger.draw;
                        }
                    }
                    // Select enemy via keypress
                    // this is done via an event system since it can also happen from other causes
                    // like mouse click in the UI etc.
                    Key::Number1 => {
                        dispatch_select_event!(&enemies, &mut enemy_select_events, EnemyKind::One);
                    }
                    Key::Number2 => {
                        dispatch_select_event!(&enemies, &mut enemy_select_events, EnemyKind::Two);
                    }
                    Key::Number3 => {
                        dispatch_select_event!(
                            &enemies,
                            &mut enemy_select_events,
                            EnemyKind::Three
                        );
                    }
                    Key::Number4 => {
                        dispatch_select_event!(&enemies, &mut enemy_select_events, EnemyKind::Four);
                    }
                    Key::Pause => {
                        // already handled
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
