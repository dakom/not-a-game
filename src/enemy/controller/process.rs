// These are called via pub fn controller_queue_sys
use crate::{
    audio::{AudioEvent, AudioEventQueue},
    controller::data::{Input, Key},
    enemy::{
        attack::data::{AttackFour, AttackOne, AttackThree, AttackTwo},
        launcher::data::LauncherSide,
    },
    prelude::*,
};

use super::data::*;

pub struct EnemyControllerInput<'a> {
    pub id: EntityId,
    pub input: &'a Input,
    pub position: &'a Vec3,
}

impl EnemyControllerProcessInput for EnemyControllerOne {
    fn process_input(
        &mut self,
        EnemyControllerInput {
            id,
            input,
            position,
        }: EnemyControllerInput,
        audio_events: &mut AudioEventQueue,
    ) {
        if let Some(update) = process_horizontal(self.horizontal_movement, input) {
            self.horizontal_movement = update;
        }

        if self.jump.is_none() {
            if process_hiding(self.hiding, input, audio_events) {
                self.hiding = Some(Hiding::Down {
                    start_y: position.y,
                });
            }
        }

        if self.hiding.is_none() {
            if let Some(jump) = process_jump(&self.jump, input, position.y, audio_events) {
                self.jump = Some(jump);
            }

            if process_attack(&self.attack, input) {
                audio_events.push(AudioEvent::WeaponExplode);
                self.attack = Some(AttackOne::new());
            }
        }
    }
}

impl EnemyControllerProcessInput for EnemyControllerTwo {
    fn process_input(
        &mut self,
        EnemyControllerInput {
            id,
            input,
            position,
        }: EnemyControllerInput,
        audio_events: &mut AudioEventQueue,
    ) {
        if let Some(update) = process_horizontal(self.horizontal_movement, input) {
            self.horizontal_movement = update;
        }

        if self.jump.is_none() {
            if process_hiding(self.hiding, input, audio_events) {
                self.hiding = Some(Hiding::Down {
                    start_y: position.y,
                });
            }
        }

        if self.hiding.is_none() {
            if let Some(jump) = process_jump(&self.jump, input, position.y, audio_events) {
                self.jump = Some(jump);
            }

            if process_attack(&self.attack, input) {
                audio_events.push(AudioEvent::WeaponBullet);
                self.attack = Some(AttackTwo::new());
            }
        }
    }
}
impl EnemyControllerProcessInput for EnemyControllerThree {
    fn process_input(
        &mut self,
        EnemyControllerInput {
            id,
            input,
            position,
        }: EnemyControllerInput,
        audio_events: &mut AudioEventQueue,
    ) {
        if let Some(update) = process_horizontal(self.horizontal_movement, input) {
            self.horizontal_movement = update;
        }

        if self.jump.is_none() {
            if process_hiding(self.hiding, input, audio_events) {
                self.hiding = Some(Hiding::Down {
                    start_y: position.y,
                });
            }
        }

        if self.hiding.is_none() {
            if let Some(jump) = process_jump(&self.jump, input, position.y, audio_events) {
                self.jump = Some(jump);
            }

            if process_attack(&self.attack, input) {
                audio_events.push(AudioEvent::WeaponRpg);
                self.attack = Some(AttackThree::new());
            }
        }
    }
}
impl EnemyControllerProcessInput for EnemyControllerFour {
    fn process_input(
        &mut self,
        EnemyControllerInput {
            id,
            input,
            position,
        }: EnemyControllerInput,
        audio_events: &mut AudioEventQueue,
    ) {
        match input {
            Input::KeyDown(key) => match key {
                Key::Right => {
                    self.side = LauncherSide::Right;
                }
                Key::Left => {
                    self.side = LauncherSide::Left;
                }
                _ => {}
            },
            _ => {}
        }

        if self.jump.is_none() {
            // if process_hiding(self.hiding, input, audio_events) {
            //     self.hiding = Some(Hiding::Down{start_y: position.y});
            // }
        }

        if self.hiding.is_none() {
            if let Some(jump) = process_jump(&self.jump, input, position.y, audio_events) {
                self.jump = Some(jump);
            }

            if process_attack(&self.attack, input) {
                audio_events.push(AudioEvent::WeaponLauncher);
                self.attack = Some(AttackFour::new());
            }
        }
    }
}

// outer option is whether to apply at all
// inner option is the value to set
fn process_horizontal(
    prev: Option<HorizontalMovement>,
    input: &Input,
) -> Option<Option<HorizontalMovement>> {
    match input {
        Input::KeyDown(key) => match key {
            Key::Right => Some(Some(HorizontalMovement::Right)),
            Key::Left => Some(Some(HorizontalMovement::Left)),
            _ => None,
        },
        Input::KeyUp(key) => match key {
            Key::Right if prev == Some(HorizontalMovement::Right) => Some(None),
            Key::Left if prev == Some(HorizontalMovement::Left) => Some(None),
            _ => None,
        },
        _ => None,
    }
}

fn process_jump(
    prev: &Option<Jump>,
    input: &Input,
    start_y: f32,
    audio_events: &mut AudioEventQueue,
) -> Option<Jump> {
    let jump = match (prev, input) {
        (None, Input::KeyDown(Key::Up)) => Some(Jump::new(start_y)),
        (Some(jump), Input::KeyDown(Key::Up)) if !jump.has_double_jumped => {
            Some(jump.double_jump())
        }
        _ => None,
    };

    if jump.is_some() {
        audio_events.push(AudioEvent::MoveJump);
    }

    jump
}
fn process_hiding(prev: Option<Hiding>, input: &Input, audio_events: &mut AudioEventQueue) -> bool {
    let hiding = match (prev, input) {
        (None, Input::KeyDown(Key::Down)) => true,
        _ => false,
    };

    if hiding {
        audio_events.push(AudioEvent::MoveDuck);
    }

    hiding
}

fn process_attack<T>(prev: &Option<T>, input: &Input) -> bool {
    match (prev, input) {
        (None, Input::KeyDown(Key::Space)) => true,
        _ => false,
    }
}
