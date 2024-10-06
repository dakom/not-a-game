use std::ops::{Deref, DerefMut};

use awsm_web::{audio::AudioMixer, loaders};
use wasm_bindgen_futures::spawn_local;
use web_sys::{AudioBuffer, AudioContext};

use crate::{media::Media, prelude::*};

pub type AudioPlayerView<'a> = NonSendSync<UniqueView<'a, AudioPlayer>>;
pub type AudioPlayerViewMut<'a> = NonSendSync<UniqueViewMut<'a, AudioPlayer>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioEvent {
    CollisionDie,
    CollisionImpact,
    MoveDuck,
    MoveJump,
    WeaponBullet,
    WeaponExplode,
    WeaponLauncher,
    WeaponRpg,
}

#[derive(Component, Unique)]
pub struct AudioPlayer {
    pub mixer: AudioMixer,
}

#[derive(Unique, Component, Default)]
pub struct AudioEventQueue(Vec<AudioEvent>);
impl AudioEventQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for AudioEventQueue {
    type Target = Vec<AudioEvent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AudioEventQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            mixer: AudioMixer::new(None),
        }
    }
}