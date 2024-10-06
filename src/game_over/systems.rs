use crate::{delete::data::MarkForDeletion, dom::{ui::game::GameUiPhase, DomView}, enemy::data::Enemy, prelude::*, tick::PauseTick};

pub fn game_over_sys(
    mut pause_tick: UniqueViewMut<PauseTick>,
    mut enemies: ViewMut<Enemy>,
    mut deletions: ViewMut<MarkForDeletion>,
    dom: DomView,
) {

    if (&enemies, !&deletions).iter().next().is_none() {
        *pause_tick = PauseTick::GameOver{};
        dom.ui.game_ui_unchecked().phase.set_neq(Some(GameUiPhase::GameOver));
    }
}