use nalgebra::Point3;

use super::data::{Explosion, ExplosionSpawnerViewMut};
use crate::{
    animation::data::Animation,
    audio::{AudioEvent, AudioEventQueue},
    camera::CameraView,
    collision::data::Collider,
    layout::data::{LayoutAnchor, LayoutPosition},
    prelude::*,
    tick::UpdateTickView,
};

pub fn explosion_spawn_sys(
    mut spawner: ExplosionSpawnerViewMut,
    group: (
        ViewMut<Explosion>,
        ViewMut<LayoutPosition>,
        ViewMut<LayoutAnchor>,
        ViewMut<Animation>,
    ),
    mut sg_storages: SceneGraphStoragesMut,
    mut rand: RandViewMut,
    mut audio_events: UniqueViewMut<AudioEventQueue>,
    camera: CameraView,
    colliders: View<Collider>,
    tick: UpdateTickView,
) {
    let (mut explosions, mut positions, mut anchors, mut animations) = group;

    let (width, height) = (
        spawner.spritesheet.max_cell_width,
        spawner.spritesheet.max_cell_height,
    );

    let animation = Animation::new(&spawner.spritesheet);

    let spawner = &mut *spawner;

    let mut should_play_sound = false;
    for to_spawn in spawner.to_spawn.drain() {
        // comment this out to trace the explosion along the path, helpful for debugging
        if spawner.spawned.contains(&to_spawn) {
            continue;
        }

        spawner.spawned.insert(to_spawn);

        let collider = colliders.get(to_spawn).unwrap();
        let transform = sg_storages.local_transforms.get(to_spawn).unwrap();

        let bottom_left_point = transform.transform_point(&Point3::new(0.0, 0.0, 0.0));
        let top_right_point =
            transform.transform_point(&Point3::new(collider.width, collider.height, 0.0));
        let mut origin = bottom_left_point.coords;

        // it's already at the bottom_left, now we need to move it to the center of the box
        origin.x += (top_right_point.x - bottom_left_point.x) / 2.0;
        origin.y += (top_right_point.y - bottom_left_point.y) / 2.0;

        // will place at the corner of the bounding box
        // update our center point of the explosion to be origin middle (this doesn't put it in the middle of the box)
        origin.x -= width as f32 / 2.0;
        origin.y -= height as f32 / 2.0;

        let entity = sg_storages.spawn_child_trs(None, Some(origin), None, None);

        let explosion = Explosion { explodee: to_spawn };

        (&mut explosions, &mut animations)
            .add_component_unchecked(entity, (explosion, animation.clone()));

        should_play_sound = true;
    }

    if should_play_sound {
        audio_events.push(AudioEvent::CollisionImpact);
    }
}
