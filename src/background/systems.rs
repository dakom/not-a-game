use crate::{prelude::*, tick::BeginTickView};
use super::data::BackgroundViewMut;

pub fn background_move_sys(
    mut background: BackgroundViewMut,
    tick: BeginTickView 
) {

    background.cloud_offset += tick.delta * 0.00001;

    // TODO - move background layers
}
