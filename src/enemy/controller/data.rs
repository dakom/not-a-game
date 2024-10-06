use crate::{audio::AudioEventQueue, controller::data::{Input, Key}, enemy::{attack::data::{AttackFour, AttackOne, AttackThree, AttackTwo}, launcher::data::LauncherSide, physics::data::EnemyDirection}, prelude::*};

use super::process::EnemyControllerInput;

#[derive(Component, Debug)]
pub struct ActiveEnemyController {}

// common stuff for all enemy controllers
pub trait EnemyController: EnemyControllerProcessInput {
    fn direction(&self) -> EnemyDirection;
    fn set_direction(&mut self, direction: EnemyDirection);

    fn hiding(&self) -> &Option<Hiding>;
    fn set_hiding(&mut self, hiding: Option<Hiding>);

    fn jump(&self) -> &Option<Jump>;
    fn set_jump(&mut self, jump: Option<Jump>);

    fn stop_attack(&mut self);

    fn clear(&mut self);

    fn apply_update(&mut self, update: ControllerUpdate) {
        if let Some(direction) = update.direction {
            self.set_direction(direction);
        }

        if let Some(hiding) = update.hiding{
            self.set_hiding(hiding);
        }

        if let Some(jump) = update.jump {
            self.set_jump(jump);
        }
    }
}

pub trait EnemyControllerProcessInput {
    fn process_input(&mut self, input: EnemyControllerInput, audio_events: &mut AudioEventQueue); 
}

#[derive(Default)]
pub struct ControllerUpdate {
    pub direction: Option<EnemyDirection>,
    pub hiding: Option<Option<Hiding>>,
    pub jump: Option<Option<Jump>>,
}

#[derive(Debug)]
pub struct EnemyControllerOne {
    pub horizontal_movement: Option<HorizontalMovement>,
    pub hiding: Option<Hiding>,
    pub attack: Option<AttackOne>,
    pub jump: Option<Jump>,
    pub direction: EnemyDirection,
}
impl EnemyControllerOne {
    pub fn new(direction: EnemyDirection) -> Self {
        Self {
            horizontal_movement: None,
            hiding: None,
            attack: None,
            jump: None,
            direction,
        }
    }
}

impl EnemyController for EnemyControllerOne {
    fn direction(&self) -> EnemyDirection {
        self.direction
    }

    fn set_direction(&mut self, direction: EnemyDirection) {
        self.direction = direction;
    }

    fn hiding(&self) -> &Option<Hiding> {
        &self.hiding
    }

    fn set_hiding(&mut self, hiding: Option<Hiding>) {
        self.hiding = hiding;
    }

    fn jump(&self) -> &Option<Jump> {
        &self.jump
    }

    fn set_jump(&mut self, jump: Option<Jump>) {
        self.jump = jump;
    }

    fn stop_attack(&mut self) {
        self.attack = None;
    }

    fn clear(&mut self) {
        self.horizontal_movement = None;
        self.hiding = None;
        self.attack = None;
        self.jump = None;
    }
}

#[derive(Debug)]
pub struct EnemyControllerTwo {
    pub horizontal_movement: Option<HorizontalMovement>,
    pub hiding: Option<Hiding>,
    pub attack: Option<AttackTwo>,
    pub jump: Option<Jump>,
    pub direction: EnemyDirection,
}

impl EnemyControllerTwo {
    pub fn new(direction: EnemyDirection) -> Self {
        Self {
            horizontal_movement: None,
            hiding: None,
            attack: None,
            jump: None,
            direction
        }
    }
}

impl EnemyController for EnemyControllerTwo {
    fn direction(&self) -> EnemyDirection {
        self.direction
    }

    fn set_direction(&mut self, direction: EnemyDirection) {
        self.direction = direction;
    }

    fn hiding(&self) -> &Option<Hiding> {
        &self.hiding
    }

    fn set_hiding(&mut self, hiding: Option<Hiding>) {
        self.hiding = hiding;
    }

    fn jump(&self) -> &Option<Jump> {
        &self.jump
    }

    fn set_jump(&mut self, jump: Option<Jump>) {
        self.jump = jump;
    }

    fn stop_attack(&mut self) {
        self.attack = None;
    }
    fn clear(&mut self) {
        self.horizontal_movement = None;
        self.hiding = None;
        self.attack = None;
        self.jump = None;
    }
}

#[derive(Debug)]
pub struct EnemyControllerThree {
    pub horizontal_movement: Option<HorizontalMovement>,
    pub hiding: Option<Hiding>,
    pub attack: Option<AttackThree>,
    pub jump: Option<Jump>,
    pub direction: EnemyDirection
}

impl EnemyControllerThree {
    pub fn new(direction: EnemyDirection) -> Self {
        Self {
            horizontal_movement: None,
            hiding: None,
            attack: None,
            jump: None,
            direction,
        }
    }
}

impl EnemyController for EnemyControllerThree {
    fn direction(&self) -> EnemyDirection {
        self.direction
    }

    fn set_direction(&mut self, direction: EnemyDirection) {
        self.direction = direction;
    }
    fn hiding(&self) -> &Option<Hiding> {
        &self.hiding
    }

    fn set_hiding(&mut self, hiding: Option<Hiding>) {
        self.hiding = hiding;
    }

    fn jump(&self) -> &Option<Jump> {
        &self.jump
    }

    fn set_jump(&mut self, jump: Option<Jump>) {
        self.jump = jump;
    }

    fn stop_attack(&mut self) {
        self.attack = None;
    }
    fn clear(&mut self) {
        self.horizontal_movement = None;
        self.hiding = None;
        self.attack = None;
        self.jump = None;
    }
}

#[derive(Debug)]
pub struct EnemyControllerFour {
    pub side: LauncherSide,
    pub hiding: Option<Hiding>,
    pub attack: Option<AttackFour>,
    pub jump: Option<Jump>,
    pub direction: EnemyDirection
}

impl EnemyControllerFour {
    pub fn new(direction: EnemyDirection) -> Self {
        Self {
            side: LauncherSide::Left,
            hiding: None,
            attack: None,
            jump: None,
            direction
        }
    }
}

impl EnemyController for EnemyControllerFour {
    fn direction(&self) -> EnemyDirection {
        self.direction
    }

    fn set_direction(&mut self, direction: EnemyDirection) {
        self.direction = direction;
    }
    fn hiding(&self) -> &Option<Hiding> {
        &self.hiding
    }

    fn set_hiding(&mut self, hiding: Option<Hiding>) {
        self.hiding = hiding;
    }

    fn jump(&self) -> &Option<Jump> {
        &self.jump
    }

    fn set_jump(&mut self, jump: Option<Jump>) {
        self.jump = jump;
    }

    fn stop_attack(&mut self) {
        self.attack = None;
    }
    fn clear(&mut self) {
        self.hiding = None;
        self.attack = None;
        self.jump = None;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HorizontalMovement {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hiding {
    Down {
        start_y: f32
    },
    Hidden {
        start_y: f32
    },
    Up {
        start_y: f32
    },
}

#[derive(Debug, Clone)]
pub struct Jump {
    pub velocity: f32,
    pub acceleration: f32,
    pub start_y: f32,
    pub has_double_jumped: bool,
}

impl Jump {
    pub fn new(start_y: f32) -> Self {
        Self {
            velocity: 0.035,
            acceleration: -0.002,
            start_y,
            has_double_jumped: false,
        }
    }

    pub fn double_jump(&self) -> Self {
        let mut jump = Self::new(self.start_y);
        jump.has_double_jumped = true;
        jump
    }
}