use crate::{prelude::*, tick::UpdateTickView};

use super::data::{LayoutAnchor, LayoutPosition};

const FOOTER_HEIGHT: f32 = 64.0;

// Translate from normalized units to screen space
pub fn flush_layout_sys(
    mut sg_storages: SceneGraphStoragesMut,
    positions: View<LayoutPosition>,
    anchors: View<LayoutAnchor>,
    tick: UpdateTickView,
) {
    let viewport_width = tick.viewport_width as f32;
    let viewport_height = tick.viewport_height as f32;

    for (mut translation, position) in (&mut sg_storages.translations, &positions).iter() {
        translation.x = (position.x * viewport_width) - (viewport_width / 2.0);
        translation.y = (position.y * viewport_height) - (viewport_height / 2.0);
        translation.y += FOOTER_HEIGHT;

        //log_n!("flush_layout_sys", 10, "translation: {:?}, position: {:?}", translation, position);
    }

    for (mut translation, anchor) in (&mut sg_storages.translations, &anchors).iter() {
        translation.x += (anchor.x / viewport_width) * viewport_width;
        translation.y += (anchor.y / viewport_height) * viewport_height;
        // translation.z += anchor.z;
    }
}
