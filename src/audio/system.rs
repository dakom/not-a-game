use awsm_web::{audio::AudioSource, loaders};
use wasm_bindgen_futures::spawn_local;

use crate::{media::MediaView, prelude::*};

use super::{AudioEvent, AudioEventQueue, AudioPlayer, AudioPlayerViewMut};

pub fn audio_event_process_sys(
    mut event_queue: UniqueViewMut<AudioEventQueue>,
    mut audio_player: AudioPlayerViewMut,
    media: MediaView
) {

    for event in event_queue.drain(..) {
        let _ = match event {
            AudioEvent::CollisionDie => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.collision_die.clone())),
            AudioEvent::CollisionImpact => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.collision_impact.clone())),
            AudioEvent::MoveDuck => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.move_duck.clone())),
            AudioEvent::MoveJump => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.move_jump.clone())),
            AudioEvent::WeaponBullet => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.weapon_bullet.clone())),
            AudioEvent::WeaponExplode => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.weapon_explode.clone())),
            AudioEvent::WeaponLauncher => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.weapon_launcher.clone())),
            AudioEvent::WeaponRpg => audio_player.mixer.play_oneshot(AudioSource::Buffer(media.audio.weapon_rpg.clone())),
        };
    }
}